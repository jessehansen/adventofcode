use aoc_common::run;

fn main() {
    run(parse, part1, part2);
}

fn parse(contents: &str) -> Vec<[String; 2]> {
    contents
        .lines()
        .into_iter()
        .map(|x| {
            let mut edge = x.split('-');
            [
                edge.next().unwrap().to_string(),
                edge.next().unwrap().to_string(),
            ]
        })
        .collect()
}

fn traverse<'a>(
    edges: &'a [[String; 2]],
    at: &'a str,
    path_so_far: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    for edge in edges.iter().filter(|x| x[0] == at || x[1] == at) {
        let other_node = if edge[0] == at { &edge[1] } else { &edge[0] };
        if other_node.chars().next().unwrap().is_lowercase()
            && path_so_far.iter().any(|x| x == other_node)
        {
            continue;
        }
        let mut path = path_so_far.clone();
        path.push(other_node);
        if other_node == "end" {
            paths.push(path);
        } else {
            for other_path in traverse(edges, other_node, path) {
                paths.push(other_path);
            }
        }
    }
    paths
}

fn part1(edges: &Vec<[String; 2]>) -> String {
    let paths = traverse(edges, "start", vec!["start"]);

    format!("{}", paths.len())
}

fn calc_max_small_cave_visits(path_so_far: &[&str]) -> usize {
    let len = path_so_far.len();
    for index in 0..len {
        let node = path_so_far[index];
        if node.chars().next().unwrap().is_lowercase()
            && path_so_far[index + 1..len].iter().any(|x| *x == node)
        {
            return 1;
        }
    }
    2
}

fn traverse2<'a>(
    edges: &'a [[String; 2]],
    at: &'a str,
    path_so_far: Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];

    let max_small_cave_visits = calc_max_small_cave_visits(&path_so_far);
    for edge in edges.iter().filter(|x| x[0] == at || x[1] == at) {
        let other_node = if edge[0] == at { &edge[1] } else { &edge[0] };
        if other_node == "start" {
            continue;
        }
        if other_node.chars().next().unwrap().is_lowercase()
            && path_so_far.iter().filter(|x| x == &other_node).count() >= max_small_cave_visits
        {
            continue;
        }
        let mut path = path_so_far.clone();
        path.push(other_node);
        if other_node == "end" {
            paths.push(path);
        } else {
            for other_path in traverse2(edges, other_node, path) {
                paths.push(other_path);
            }
        }
    }
    paths
}

fn part2(edges: &Vec<[String; 2]>) -> String {
    let paths = traverse2(edges, "start", vec!["start"]);
    format!("{}", paths.len())
}
