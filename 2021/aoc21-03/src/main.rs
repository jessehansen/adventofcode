use aoc_common::run;

fn main() {
    run(&parse, &part1, &part2);
}

fn parse(contents:&str) -> Vec<String> {
    contents.lines().into_iter().map(|x| x.to_string()).collect()
}

fn part1(contents: &Vec<String>) -> String {
    let item_length = contents[0].len();
    let half_line_count = contents.len() / 2;
    let mut mcbs = vec![0; item_length];
    for bit_string in contents {
        for (pos, mcb) in mcbs.iter_mut().enumerate() {
            if bit_string.chars().nth(pos).unwrap() == '1' {
                *mcb = *mcb + 1;
            }
        }
    }

    let mut gamma:u32 = 0;
    let mut epsilon:u32 = 0;
    for (pos, mcb) in mcbs.iter().enumerate() {
        let bit_pos = item_length - pos - 1;

        if *mcb > half_line_count {
            gamma |= 1 << bit_pos;
        } else {
            epsilon |= 1 << bit_pos;
        }
    }

    format!("gamma={:b}, epsilon={:b}, consumption={}", gamma, epsilon, gamma * epsilon)
}

fn part2(contents: &Vec<String>) -> String {
    let item_length = contents[0].len();

    let mut oxygen_lines = contents.clone();
    let mut scrubber_lines = contents.clone();

    let pos_vec = vec![0; item_length];

    for (pos, _) in pos_vec.iter().enumerate() {
        if oxygen_lines.len() > 1 {
            let oxygen_mcb = get_mcb_at_pos(&oxygen_lines, pos);
            oxygen_lines = oxygen_lines.into_iter().filter(|x| x.chars().nth(pos).unwrap() == oxygen_mcb).collect();
        }
        if scrubber_lines.len() > 1 {
            let scrubber_lcb = if get_mcb_at_pos(&scrubber_lines, pos) == '1' { '0' } else { '1' };
            scrubber_lines = scrubber_lines.into_iter().filter(|x| x.chars().nth(pos).unwrap() == scrubber_lcb).collect();
        }
        if oxygen_lines.len() == 1 && scrubber_lines.len() == 1 {
            break;
        }
    }

    let oxygen_rating = u32::from_str_radix(oxygen_lines[0].as_str(), 2).unwrap();
    let scrubber_rating = u32::from_str_radix(scrubber_lines[0].as_str(), 2).unwrap();

    format!("oxygen={}, scrubber={}, life_support={}", oxygen_rating, scrubber_rating, oxygen_rating * scrubber_rating)
}

fn get_mcb_at_pos(lines:&Vec<String>, pos: usize) -> char {
    let half_line_count = lines.len() as f64 / 2.0;
    let line_count = lines.iter().filter(|x| x.chars().nth(pos).unwrap() == '1').count() as f64;

    if line_count >= half_line_count {
        return '1';
    }

    return '0';
}
