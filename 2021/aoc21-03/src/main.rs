use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

fn parse(contents: &str) -> Result<Vec<String>> {
    Ok(contents.lines().map(|x| x.to_string()).collect())
}

fn part1(contents: &[String]) -> Result<u32> {
    let item_length = contents[0].len();
    let half_line_count = contents.len() / 2;
    let mut mcbs = vec![0; item_length];
    for bit_string in contents {
        for (pos, mcb) in mcbs.iter_mut().enumerate() {
            if bit_string.chars().nth(pos).unwrap() == '1' {
                *mcb += 1;
            }
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (pos, mcb) in mcbs.iter().enumerate() {
        let bit_pos = item_length - pos - 1;

        if *mcb > half_line_count {
            gamma |= 1 << bit_pos;
        } else {
            epsilon |= 1 << bit_pos;
        }
    }

    Ok(gamma * epsilon)
}

fn part2(contents: &[String]) -> Result<u32> {
    let item_length = contents[0].len();

    let mut oxygen_lines = contents.to_owned();
    let mut scrubber_lines = contents.to_owned();

    let pos_vec = vec![0; item_length];

    for (pos, _) in pos_vec.iter().enumerate() {
        if oxygen_lines.len() > 1 {
            let oxygen_mcb = get_mcb_at_pos(&oxygen_lines, pos);
            oxygen_lines = oxygen_lines
                .into_iter()
                .filter(|x| x.chars().nth(pos).unwrap() == oxygen_mcb)
                .collect::<Vec<String>>();
        }
        if scrubber_lines.len() > 1 {
            let scrubber_lcb = if get_mcb_at_pos(&scrubber_lines, pos) == '1' {
                '0'
            } else {
                '1'
            };
            scrubber_lines = scrubber_lines
                .into_iter()
                .filter(|x| x.chars().nth(pos).unwrap() == scrubber_lcb)
                .collect::<Vec<String>>();
        }
        if oxygen_lines.len() == 1 && scrubber_lines.len() == 1 {
            break;
        }
    }

    let oxygen_rating = u32::from_str_radix(oxygen_lines[0].as_str(), 2).unwrap();
    let scrubber_rating = u32::from_str_radix(scrubber_lines[0].as_str(), 2).unwrap();

    Ok(oxygen_rating * scrubber_rating)
}

fn get_mcb_at_pos(lines: &[String], pos: usize) -> char {
    let half_line_count = lines.len() as f64 / 2.0;
    let line_count = lines
        .iter()
        .filter(|x| x.chars().nth(pos).unwrap() == '1')
        .count() as f64;

    if line_count >= half_line_count {
        '1'
    } else {
        '0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 198);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 230);

        Ok(())
    }

    const SAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";
}
