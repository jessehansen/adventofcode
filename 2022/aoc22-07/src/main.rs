use std::collections::HashMap;

use anyhow::*;
use aoc_common::*;
use indextree::*;

fn main() -> Result<()> {
    run_progressive(parse, part1, part2)
}

enum FileNode {
    Directory(String),
    File(String, u32),
}

use FileNode::*;

fn parse(contents: &str) -> Result<(Arena<FileNode>, NodeId)> {
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
    Ok((arena, root))
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

fn part1((arena, root): &(Arena<FileNode>, NodeId)) -> Result<(u32, HashMap<NodeId, u32>)> {
    let mut dir_sizes = HashMap::new();
    calculate_size(&mut dir_sizes, root, arena)?;

    Ok((
        dir_sizes.values().filter(|x| **x <= 100_000).sum(),
        dir_sizes,
    ))
}

const TOTAL_SPACE: u32 = 70_000_000;
const DESIRED_FREE_SPACE: u32 = 30_000_000;

fn part2(
    (_arena, root): &(Arena<FileNode>, NodeId),
    dir_sizes: &HashMap<NodeId, u32>,
) -> Result<u32> {
    let current_free_space = TOTAL_SPACE - dir_sizes[root];
    let min_to_delete = DESIRED_FREE_SPACE - current_free_space;
    let mut sizes: Vec<u32> = dir_sizes.values().copied().collect();
    sizes.sort();

    sizes
        .into_iter()
        .find(|x| x >= &min_to_delete)
        .ok_or_else(|| anyhow!("no directory big enough to delete"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let (arena, root) = parse(SAMPLE)?;

        assert_eq!(14, arena.count());

        let total_size = calculate_size(&mut HashMap::new(), &root, &arena)?;

        assert_eq!(48381165, total_size);
        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let (result, _) = part1(&parsed)?;

        assert_eq!(95437, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let (_, sizes) = part1(&parsed)?;
        let result = part2(&parsed, &sizes)?;

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
