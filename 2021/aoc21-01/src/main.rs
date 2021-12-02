use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    // for part 1 answer
    let mut prev = 0;
    let mut count = -1; // start at -1 since the first entry doesn't count

    // for part 2 answer, keep a sliding sum.
    let mut prev_window = 0;
    let mut sliding_count = -3; // start at -3 since the first sliding window doesn't count
    let mut sliding_index = 0;
    let mut sliding_window = [0,0,0];
    for line in contents.lines().into_iter() {
        // parse input line
        let cur = line.parse().expect("invalid input");

        // calc part 1
        if cur > prev {
            count = count + 1;
        }
        prev = cur;

        // calc part 2
        sliding_window[sliding_index] = cur;
        let cur_window_sum = sliding_window[0] + sliding_window[1] + sliding_window[2];

        if cur_window_sum > prev_window {
            sliding_count = sliding_count + 1;
        }
        sliding_index = (sliding_index + 1) % 3;
        prev_window = cur_window_sum;
    }

    println!("Answer (Part 1)={}", count);
    println!("Answer (Part 2)={}", sliding_count);
}
