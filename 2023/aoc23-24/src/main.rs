use std::str::FromStr;

use anyhow::*;
use aoc_common::*;
use ndarray::*;
use ndarray_linalg::*;

fn main() -> Result<()> {
    Problem::go()
}

// y=ax + b
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct FPoint2D {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct LineSegment2D {
    slope: f64,
    y_intercept: f64,
    min_x: Option<f64>,
    max_x: Option<f64>,
}

impl LineSegment2D {
    fn intersect(&self, other: &LineSegment2D) -> Option<FPoint2D> {
        if self.slope == other.slope {
            None
        } else {
            let x = (other.y_intercept - self.y_intercept) / (self.slope - other.slope);
            let y = self.slope * x + self.y_intercept;
            if self.min_x.is_some_and(|min_x| x < min_x)
                || other.min_x.is_some_and(|min_x| x < min_x)
                || self.max_x.is_some_and(|max_x| x > max_x)
                || other.max_x.is_some_and(|max_x| x > max_x)
            {
                None
            } else {
                Some(FPoint2D { x, y })
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Hail {
    position: Point3D,
    velocity: IVector3D,
}

struct Problem {
    hail: Vec<Hail>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            hail: contents.parse_lines()?,
        })
    }
}

impl FromStr for Hail {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let (position, velocity) = line.split_once(" @ ").ok_or_invalid()?;

        Ok(Self {
            position: position.parse_split_trim(',')?.try_into()?,
            velocity: velocity.parse_split_trim(',')?.try_into()?,
        })
    }
}

impl Hail {
    fn as_line_segment(&self) -> LineSegment2D {
        let x: f64 = self.position.x as f64;
        let y: f64 = self.position.y as f64;
        let dx: f64 = self.velocity.dx as f64;
        let dy: f64 = self.velocity.dy as f64;

        let slope = dy / dx;
        // y = slope*x + y_intercept
        // y_intercept = y - slope * x
        let y_intercept = y - slope * x;
        let min_x = if dx >= 0.0 { Some(x) } else { None };
        let max_x = if dx <= 0.0 { Some(x) } else { None };

        LineSegment2D {
            slope,
            y_intercept,
            min_x,
            max_x,
        }
    }
}

impl Problem {
    fn hail_index_pairs(&self) -> Vec<(usize, usize)> {
        let len = self.hail.len();
        (0..len)
            .flat_map(|first_ix| ((first_ix + 1)..len).map(move |second_ix| (first_ix, second_ix)))
            .collect()
    }

    fn count_intersections(&self, min_xy: f64, max_xy: f64) -> usize {
        let lines: Vec<_> = self
            .hail
            .iter()
            .map(|hail| hail.as_line_segment())
            .collect();

        self.hail_index_pairs()
            .into_iter()
            .filter_map(|(i0, i1)| lines[i0].intersect(&lines[i1]))
            .filter(|intersection| {
                intersection.x >= min_xy
                    && intersection.y >= min_xy
                    && intersection.x <= max_xy
                    && intersection.y <= max_xy
            })
            .count()
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.count_intersections(200_000_000_000_000.0, 400_000_000_000_000.0))
    }

    fn part2(&self) -> Result<Self::Part2> {
        // See https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
        //
        // Let P = thrown rock position vector and V = thrown rock velocity vector
        // Let P[i] = hail[i] position and V[i] = hail[i] velocity (known)
        // Let t[i] = time to intersect hail[i] (t == u, lines must intersect at same time)
        //
        // 3 i values allow us to come up with 6 linear equations which can be solved using matrix
        //   math
        //
        // My try at the linear matrices wasn't successful since it's been 15 years since I took
        // linear algebra, so I borrowed matrices from someone else's solution
        let p0 = self.hail[0].position.to_f64_vec();
        let v0 = self.hail[0].velocity.to_f64_vec();
        let p1 = self.hail[1].position.to_f64_vec();
        let v1 = self.hail[1].velocity.to_f64_vec();
        let p2 = self.hail[2].position.to_f64_vec();
        let v2 = self.hail[2].velocity.to_f64_vec();

        let a: Array2<f64> = array![
            [
                -(v0[1] - v1[1]),
                v0[0] - v1[0],
                0.0,
                p0[1] - p1[1],
                -(p0[0] - p1[0]),
                0.0
            ],
            [
                -(v0[1] - v2[1]),
                v0[0] - v2[0],
                0.0,
                p0[1] - p2[1],
                -(p0[0] - p2[0]),
                0.0
            ],
            [
                0.0,
                -(v0[2] - v1[2]),
                v0[1] - v1[1],
                0.0,
                p0[2] - p1[2],
                -(p0[1] - p1[1])
            ],
            [
                0.0,
                -(v0[2] - v2[2]),
                v0[1] - v2[1],
                0.0,
                p0[2] - p2[2],
                -(p0[1] - p2[1])
            ],
            [
                -(v0[2] - v1[2]),
                0.0,
                v0[0] - v1[0],
                p0[2] - p1[2],
                0.0,
                -(p0[0] - p1[0])
            ],
            [
                -(v0[2] - v2[2]),
                0.0,
                v0[0] - v2[0],
                p0[2] - p2[2],
                0.0,
                -(p0[0] - p2[0])
            ]
        ];

        let b: Array1<f64> = array![
            (p0[1] * v0[0] - p1[1] * v1[0]) - (p0[0] * v0[1] - p1[0] * v1[1]),
            (p0[1] * v0[0] - p2[1] * v2[0]) - (p0[0] * v0[1] - p2[0] * v2[1]),
            (p0[2] * v0[1] - p1[2] * v1[1]) - (p0[1] * v0[2] - p1[1] * v1[2]),
            (p0[2] * v0[1] - p2[2] * v2[1]) - (p0[1] * v0[2] - p2[1] * v2[2]),
            (p0[2] * v0[0] - p1[2] * v1[0]) - (p0[0] * v0[2] - p1[0] * v1[2]),
            (p0[2] * v0[0] - p2[2] * v2[0]) - (p0[0] * v0[2] - p2[0] * v2[2])
        ];

        let solution = a.solve_into(b)?;

        let thrown_stone = Hail {
            position: solution
                .slice(s![0..3])
                .into_iter()
                .map(|f| f.round() as usize)
                .collect::<Vec<usize>>()
                .try_into()?,
            velocity: solution
                .slice(s![3..6])
                .into_iter()
                .map(|f| f.round() as i64)
                .collect::<Vec<i64>>()
                .try_into()?,
        };

        Ok(thrown_stone.position.x + thrown_stone.position.y + thrown_stone.position.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.count_intersections(7.0, 27.0);

        assert_eq!(2, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(47, result);

        Ok(())
    }

    const SAMPLE: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
}
