use aoc_common::*;

fn main() {
    run_vec(parse, part1, part2);
}

fn parse(contents: &str) -> Vec<String> {
    contents
        .lines()
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

fn part1(contents: &[String]) -> u32 {
    let mut score = 0;

    for line in contents {
        let mut open: Vec<char> = vec![];
        for char in line.chars() {
            match char {
                '(' => open.push(char),
                '[' => open.push(char),
                '{' => open.push(char),
                '<' => open.push(char),

                ')' => {
                    if open.pop() != Some('(') {
                        score += 3;
                    }
                }
                ']' => {
                    if open.pop() != Some('[') {
                        score += 57;
                    }
                }
                '}' => {
                    if open.pop() != Some('{') {
                        score += 1197;
                    }
                }
                '>' => {
                    if open.pop() != Some('<') {
                        score += 25137;
                    }
                }

                _ => (),
            }
        }
    }

    score
}

fn part2(contents: &[String]) -> u64 {
    let mut line_scores = vec![];

    for line in contents {
        let mut open: Vec<char> = vec![];
        let mut syntax_error = false;
        for char in line.chars() {
            match char {
                '(' => open.push(char),
                '[' => open.push(char),
                '{' => open.push(char),
                '<' => open.push(char),

                ')' => {
                    if open.pop() != Some('(') {
                        syntax_error = true;
                    }
                }
                ']' => {
                    if open.pop() != Some('[') {
                        syntax_error = true;
                    }
                }
                '}' => {
                    if open.pop() != Some('{') {
                        syntax_error = true;
                    }
                }
                '>' => {
                    if open.pop() != Some('<') {
                        syntax_error = true;
                    }
                }

                _ => (),
            }
        }

        if !syntax_error {
            let mut line_score: u64 = 0;
            for char in open.iter().rev() {
                match char {
                    '(' => line_score = line_score * 5 + 1,
                    '[' => line_score = line_score * 5 + 2,
                    '{' => line_score = line_score * 5 + 3,
                    '<' => line_score = line_score * 5 + 4,
                    _ => (),
                }
            }
            line_scores.push(line_score);
        }
    }
    line_scores.sort_unstable();

    line_scores[line_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 26397);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 288957);
    }

    const SAMPLE: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
}
