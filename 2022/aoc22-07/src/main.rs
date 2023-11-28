use std::collections::HashMap;
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;
use indextree::*;

fn main() -> Result<()> {
    Problem::go()
}

enum FileNode {
    Directory(String),
    File(String, u32),
}

use FileNode::*;

struct Problem {
    arena: Arena<FileNode>,
    root: NodeId,
    dir_sizes: HashMap<NodeId, u32>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        let mut arena = Arena::new();
        let mut lines = contents.lines();

        if lines.next() != Some("$ cd /") {
            bail!("malformed input, first line must go to root");
        }
        let mut current_dir = arena.new_node(Directory("".to_string()));

        let mut line = lines.next();
        while let Some(command) = line {
            if !command.starts_with("$ ") {
                bail!("malformed input, missing command prefix");
            }
            let parts: Vec<&str> = command.split(' ').collect();
            match parts[1] {
                "ls" => {
                    line = lines.next();
                    while let Some(ls_output_line) = line {
                        if ls_output_line.starts_with("$ ") {
                            break;
                        }
                        let ls_line_parts: Vec<&str> = ls_output_line.split(' ').collect();
                        match ls_line_parts[0] {
                            "dir" => {
                                current_dir.append(
                                    arena.new_node(Directory(ls_line_parts[1].to_string())),
                                    &mut arena,
                                );
                            }
                            size_str => {
                                let size: u32 = size_str.parse()?;
                                current_dir.append(
                                    arena.new_node(File(ls_line_parts[1].to_string(), size)),
                                    &mut arena,
                                );
                            }
                        }
                        line = lines.next();
                    }
                }
                "cd" => {
                    if parts[2] == ".." {
                        current_dir = current_dir
                            .ancestors(&arena)
                            .nth(1)
                            .ok_or_else(|| anyhow!("malformed input, changed dirs above root"))?;
                    } else {
                        match current_dir.children(&arena).find(|x| {
                            match arena.get(*x).map(|y| y.get()) {
                                Some(Directory(name)) => name == parts[2],
                                _ => false,
                            }
                        }) {
                            Some(target_dir) => {
                                current_dir = target_dir;
                            }
                            None => {
                                bail!(
                                    "malformed input, could not find child directory {}",
                                    parts[2]
                                );
                            }
                        }
                    }
                    line = lines.next();
                }
                _ => bail!("malformed input, unsupported command"),
            }
        }

        let root = current_dir
            .ancestors(&arena)
            .last()
            .ok_or_else(|| anyhow!("malformed input, changed dir outside of root"))?;
        Ok(Problem {
            arena,
            root,
            dir_sizes: HashMap::new(),
        })
    }
}

fn calculate_size(
    size_cache: &mut HashMap<NodeId, u32>,
    node: &NodeId,
    arena: &Arena<FileNode>,
) -> Result<u32> {
    if let Some(size) = size_cache.get(node) {
        Ok(*size)
    } else {
        match arena.get(*node).map(|x| x.get()) {
            Some(Directory(_)) => {
                let size: u32 = node
                    .children(arena)
                    .filter_map(|child| calculate_size(size_cache, &child, arena).ok())
                    .sum();

                size_cache.insert(*node, size);
                Ok(size)
            }
            Some(File(_, size)) => Ok(*size),
            None => bail!("node not in arena"),
        }
    }
}

const TOTAL_SPACE: u32 = 70_000_000;
const DESIRED_FREE_SPACE: u32 = 30_000_000;

impl Solution for Problem {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&mut self) -> Result<u32> {
        calculate_size(&mut self.dir_sizes, &self.root, &self.arena)?;

        Ok(self.dir_sizes.values().filter(|x| **x <= 100_000).sum())
    }

    fn part2(&self) -> Result<u32> {
        let current_free_space = TOTAL_SPACE - self.dir_sizes[&self.root];
        let min_to_delete = DESIRED_FREE_SPACE - current_free_space;
        let mut sizes: Vec<u32> = self.dir_sizes.values().copied().collect();
        sizes.sort();

        sizes
            .into_iter()
            .find(|x| x >= &min_to_delete)
            .ok_or_else(|| anyhow!("no directory big enough to delete"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(14, problem.arena.count());

        let total_size = calculate_size(&mut HashMap::new(), &problem.root, &problem.arena)?;

        assert_eq!(48381165, total_size);
        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(95437, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        problem.part1()?;
        let result = problem.part2()?;

        assert_eq!(24933642, result);

        Ok(())
    }

    const SAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
}
