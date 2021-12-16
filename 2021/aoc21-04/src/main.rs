#![feature(drain_filter)]

use aoc_common::run;
use std::fmt;

fn main() {
    run(parse, part1, part2);
}

struct Game {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    cells: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
}

fn empty_board() -> Board {
    Board {
        cells: [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        marks: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
    }
}

impl Board {
    fn mark(&mut self, draw: u32) -> bool {
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == draw {
                    self.marks[x][y] = true;
                }
            }
        }

        // check horizontal win
        for x in 0..5 {
            let mut win = true;
            for y in 0..5 {
                win = win && self.marks[x][y];
            }
            if win {
                return true;
            }
        }

        // check vertical win
        for y in 0..5 {
            let mut win = true;
            for x in 0..5 {
                win = win && self.marks[x][y];
            }
            if win {
                return true;
            }
        }

        false
    }

    fn score(&self, draw: u32) -> u32 {
        let mut score: u32 = 0;
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if !self.marks[x][y] {
                    score += cell;
                }
            }
        }
        score * draw
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
{}{:>2} {}{:>2} {}{:>2} {}{:>2} {}{:>2}
{}{:>2} {}{:>2} {}{:>2} {}{:>2} {}{:>2}
{}{:>2} {}{:>2} {}{:>2} {}{:>2} {}{:>2}
{}{:>2} {}{:>2} {}{:>2} {}{:>2} {}{:>2}
{}{:>2} {}{:>2} {}{:>2} {}{:>2} {}{:>2}
",
            bool_star(self.marks[0][0]),
            self.cells[0][0],
            bool_star(self.marks[0][1]),
            self.cells[0][1],
            bool_star(self.marks[0][2]),
            self.cells[0][2],
            bool_star(self.marks[0][3]),
            self.cells[0][3],
            bool_star(self.marks[0][4]),
            self.cells[0][4],
            bool_star(self.marks[1][0]),
            self.cells[1][0],
            bool_star(self.marks[1][1]),
            self.cells[1][1],
            bool_star(self.marks[1][2]),
            self.cells[1][2],
            bool_star(self.marks[1][3]),
            self.cells[1][3],
            bool_star(self.marks[1][4]),
            self.cells[1][4],
            bool_star(self.marks[2][0]),
            self.cells[2][0],
            bool_star(self.marks[2][1]),
            self.cells[2][1],
            bool_star(self.marks[2][2]),
            self.cells[2][2],
            bool_star(self.marks[2][3]),
            self.cells[2][3],
            bool_star(self.marks[2][4]),
            self.cells[2][4],
            bool_star(self.marks[3][0]),
            self.cells[3][0],
            bool_star(self.marks[3][1]),
            self.cells[3][1],
            bool_star(self.marks[3][2]),
            self.cells[3][2],
            bool_star(self.marks[3][3]),
            self.cells[3][3],
            bool_star(self.marks[3][4]),
            self.cells[3][4],
            bool_star(self.marks[4][0]),
            self.cells[4][0],
            bool_star(self.marks[4][1]),
            self.cells[4][1],
            bool_star(self.marks[4][2]),
            self.cells[4][2],
            bool_star(self.marks[4][3]),
            self.cells[4][3],
            bool_star(self.marks[4][4]),
            self.cells[4][4],
        )
    }
}

fn bool_star(a: bool) -> &'static str {
    if a {
        return "*";
    }
    " "
}

fn lines_to_board(lines: &str) -> Result<Board, ()> {
    let lines: Vec<&str> = lines
        .lines()
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect();
    if lines.len() < 5 {
        return Err(());
    }
    let mut board = empty_board();
    for (x, line) in lines[..5].iter().enumerate() {
        let cells: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if cells.len() != 5 {
            return Err(());
        }
        for (y, cell) in cells.into_iter().enumerate() {
            board.cells[x][y] = cell;
        }
    }
    Ok(board)
}

fn parse(contents: &str) -> Game {
    let mut parts: Vec<&str> = contents.split("\n\n").into_iter().collect();

    let boards = parts.split_off(1);

    let draws: Vec<u32> = parts[0]
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let boards: Vec<Board> = boards
        .into_iter()
        .map(|x| lines_to_board(x).unwrap())
        .collect();

    Game { draws, boards }
}

fn part1(game: &Game) -> u32 {
    let mut boards = game.boards.clone();

    for draw in &game.draws {
        for board in &mut boards {
            if board.mark(*draw) {
                return board.score(*draw);
            }
        }
    }

    0
}

fn part2(game: &Game) -> u32 {
    let mut boards = game.boards.clone();

    for draw in &game.draws {
        let mut last_win: Option<u32> = None;
        boards.drain_filter(|x| {
            if x.mark(*draw) {
                last_win = Some(x.score(*draw));
                return true;
            }
            false
        });
        if boards.is_empty() {
            return last_win.unwrap();
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 4512);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 1924);
    }

    const SAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
}
