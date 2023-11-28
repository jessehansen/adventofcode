use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

fn parse(contents: &str) -> Result<Vec<i32>> {
    contents
        .trim()
        .split(',')
        .map(|x| x.parse().context("invalid input"))
        .collect()
}

// this can be solved in the same way as part 2, but I thought it was interesting to leave in a
// naive solution
fn part1(fishes: &[i32]) -> Result<usize> {
    let mut fishes = fishes.to_owned();

    for _ in 0..80 {
        let mut new_fish = 0;
        for fish in &mut fishes {
            *fish -= 1;
            if *fish < 0 {
                new_fish += 1;
                *fish = 6;
            }
        }

        fishes.resize(fishes.len() + new_fish, 8);
    }

    Ok(fishes.len())
}

fn part2(fishes: &[i32]) -> Result<usize> {
    // breeders by day of week
    let mut breeders: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0];
    // new babies by day of week
    let mut babies: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0];
    for fish in fishes {
        breeders[*fish as usize] += 1;
    }

    for day in 0..256 {
        let day_of_week = day % 7;
        let baby_day = (day + 2) % 7;

        // babies who were born on this day last week will breed in 2 days
        breeders[baby_day] += babies[day_of_week];
        // new babies are born today
        babies[day_of_week] = breeders[day_of_week];
    }

    Ok(breeders.iter().sum::<usize>() + babies.iter().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 5934);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 26984457539);

        Ok(())
    }

    const SAMPLE: &str = "\
3,4,3,1,2
";
}
