use aoc_common::run;
use aoc_common::Point2D;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

fn main() {
    run(parse, part1, part2);
}

fn parse(contents: &str) -> (Vec<Point2D>, Vec<Fold>) {
    let mut parts = contents.split("\n\n");

    (
        parts
            .next()
            .unwrap()
            .lines()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect(),
        parts
            .next()
            .unwrap()
            .lines()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect(),
    )
}

struct OrigamiSheet {
    marks: HashSet<Point2D>,
}

// gets the new position given a fold position
fn fold_one_d(pos: usize, fold_pos: usize) -> usize {
    fold_pos - (pos - fold_pos)
}

impl OrigamiSheet {
    fn new(marks: &[Point2D]) -> OrigamiSheet {
        OrigamiSheet {
            marks: HashSet::from_iter(marks.iter().copied()),
        }
    }

    fn fold(&mut self, fold: Fold) {
        self.marks
            .clone()
            .iter()
            .filter(|pt| match fold {
                Fold::Horizontal(x) => pt.x > x,
                Fold::Vertical(y) => pt.y > y,
            })
            .for_each(|pt| {
                self.marks.remove(pt);
                let &Point2D { x, y } = pt;
                self.marks.insert(match fold {
                    Fold::Horizontal(fold_x) => Point2D {
                        x: fold_one_d(x, fold_x),
                        y,
                    },
                    Fold::Vertical(fold_y) => Point2D {
                        x,
                        y: fold_one_d(y, fold_y),
                    },
                });
            })
    }
}

impl Display for OrigamiSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (width, height) =
            self.marks
                .iter()
                .fold((0, 0), |(max_x, max_y), &Point2D { x, y }| {
                    (
                        if x > max_x { x } else { max_x },
                        if y > max_y { y } else { max_y },
                    )
                });
        for y in 0..=height {
            for x in 0..=width {
                write!(
                    f,
                    "{}",
                    // using double-width characters because it makes the result easier to read
                    if self.marks.contains(&Point2D { x, y }) {
                        "##"
                    } else {
                        "  "
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(input: &str) -> Result<Fold, Self::Err> {
        let mut parts = input.split('=');

        match parts.next().unwrap() {
            "fold along x" => Ok(Fold::Horizontal(parts.next().unwrap().parse().unwrap())),
            "fold along y" => Ok(Fold::Vertical(parts.next().unwrap().parse().unwrap())),
            _ => Err(()),
        }
    }
}

fn part1((marks, folds): &(Vec<Point2D>, Vec<Fold>)) -> usize {
    let mut sheet = OrigamiSheet::new(marks);

    sheet.fold(folds[0]);

    sheet.marks.len()
}

fn part2((marks, folds): &(Vec<Point2D>, Vec<Fold>)) -> OrigamiSheet {
    let mut sheet = OrigamiSheet::new(marks);

    for fold in folds {
        sheet.fold(*fold);
    }

    sheet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 17);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(
            result.to_string(),
            "\
##########
##      ##
##      ##
##      ##
##########
"
        );
    }

    const SAMPLE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
}
