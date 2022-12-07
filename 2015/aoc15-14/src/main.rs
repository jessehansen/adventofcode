use anyhow::*;
use aoc_common::*;
use std::collections::VecDeque;
use std::str::FromStr;

fn main() -> Result<()> {
    run_vec(parse_lines, part1, part2)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn race(&self, secs: u32) -> u32 {
        let mut segments = VecDeque::from([(self.fly_time, self.speed), (self.rest_time, 0)]);
        let mut remaining_time = secs;
        let mut dist = 0;
        while remaining_time > segments[0].0 {
            let (duration, speed) = segments[0];
            dist += duration * speed;
            remaining_time -= duration;
            segments.rotate_left(1);
        }
        dist + remaining_time * segments[0].1
    }
}

impl FromStr for Reindeer {
    type Err = Error;

    fn from_str(reindeer: &str) -> Result<Self> {
        let parts: Vec<&str> = reindeer.split_whitespace().collect();
        Ok(Reindeer {
            name: parts[0].to_string(),
            speed: parts[3].parse().unwrap(),
            fly_time: parts[6].parse().unwrap(),
            rest_time: parts[13].parse().unwrap(),
        })
    }
}

enum ReindeerStatus {
    Flying(u32),
    Resting(u32),
}

struct RacingReindeer {
    deer: Reindeer,
    status: ReindeerStatus,
    distance: u32,
    score: u32,
}

impl RacingReindeer {
    fn new(deer: Reindeer) -> RacingReindeer {
        let status = ReindeerStatus::Flying(deer.fly_time);
        RacingReindeer {
            deer,
            status,
            distance: 0,
            score: 0,
        }
    }

    fn tick(&mut self) {
        match self.status {
            ReindeerStatus::Flying(duration) => {
                self.distance += self.deer.speed;
                self.status = if duration > 1 {
                    ReindeerStatus::Flying(duration - 1)
                } else {
                    ReindeerStatus::Resting(self.deer.rest_time)
                }
            }
            ReindeerStatus::Resting(duration) => {
                self.status = if duration > 1 {
                    ReindeerStatus::Resting(duration - 1)
                } else {
                    ReindeerStatus::Flying(self.deer.fly_time)
                }
            }
        }
    }
}

fn part1(reindeer: &[Reindeer]) -> Result<u32> {
    reindeer
        .iter()
        .map(|x| x.race(2503))
        .max()
        .ok_or_else(|| anyhow!("no max result"))
}

fn race_new_score(reindeer: &[Reindeer], secs: u32) -> u32 {
    let mut racing_deer: Vec<RacingReindeer> = reindeer
        .iter()
        .map(|x| RacingReindeer::new(x.clone()))
        .collect();

    for _ in 0..secs {
        racing_deer.iter_mut().for_each(|x| x.tick());
        let lead_distance = racing_deer.iter().map(|x| x.distance).max().unwrap();
        racing_deer
            .iter_mut()
            .filter(|x| x.distance == lead_distance)
            .for_each(|x| x.score += 1);
    }

    racing_deer.iter().map(|x| x.score).max().unwrap()
}

fn part2(reindeer: &[Reindeer]) -> Result<u32> {
    Ok(race_new_score(reindeer, 2503))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_race() -> Result<()> {
        let comet = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        let dancer = Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        assert_eq!(comet.race(1), 14);
        assert_eq!(dancer.race(1), 16);
        assert_eq!(comet.race(1000), 1120);
        assert_eq!(dancer.race(1000), 1056);

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let comet = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        let dancer = Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        assert_eq!(race_new_score(&[comet, dancer], 1000), 689);

        Ok(())
    }
}
