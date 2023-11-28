use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

fn parse(contents: &str) -> Result<Vec<String>> {
    Ok(contents.lines().map(|x| x.to_string()).collect())
}

fn part1(contents: &[String]) -> Result<u32> {
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

                c => bail!("unexpected character '{}' in input", c),
            }
        }
    }

    Ok(score)
}

fn part2(contents: &[String]) -> Result<u64> {
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

                c => bail!("unexpected character '{}' in input", c),
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
                    c => bail!("unexpected character '{}' in input", c),
                }
            }
            line_scores.push(line_score);
        }
    }
    line_scores.sort_unstable();

    Ok(line_scores[line_scores.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 26397);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 288957);

        Ok(())
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
