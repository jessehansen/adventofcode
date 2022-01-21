use aoc_common::*;

fn main() {
    run_vec(parse_chars, part1, part2);
}

fn look_and_say_step(input: &[u8]) -> Vec<u8> {
    let mut result = vec![];
    let mut last_char = &0;
    let mut count = 0;
    for char in input {
        if char != last_char && count > 0 {
            result.push(count);
            result.push(*last_char);
            count = 0;
        }
        last_char = char;
        count += 1;
    }
    result.push(count);
    result.push(*last_char);
    result
}

fn look_and_say(start: &[u8], steps: usize) -> Vec<u8> {
    let mut result: Vec<u8> = start.to_vec();
    for _ in 0..steps {
        result = look_and_say_step(&result);
    }
    result
}

fn part1(start: &[u8]) -> usize {
    look_and_say(start, 40).len()
}

fn part2(start: &[u8]) -> usize {
    look_and_say(start, 50).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_and_say_test() {
        let mut input = vec![1];

        input = look_and_say_step(&input);
        assert_eq!(input, vec![1, 1]);

        input = look_and_say_step(&input);
        assert_eq!(input, vec![2, 1]);

        input = look_and_say_step(&input);
        assert_eq!(input, vec![1, 2, 1, 1]);

        input = look_and_say_step(&input);
        assert_eq!(input, vec![1, 1, 1, 2, 2, 1]);

        input = look_and_say_step(&input);
        assert_eq!(input, vec![3, 1, 2, 2, 1, 1]);
    }
}
