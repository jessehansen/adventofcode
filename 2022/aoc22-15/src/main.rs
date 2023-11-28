use std::cmp::{max, min};
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

trait RangeExtensions {
    fn overlaps(&self, other: &Self) -> bool;
    fn containing_range(&self, other: &Self) -> Self;
    fn len(&self) -> usize;
}

impl RangeExtensions for RangeInclusive<i32> {
    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }

    fn containing_range(&self, other: &Self) -> Self {
        min(*self.start(), *other.start())..=max(*self.end(), *other.end())
    }

    fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            ((self.end() - self.start()) as usize) + 1
        }
    }
}

#[derive(Debug)]
struct RowCoverage {
    ranges: Vec<RangeInclusive<i32>>,
}

#[allow(clippy::reversed_empty_ranges)] // need an empty range on purpose
const EMPTY_RANGE: RangeInclusive<i32> = 1..=0;

impl RowCoverage {
    fn include(&mut self, range: RangeInclusive<i32>) {
        if range.is_empty() {
            return;
        }

        let mut new_range = range;
        let mut output_ranges = vec![];

        for existing_range in self.ranges.drain(..) {
            if new_range.is_empty() {
                // we've already added the new range, just keep appending the existing ranges
                output_ranges.push(existing_range);
            } else if new_range.overlaps(&existing_range) {
                // generate the union of the two ranges
                new_range = new_range.containing_range(&existing_range)
            } else if new_range.start() < existing_range.start() {
                output_ranges.push(new_range);
                new_range = EMPTY_RANGE.clone();
                output_ranges.push(existing_range);
            } else {
                output_ranges.push(existing_range);
            }
        }
        if !new_range.is_empty() {
            output_ranges.push(new_range);
        }
        self.ranges = output_ranges;
    }

    fn first_possible_beacon_in_row(&self, from: i32, to: i32) -> Option<i32> {
        let mut last_end = i32::MIN;
        for range in &self.ranges {
            if range.end() < &from {
                continue;
            }
            if last_end >= from && range.start() > &(last_end + 1) {
                return Some(last_end + 1);
            }
            if range.start() > &to {
                break;
            }
            last_end = *range.end();
        }

        None
    }
}

impl FromIterator<RangeInclusive<i32>> for RowCoverage {
    fn from_iter<I: IntoIterator<Item = RangeInclusive<i32>>>(iter: I) -> Self {
        let mut coverage = RowCoverage { ranges: vec![] };

        for range in iter {
            coverage.include(range);
        }

        coverage
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Sensor {
    location: IPoint2D,
    beacon: IPoint2D,
}

struct ProblemPoint(IPoint2D);

impl FromStr for ProblemPoint {
    type Err = Error;

    fn from_str(contents: &str) -> Result<ProblemPoint> {
        // x={}, y={}
        let (x, y) = grab_2(contents, ['=', ',', ' '], 1, 4)?;
        Ok(ProblemPoint(IPoint2D { x, y }))
    }
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Sensor> {
        // Sensor at {}: closest beacon is at x=-2, y=15
        let (prefix, rest) = contents.split_at(10);
        if prefix != "Sensor at " {
            bail!("invalid prefix '{prefix}'");
        }

        if let Some((loc, rest)) = rest.split_once(':') {
            let (prefix, dist) = rest.split_at(22);
            if prefix != " closest beacon is at " {
                bail!("Invalid prefix '{prefix}'")
            }
            Ok(Sensor {
                location: loc.parse::<ProblemPoint>()?.0,
                beacon: dist.parse::<ProblemPoint>()?.0,
            })
        } else {
            bail!("no distance");
        }
    }
}

impl Sensor {
    fn no_beacon_range_in_row(&self, y: i32, from_x: i32, to_x: i32) -> RangeInclusive<i32> {
        let dist = self.beacon.manhattan_distance(self.location);
        let distance_in_row = dist - (self.location.y - y).abs();

        if distance_in_row >= 0 {
            let mut min_x = self.location.x - distance_in_row;
            let mut max_x = self.location.x + distance_in_row;
            // if the beacon is on row y, then it is either at min_x or max_x,
            // remove it from the range
            if self.beacon.y == y {
                if self.beacon.x == min_x {
                    min_x += 1;
                } else {
                    max_x -= 1;
                }
            }
            min(to_x, min_x)..=max(max_x, from_x)
        } else {
            EMPTY_RANGE.clone() // empty range
        }
    }
}

struct Problem {
    sensors: Vec<Sensor>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            sensors: parse_lines(contents)?,
        })
    }
}

impl Problem {
    fn points_that_cannot_be_beacons_in_row(&self, y: i32) -> usize {
        let coverage = self
            .sensors
            .iter()
            .map(|s| s.no_beacon_range_in_row(y, i32::MIN, i32::MAX))
            .collect::<RowCoverage>();

        coverage.ranges.iter().map(|x| x.len()).sum()
    }

    fn tuning_frequency_for_missing_beacon_in_search_space(
        &self,
        search_boundary: IPoint2D,
    ) -> Option<i64> {
        for y in 0..search_boundary.y {
            let coverage = self
                .sensors
                .iter()
                .map(|s| s.no_beacon_range_in_row(y, 0, search_boundary.x))
                .chain(self.sensors.iter().filter_map(
                    |Sensor {
                         location: _,
                         beacon:
                             IPoint2D {
                                 x: beacon_x,
                                 y: beacon_y,
                             },
                     }| {
                        if *beacon_y == y {
                            Some(*beacon_x..=*beacon_x)
                        } else {
                            None
                        }
                    },
                ))
                .collect::<RowCoverage>();

            match coverage.first_possible_beacon_in_row(0, search_boundary.y) {
                None => {}
                Some(x) => {
                    return Some((x as i64) * 4000000 + (y as i64));
                }
            }
        }

        None
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = i64;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.points_that_cannot_be_beacons_in_row(2_000_000))
    }

    fn part2(&self) -> Result<Self::Part2> {
        self.tuning_frequency_for_missing_beacon_in_search_space(ipt(4_000_000, 4_000_000))
            .ok_or(anyhow!("no beacon found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_no_beacon_range_in_row() -> Result<()> {
        assert_eq!(
            3..=14,
            Sensor {
                location: ipt(8, 7),
                beacon: ipt(2, 10)
            }
            .no_beacon_range_in_row(10, -100, 100)
        );
        assert!(Sensor {
            location: ipt(8, 7),
            beacon: ipt(2, 10)
        }
        .no_beacon_range_in_row(100, -100, 100)
        .is_empty());

        Ok(())
    }

    #[test]
    fn test_row_coverage() -> Result<()> {
        let ranges = vec![
            -152904..=180544,
            -1432930..=2183490,
            1546506..=1680294,
            2846248..=3765680,
            2536648..=2536648,
            3643386..=4234952,
            2552965..=3031791,
            2558169..=2670881,
            1940105..=2429293,
            2230818..=2864704,
            3329976..=4093060,
        ];

        let coverage = RowCoverage::from_iter(ranges);

        assert_eq!(coverage.ranges.len(), 1);
        assert_eq!(coverage.ranges[0], -1432930..=4234952);

        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.points_that_cannot_be_beacons_in_row(10);

        assert_eq!(26, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.tuning_frequency_for_missing_beacon_in_search_space(ipt(20, 20));

        assert_eq!(Some(56000011), result);

        Ok(())
    }

    const SAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
}
