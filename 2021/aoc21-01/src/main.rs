use std::fs;

fn main() {
    let sample = fs::read_to_string("./sample.txt")
        .expect("Something went wrong reading the file");
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    part1(&sample, "sample");
    part2(&sample, "sample");

    part1(&input, "input");
    part2(&input, "input");
}

fn parse(contents:&str) -> Vec<u32> {
    return contents.lines().into_iter().map(|x| x.parse().expect("invalid input")).collect();
}

fn part1(contents:&str, description: &str) {
    let contents = parse(contents);
    let count = contents.windows(2).filter(|x| x[1] > x[0]).count();

    println!("Answer (Part 1) ({}) = {}", description, count)
}

fn part2(contents:&str, description: &str) {
    let contents = parse(contents);

    // this probably could be done without collecting early here (resuling in more memory use)
    // but I'm not familiar enough with rust to do it
    let windows_of_3:Vec<u32> = contents.windows(3).map(|x| x.into_iter().sum()).collect();

    let count = windows_of_3.windows(2).filter(|x| x[1] > x[0]).count();
    println!("Answer (Part 2) ({}) = {}", description, count)
}
