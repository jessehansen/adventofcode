use aoc_common::run;

fn main() {
    run(parse, part1, part2);
}

fn parse(contents: &str) -> Vec<i32> {
    contents
        .trim()
        .split(",")
        .into_iter()
        .map(|x| x.parse().expect("invalid input"))
        .collect()
}

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

// this can be solved in the same way as part 2, but I thought it was interesting to leave in a
// naive solution
fn part1(fishes: &Vec<i32>) -> String {
    let mut fishes = fishes.clone();

    for _ in times(80) {
        let mut new_fish = 0;
        for fish in &mut fishes {
            *fish -= 1;
            if *fish < 0 {
                new_fish += 1;
                *fish = 6;
            }
        }

        for _ in times(new_fish) {
            fishes.push(8);
        }
    }

    format!("{}", fishes.len())
}

fn part2(fishes: &Vec<i32>) -> String {
    // breeders by day of week
    let mut breeders: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0];
    // new babies by day of week
    let mut babies: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0];
    for fish in fishes {
        breeders[*fish as usize] += 1;
    }

    let mut day = 0;
    for _ in times(256) {
        let day_of_week = day % 7;
        let baby_day = (day + 2) % 7;

        // babies who were born on this day last week will breed in 2 days
        breeders[baby_day] += babies[day_of_week];
        // new babies are born today
        babies[day_of_week] = breeders[day_of_week];

        day += 1;
    }

    let pop: usize = breeders.iter().sum::<usize>() + babies.iter().sum::<usize>();

    format!("{}", pop)
}
