use aoc_common::*;
use std::cmp::max;
use std::str::FromStr;

fn main() {
    run_vec(parse_lines, part1, part2);
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(ingredient: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = ingredient
            .split(&[' ', ':', ','])
            .filter(|x| !x.is_empty())
            .collect();
        Ok(Ingredient {
            capacity: parts[2].parse().unwrap(),
            durability: parts[4].parse().unwrap(),
            flavor: parts[6].parse().unwrap(),
            texture: parts[8].parse().unwrap(),
            calories: parts[10].parse().unwrap(),
        })
    }
}

fn total_score<'a, T>(ingredients: T, set_calories: bool) -> i32
where
    T: IntoIterator<Item = (i32, &'a Ingredient)>,
{
    let mut totals = Ingredient {
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };

    for (tsps, ingredient) in ingredients {
        totals.capacity += tsps * ingredient.capacity;
        totals.durability += tsps * ingredient.durability;
        totals.flavor += tsps * ingredient.flavor;
        totals.texture += tsps * ingredient.texture;
        totals.calories += tsps * ingredient.calories;
    }

    if totals.capacity < 0
        || totals.durability < 0
        || totals.flavor < 0
        || totals.texture < 0
        || (set_calories && totals.calories != 500)
    {
        0
    } else {
        max(
            totals.capacity * totals.durability * totals.flavor * totals.texture,
            0,
        )
    }
}

fn two_ingredient_permutations() -> Vec<Vec<i32>> {
    let mut result = vec![];
    for x in 1..=99 {
        result.push(vec![x, 100 - x]);
    }
    result
}

// returns all the permutations for a given set of ingredients
// ingredients must add to 100 tsps and each ingredient must have at least 1 tsp
fn four_ingredient_permutations() -> Vec<Vec<i32>> {
    let mut result = vec![];
    for x in 1..=97 {
        for y in 1..=(98 - x) {
            for z in 1..=(99 - x - y) {
                result.push(vec![x, y, z, 100 - x - y - z]);
            }
        }
    }
    result
}

fn get_best_cookie(ingredients: &[Ingredient], set_calories: bool) -> i32 {
    let permutations = match ingredients.len() {
        2 => two_ingredient_permutations(),
        4 => four_ingredient_permutations(),
        _ => panic!(),
    };
    let mut max_score = 0;
    for p in permutations {
        max_score = max(
            total_score(p.to_owned().into_iter().zip(ingredients), set_calories),
            max_score,
        );
    }
    max_score
}

fn part1(ingredients: &[Ingredient]) -> i32 {
    get_best_cookie(ingredients, false)
}

fn part2(ingredients: &[Ingredient]) -> i32 {
    get_best_cookie(ingredients, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_score_test() {
        let sample_ingredients = parse_lines(SAMPLE);
        let butterscotch = &sample_ingredients[0];
        let cinnamon = &sample_ingredients[1];

        assert_eq!(
            total_score([(44, butterscotch), (56, cinnamon)], false),
            62842880
        );
    }

    #[test]
    fn sample_part1() {
        assert_eq!(part1(&parse_lines(SAMPLE)), 62842880);
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(&parse_lines(SAMPLE)), 57600000);
    }

    const SAMPLE: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
}
