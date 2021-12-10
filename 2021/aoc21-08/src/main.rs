use aoc_common::run;
use itertools::Itertools;

fn main() {
    run(parse, part1, part2);
}

struct DisplayLine {
    signal: Vec<String>,
    output: Vec<String>,
}

fn parse(contents: &str) -> Vec<DisplayLine> {
    contents
        .lines()
        .into_iter()
        .map(|x| {
            let mut parts = x.split("|");

            DisplayLine {
                signal: parts
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect(),
                output: parts
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect(),
            }
        })
        .collect()
}

fn part1(contents: &Vec<DisplayLine>) -> String {
    let count: usize = contents
        .iter()
        .map(|x| {
            x.output
                .iter()
                .filter(|y| match y.len() {
                    2 => true,
                    4 => true,
                    3 => true,
                    7 => true,
                    _ => false,
                })
                .count()
        })
        .sum();

    format!("{}", count)
}

fn get_codes(signal: &Vec<String>) -> [String; 10] {
    let mut result: [String; 10] = Default::default();
    let mut five_segment: Vec<&str> = vec![];
    let mut six_segment: Vec<&str> = vec![];
    let mut one = "";

    for data in signal {
        match data.len() {
            2 => {
                result[1] = data.to_string();
                one = data;
            }
            3 => result[7] = data.to_string(),
            4 => result[4] = data.to_string(),
            7 => result[8] = data.to_string(),
            5 => {
                five_segment.push(data);
            }
            6 => {
                six_segment.push(data);
            }
            _ => panic!("invalid signal datum length"),
        }
    }

    // collect bottom_left and bottom - the only ones in 8 but not 4 or 7
    let bottom_left_and_bottom: String = result[8]
        .chars()
        .into_iter()
        .filter(|x| !result[4].contains(*x) && !result[7].contains(*x))
        .collect();

    let mut top_right = 'z';
    for data in six_segment {
        // must be 0, 6, or 9
        let mut six_or_nine = false;
        // we can make some deductions from six-segment numbers: 0, 6, and 9

        // detect 6 (& store the top right segment for later)
        for char in one.chars() {
            if !data.contains(char) {
                top_right = char;

                result[6] = data.to_string();
                six_or_nine = true;
            }
        }

        // detect 9
        for char in bottom_left_and_bottom.chars() {
            if !data.contains(char) {
                result[9] = data.to_string();
                six_or_nine = true;
            }
        }

        // must be 0
        if !six_or_nine {
            result[0] = data.to_string();
        }
    }

    for data in five_segment {
        // 2, 3, and 5

        if result[1].chars().filter(|x| data.contains(*x)).count() == 2 {
            result[3] = data.to_string();
        } else if data.contains(top_right) {
            result[2] = data.to_string();
        } else {
            result[5] = data.to_string();
        }
    }

    for item in result.iter_mut() {
        *item = item.chars().sorted().collect::<String>();
    }

    result
}

fn decode_output(codes: [String; 10], output: &Vec<String>) -> u32 {
    let mut result = "".to_string();
    for data in output {
        let sorted = data.chars().sorted().collect::<String>();

        for (digit, code) in codes.iter().enumerate() {
            if *code == sorted {
                result += &format!("{}", digit);
            }
        }
    }

    result.parse().unwrap()
}

fn part2(contents: &Vec<DisplayLine>) -> String {
    let sum: u32 = contents
        .iter()
        .map(|x| {
            let codes = get_codes(&x.signal);
            decode_output(codes, &x.output)
        })
        .sum();

    format!("{}", sum)
}
