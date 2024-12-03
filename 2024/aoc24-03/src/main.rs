use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug)]
enum Call {
    Mul(usize, usize),
    Do,
    Dont,
}

#[derive(Debug)]
enum State {
    Start,
    M,
    U,
    L,
    MulParen,
    MulArg1Digits { start: usize },
    MulComma { a: usize },
    MulArg2Digits { a: usize, start: usize },
    Mul { a: usize, b: usize },
    D,
    O,
    DoParen,
    Do,
    N,
    Apos,
    T,
    DontParen,
    Dont,
}

impl State {
    fn consume(self, input: &str, pos: usize, c: char) -> State {
        use State::*;

        // parsing logic is really simple since m or d are only present at the start of a token -
        // any time we get an m or a d we start over in the state "chain", then all we need to do
        // is look for the next valid character in the chain and move to the corresponding state or
        // back to start
        //
        // Example state movement:
        // "mumul(1,2)"
        // Start -> M -> U -> M -> U -> L -> MulParen -> MulArg1Digits -> MulComma -> MulArg2Digits -> Mul
        // "do()"
        // Start -> D -> O -> DoParen -> Do
        // "don't()"
        // Start -> D -> O -> N -> Apos -> T -> DontParen -> Dont
        // "mul[1,2]"
        // Start -> M -> U -> L -> Start...
        //
        // Any invalid next character returns the state to "Start"
        match (self, c) {
            (_, 'm') => M,
            (M, 'u') => U,
            (U, 'l') => L,
            (L, '(') => MulParen,
            (MulParen, '0'..='9') => MulArg1Digits { start: pos },
            (MulArg1Digits { start }, '0'..='9') => MulArg1Digits { start },
            (MulArg1Digits { start }, ',') => MulComma {
                a: input[start..pos].parse().unwrap(),
            },
            (MulComma { a }, '0'..='9') => MulArg2Digits { a, start: pos },
            (MulArg2Digits { a, start }, '0'..='9') => MulArg2Digits { a, start },
            (MulArg2Digits { a, start }, ')') => Mul {
                a,
                b: input[start..pos].parse().unwrap(),
            },

            (_, 'd') => D,
            (D, 'o') => O,
            (O, '(') => DoParen,
            (DoParen, ')') => Do,

            (O, 'n') => N,
            (N, '\'') => Apos,
            (Apos, 't') => T,
            (T, '(') => DontParen,
            (DontParen, ')') => Dont,

            _ => Start,
        }
    }
}

struct Problem {
    calls: Vec<Call>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut calls = vec![];
        let mut state = State::Start;

        for (pos, c) in contents.char_indices() {
            state = state.consume(contents, pos, c);
            if let Some(call) = match state {
                State::Mul { a, b } => Some(Call::Mul(a, b)),
                State::Do => Some(Call::Do),
                State::Dont => Some(Call::Dont),
                _ => None,
            } {
                calls.push(call);
            }
        }

        Ok(Self { calls })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .calls
            .iter()
            .map(|c| match c {
                Call::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum::<usize>())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut sum = 0;
        let mut enabled = true;

        for call in &self.calls {
            match call {
                Call::Mul(a, b) if enabled => {
                    sum += a * b;
                }
                Call::Do => {
                    enabled = true;
                }
                Call::Dont => {
                    enabled = false;
                }
                _ => {}
            }
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE_1)?;

        let result = problem.part1()?;

        assert_eq!(161, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_2)?;

        let result = problem.part2()?;

        assert_eq!(48, result);

        Ok(())
    }

    const SAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const SAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))<D-b>";
}
