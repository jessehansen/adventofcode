use aoc_common::*;

fn main() {
    run_vec(parse_lines, part1, part2);
}

fn is_nice_part1(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut last = '\n';
    let mut double = false;
    for c in s.chars() {
        if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowel_count += 1;
        }
        if last == c {
            double = true;
        }
        if (last == 'a' && c == 'b')
            || (last == 'c' && c == 'd')
            || (last == 'p' && c == 'q')
            || (last == 'x' && c == 'y')
        {
            return false;
        }

        last = c;
    }

    vowel_count > 2 && double
}

fn is_nice_part2(s: &str) -> bool {
    let mut has_duplicate_pair = false;
    let mut has_surrounding_double = false;
    for ix in 0..s.len() - 1 {
        if let Some(pair) = s.get(ix..ix + 2) {
            if let Some(other_pair_index) = s.rfind(pair) {
                if other_pair_index > ix + 1 {
                    has_duplicate_pair = true;
                }
            }
        }
        if ix < s.len() - 2 && s.get(ix..=ix) == s.get(ix + 2..=ix + 2) {
            has_surrounding_double = true;
        }
    }

    has_duplicate_pair && has_surrounding_double
}

fn part1(contents: &[String]) -> usize {
    contents.iter().filter(|x| is_nice_part1(x)).count()
}

fn part2(contents: &[String]) -> usize {
    contents.iter().filter(|x| is_nice_part2(x)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nice_part1_check() {
        assert!(is_nice_part1("ugknbfddgicrmopn"));
        assert!(is_nice_part1("aaa"));
        assert!(!is_nice_part1("jchzalrnumimnmhp"));
        assert!(!is_nice_part1("haegwjzuvuyypxyu"));
        assert!(!is_nice_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn is_nice_part2_check() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
