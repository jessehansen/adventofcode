use anyhow::*;
use aoc_common::run;
use aoc_common::Point2D;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

fn main() -> Result<()> {
    run(parse, part1, part2)
}

fn parse(contents: &str) -> Result<(Vec<Point2D>, Vec<Fold>)> {
    let mut parts = contents.split("\n\n");

    let points = parts
        .next()
        .ok_or(anyhow!("missing points"))?
        .lines()
        .into_iter()
        .map(|x| {
            x.parse()
                .context(anyhow!("can't parse point from line {}", x))
        })
        .collect::<Result<Vec<Point2D>>>()?;
    let folds = parts
        .next()
        .ok_or(anyhow!("missing folds"))?
        .lines()
        .into_iter()
        .map(|x| {
            x.parse()
                .context(anyhow!("can't parse fold from line {}", x))
        })
        .collect::<Result<Vec<Fold>>>()?;

    Ok((points, folds))
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
        std::result::Result::Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(input: &str) -> Result<Fold> {
        let mut parts = input.split('=');

        match parts.next().ok_or(anyhow!("missing fold direction"))? {
            "fold along x" => Ok(Fold::Horizontal(
                parts
                    .next()
                    .ok_or(anyhow!("missing fold location"))?
                    .parse()?,
            )),
            "fold along y" => Ok(Fold::Vertical(
                parts
                    .next()
                    .ok_or(anyhow!("missing fold location"))?
                    .parse()?,
            )),
            fold_along => bail!("unexpected fold along text {}", fold_along),
        }
    }
}

fn part1((marks, folds): &(Vec<Point2D>, Vec<Fold>)) -> Result<usize> {
    let mut sheet = OrigamiSheet::new(marks);

    sheet.fold(folds[0]);

    Ok(sheet.marks.len())
}

fn part2((marks, folds): &(Vec<Point2D>, Vec<Fold>)) -> Result<OrigamiSheet> {
    let mut sheet = OrigamiSheet::new(marks);

    for fold in folds {
        sheet.fold(*fold);
    }

    Ok(sheet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 17);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

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

        Ok(())
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
