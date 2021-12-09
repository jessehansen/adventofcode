use aoc_common::run;

fn main() {
    run(&parse, &part1, &part2);
}

fn parse(contents:&str) -> Vec<Vec<u32>> {
    contents.lines().into_iter().map(|x| x.chars().into_iter().map(|x| {
        let result:u32 = x.to_string().parse().unwrap();
        result
    }).collect()).collect()
}

fn part1(contents:&Vec<Vec<u32>>) -> String {

    let mut risk = 0;
    for i in 0..contents.len() {
        let row = &contents[i];
        for j in 0..row.len() {
            let height = row[j];
            if i > 0 && contents[i-1][j] <= height {
                continue;
            }
            if i < contents.len() - 1 && contents[i+1][j] <= height {
                continue;
            }
            if j > 0 && row[j-1] <= height {
                continue;
            }
            if j < row.len() - 1 && row[j+1] <= height {
                continue;
            }
            risk += height + 1;
        }
    }

    format!("{}", risk)
}

fn calculate_basin_size(contents:&Vec<Vec<u32>>, low_point_i: usize, low_point_j:usize) -> usize {
    let mut basin = vec![(low_point_i,low_point_j)];
    let mut last_size = 0;

    while basin.len() > last_size {
        last_size = basin.len();
        let basin_copy = basin.clone();
        for (i, j) in basin_copy {
            let row = &contents[i];
            let height = row[j];
            if i > 0 && contents[i-1][j] >= height && contents[i-1][j] < 9 &&
                !basin.contains(&(i-1,j)) {
                basin.push((i-1, j));
            }
            if i < contents.len() - 1 && contents[i+1][j] >= height && contents[i+1][j] < 9 &&
                !basin.contains(&(i+1,j))  {
                basin.push((i+1, j));
            }
            if j > 0 && row[j-1] >= height && row[j-1] < 9 &&
                !basin.contains(&(i,j-1))  {
                basin.push((i, j-1));
            }
            if j < row.len() - 1 && row[j+1] >= height && row[j+1] < 9 &&
                !basin.contains(&(i,j+1))  {
                basin.push((i, j+1));
            }
        }
    }

    basin.len()
}

fn part2(contents:&Vec<Vec<u32>>) -> String {
    let mut low_points = vec![];

    for i in 0..contents.len() {
        let row = &contents[i];
        for j in 0..row.len() {
            let height = row[j];
            if i > 0 && contents[i-1][j] <= height {
                continue;
            }
            if i < contents.len() - 1 && contents[i+1][j] <= height {
                continue;
            }
            if j > 0 && row[j-1] <= height {
                continue;
            }
            if j < row.len() - 1 && row[j+1] <= height {
                continue;
            }
            // found a low point, save for later
            low_points.push((i, j));
        }
    }

    let mut basin_sizes = vec![];

    for (i, j) in low_points {
        basin_sizes.push(calculate_basin_size(contents, i, j));
    }

    basin_sizes.sort();

    let result = basin_sizes.iter().rev().take(3).fold(1, |acc, x| acc * x);

    format!("{}", result)
}
