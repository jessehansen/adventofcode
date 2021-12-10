use aoc_common::run;

fn main() {
    run(parse, part1, part2);
}

fn parse(contents:&str) -> Vec<i32> {
    contents.lines().into_iter().next().unwrap().
        split(",").map(|x| x.parse().expect("invalid input")).collect()
}

fn part1(contents:&Vec<i32>) -> String {
    let min = contents.iter().min().unwrap();
    let max = contents.iter().max().unwrap();

    let mut least_fuel = std::i32::MAX;

    for i in *min..*max {
        let fuel = contents.into_iter().map(|x| (x-i).abs()).sum();
        if fuel < least_fuel {
            least_fuel = fuel
        }
    }

    format!("{}", least_fuel)
}

// returns the value for the nth triangle number
// see https://en.wikipedia.org/wiki/Triangular_number
fn triangle_number(n: i32) -> i32 {
    (n * n + n) / 2
}

fn part2(contents:&Vec<i32>) -> String {
    let min = contents.iter().min().unwrap();
    let max = contents.iter().max().unwrap();

    let mut least_fuel = std::i32::MAX;

    for i in *min..*max {
        let fuel = contents.into_iter().map(|x| triangle_number((x-i).abs())).sum();
        if fuel < least_fuel {
            least_fuel = fuel
        }
    }

    format!("{}", least_fuel)
}
