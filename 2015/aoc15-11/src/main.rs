use aoc_common::*;
use std::collections::HashSet;

fn main() {
    run_raw(part1, part2);
}

fn meets_requirements(password: &[char]) -> bool {
    let mut increasing_straight = 0;
    let mut has_increasing_straight = false;
    let mut last_char = '\n';
    let mut pairs = HashSet::new();
    for c in password {
        if matches!(c, 'i' | 'o' | 'l') {
            return false;
        }
        if c == &last_char {
            pairs.insert(c);
        }
        if c == &((last_char as u8 + 1) as char) {
            increasing_straight += 1;
        } else {
            increasing_straight = 0;
        }
        if increasing_straight > 1 {
            has_increasing_straight = true
        }
        last_char = *c;
    }

    has_increasing_straight && pairs.len() > 1
}

fn next_char(c: char) -> char {
    match c {
        'z' => 'a',
        'h' | 'n' | 'k' => (c as u8 + 2) as char,
        _ => (c as u8 + 1) as char,
    }
}

fn next_pass(password: &str) -> String {
    let mut pass: Vec<char> = password.chars().collect();
    loop {
        let mut pos = pass.len() - 1;
        loop {
            pass[pos] = next_char(pass[pos]);
            if pass[pos] != 'a' {
                break;
            }

            if pos == 0 {
                panic!();
            }
            pos -= 1;
        }
        if meets_requirements(&pass) {
            break;
        }
    }
    pass.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn part1(password: &str) -> String {
    next_pass(password)
}

fn part2(password: &str) -> String {
    next_pass(&next_pass(password))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meets_requirements(password: &str) -> bool {
        super::meets_requirements(&password.chars().collect::<Vec<char>>())
    }

    #[test]
    fn meets_requirements_test() {
        assert!(!meets_requirements("hijklmmn"));
        assert!(!meets_requirements("abbceffg"));
        assert!(!meets_requirements("abbcegjk"));
        assert!(meets_requirements("abcdffaa"));
        assert!(meets_requirements("ghjaabcc"));
    }

    #[test]
    fn next_pass_test() {
        assert_eq!(next_pass("abcdefgh"), "abcdffaa");
        assert_eq!(next_pass("ghijklmn"), "ghjaabcc");
    }
}
