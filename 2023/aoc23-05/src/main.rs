use std::{ops::Range, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

fn parse_seeds(seeds: &str) -> Result<Vec<usize>> {
    let (label, seeds) = seeds.split_once(": ").ok_or_invalid()?;
    if label != "seeds" {
        bail!("invalid seeds label '{label}'");
    }
    parse_split(seeds, ' ')
}

struct GardenMap {
    rules: Vec<MapRule>,
}

impl FromStr for GardenMap {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let (_, map) = map.split_once('\n').ok_or_invalid()?;
        Ok(GardenMap {
            rules: parse_lines(map)?,
        })
    }
}

impl GardenMap {
    fn dest(&self, source: &usize) -> usize {
        for rule in &self.rules {
            if rule.source_range.contains(source) {
                return rule.dest_range.start + (source - rule.source_range.start);
            }
        }
        *source
    }
}

struct MapRule {
    source_range: Range<usize>,
    dest_range: Range<usize>,
}

impl FromStr for MapRule {
    type Err = Error;

    fn from_str(rule: &str) -> Result<Self> {
        let mut parts = rule.split(' ');
        let dest_start = parts.next().ok_or_invalid()?.parse_wrapped()?;
        let source_start = parts.next().ok_or_invalid()?.parse_wrapped()?;
        let range_len: usize = parts.next().ok_or_invalid()?.parse_wrapped()?;
        Ok(MapRule {
            source_range: source_start..(source_start + range_len),
            dest_range: dest_start..(dest_start + range_len),
        })
    }
}

struct Problem {
    seeds: Vec<usize>,
    seed_to_soil_map: GardenMap,
    soil_to_fertilizer_map: GardenMap,
    fertilizer_to_water_map: GardenMap,
    water_to_light_map: GardenMap,
    light_to_temparature_map: GardenMap,
    temperature_to_humidity_map: GardenMap,
    humitity_to_location_map: GardenMap,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut groups = contents.split("\n\n");
        Ok(Self {
            seeds: parse_seeds(groups.next().ok_or_invalid()?)?,
            seed_to_soil_map: groups.next().ok_or_invalid()?.parse_wrapped()?,
            soil_to_fertilizer_map: groups.next().ok_or_invalid()?.parse_wrapped()?,
            fertilizer_to_water_map: groups.next().ok_or_invalid()?.parse_wrapped()?,
            water_to_light_map: groups.next().ok_or_invalid()?.parse_wrapped()?,
            light_to_temparature_map: groups.next().ok_or_invalid()?.parse_wrapped()?,
            temperature_to_humidity_map: groups.next().ok_or_invalid()?.parse_wrapped()?,
            humitity_to_location_map: groups.next().ok_or_invalid()?.parse_wrapped()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        self.lowest_location(self.seeds.iter().copied())
    }

    fn part2(&self) -> Result<Self::Part2> {
        self.lowest_location(self.seeds.chunks(2).flat_map(|pairs| {
            let start = pairs[0];
            let len = pairs[1];
            start..(start + len)
        }))
    }
}

impl Problem {
    fn lowest_location<T: Iterator<Item = usize>>(&self, seeds: T) -> Result<usize> {
        seeds
            .map(|seed| {
                let soil = self.seed_to_soil_map.dest(&seed);
                let fertilizer = self.soil_to_fertilizer_map.dest(&soil);
                let water = self.fertilizer_to_water_map.dest(&fertilizer);
                let light = self.water_to_light_map.dest(&water);
                let temp = self.light_to_temparature_map.dest(&light);
                let humidity = self.temperature_to_humidity_map.dest(&temp);
                self.humitity_to_location_map.dest(&humidity)
            })
            .min()
            .ok_or_invalid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(35, result);

        Ok(())
    }

    #[test]
    fn test_seed_maps() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(81, problem.seed_to_soil_map.dest(&79));
        assert_eq!(14, problem.seed_to_soil_map.dest(&14));
        assert_eq!(57, problem.seed_to_soil_map.dest(&55));
        assert_eq!(13, problem.seed_to_soil_map.dest(&13));

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(46, result);

        Ok(())
    }

    const SAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
}
