use anyhow::*;
use aoc_common::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    run(parse, part1, part2)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct GameBoard {
    positions: [usize; 2],
    scores: [usize; 2],
}

impl GameBoard {
    fn new(position_1: usize, position_2: usize) -> GameBoard {
        GameBoard {
            positions: [position_1, position_2],
            scores: [0; 2],
        }
    }

    fn take_turn(&self, player: usize, roll: usize) -> GameBoard {
        let mut positions = self.positions;
        let mut scores = self.scores;

        let pos = (self.positions[player] + roll - 1) % 10 + 1;
        let score = self.scores[player] + pos;
        positions[player] = pos;
        scores[player] = score;

        GameBoard { positions, scores }
    }

    fn is_win(&self, target_score: usize) -> bool {
        self.scores.iter().any(|x| *x >= target_score)
    }
}

fn parse(contents: &str) -> Result<GameBoard> {
    let positions = contents
        .lines()
        .map(|x| {
            Ok(x.split(": ")
                .last()
                .ok_or(anyhow!("missing content on line"))?
                .parse()?)
        })
        .collect::<Result<Vec<usize>>>()?;

    Ok(GameBoard::new(positions[0], positions[1]))
}

struct DeterministicDie {
    next_roll: usize,
    roll_count: u32,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie {
            next_roll: 1,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> usize {
        let roll = self.next_roll;
        self.next_roll += 1;
        if self.next_roll > 100 {
            self.next_roll = 1;
        }
        self.roll_count += 1;
        roll
    }
}

fn part1(board: &GameBoard) -> Result<u32> {
    let mut board = *board;
    let mut player = 0;
    let mut die = DeterministicDie::new();
    while !board.is_win(1000) {
        let roll = die.roll() + die.roll() + die.roll();

        board = board.take_turn(player, roll);

        player = (player + 1) % 2;
    }
    Ok(board.scores[player] as u32 * die.roll_count)
}

const DIRAC_ROLL_3: [u64; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn roll_dirac() -> impl Iterator<Item = (usize, u64)> {
    DIRAC_ROLL_3
        .into_iter()
        .enumerate()
        .filter(|(_, count)| *count > 0)
}

struct Multiverse {
    running_games: HashMap<GameBoard, u64>,
    wins: [u64; 2],
    next_turn: usize,
}

impl Multiverse {
    fn new(board: GameBoard) -> Multiverse {
        Multiverse {
            running_games: HashMap::from([(board, 1)]),
            wins: [0; 2],
            next_turn: 0,
        }
    }

    fn take_turn(&mut self) -> Option<u64> {
        let player = self.next_turn;

        let mut turn_games = HashMap::new();
        for (game, universes) in &self.running_games {
            for (roll, split_count) in roll_dirac() {
                let new_universes = universes * split_count;
                let new_game = game.take_turn(player, roll);
                if new_game.is_win(21) {
                    self.wins[player] += new_universes;
                    continue;
                }
                let games_at_state = turn_games.entry(new_game).or_insert(0);
                *games_at_state += new_universes;
            }
        }
        self.next_turn = (player + 1) % 2;
        self.running_games = turn_games;

        if self.running_games.is_empty() {
            self.wins.into_iter().max()
        } else {
            None
        }
    }
}

fn part2(board: &GameBoard) -> Result<u64> {
    let mut multiverse = Multiverse::new(*board);
    loop {
        if let Some(winning_universes) = multiverse.take_turn() {
            return Ok(winning_universes);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 739785);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 444356092776315);

        Ok(())
    }

    const SAMPLE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8
";
}
