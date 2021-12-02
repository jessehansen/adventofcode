use std::fs;

fn main() {
    let sample = fs::read_to_string("./sample.txt")
        .expect("Something went wrong reading the file");
    print_answer(&sample, "sample");
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");
    print_answer(&input, "input");
}

/*
 * down X increases your aim by X units.
up X decreases your aim by X units.
forward X does two things:
It increases your horizontal position by X units.
It increases your depth by your aim multiplied by X.
*/

fn print_answer(contents:&str, description: &str) {
    let mut horizontal_position = 0;
    let mut depth_1 = 0;
    let mut depth_2 = 0;
    let mut aim = 0;
    for line in contents.lines().into_iter() {
        let line: Vec<&str> =  line.split_whitespace().collect();
        let delta:u32 = line.get(1).expect("Invalid line structure").parse().expect("Ivalid line structure");
        match line.get(0).expect("Invalid line structure") {
            &"forward" => {
                horizontal_position += delta;
                depth_2 += aim * delta;
            },
            &"down" => {
                depth_1 += delta;
                aim += delta;
            },
            &"up" => {
                depth_1 -= delta;
                aim -= delta;
            },
            _ => ()
        }
    }

    println!("Answer Part 1 ({}) = x={}, depth={}, mult={}", description, horizontal_position, depth_1, horizontal_position * depth_1);
    println!("Answer Part 2 ({}) = x={}, depth={}, mult={}", description, horizontal_position, depth_2, horizontal_position * depth_2);
}
