use aoc_common::run;

fn main() {
    run(&parse, &part1, &part2);
}

fn parse(contents:&str) -> Vec<u32> {
    contents.lines().into_iter().map(|x| x.parse().expect("invalid input")).collect()
}

fn part1(contents:&Vec<u32>) -> String {
    let count = contents.windows(2).filter(|x| x[1] > x[0]).count();

    format!("{}", count)
}

fn part2(contents:&Vec<u32>) -> String {
    // this probably could be done without collecting early here (resuling in more memory use)
    // but I'm not familiar enough with rust to do it
    let windows_of_3:Vec<u32> = contents.windows(3).map(|x| x.into_iter().sum()).collect();

    let count = windows_of_3.windows(2).filter(|x| x[1] > x[0]).count();
    format!("{}", count)
}
