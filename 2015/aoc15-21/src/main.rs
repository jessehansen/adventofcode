use std::str::FromStr;

use anyhow::*;
use aoc_common::*;
use itertools::Itertools;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug, Clone)]
struct Character {
    hp: usize,
    damage: usize,
    armor: usize,
}

impl FromStr for Character {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (hp, contents) = contents.split_once('\n').ok_or_invalid()?;
        let (_, hp) = hp.split_once(": ").ok_or_invalid()?;
        let (damage, armor) = contents.split_once('\n').ok_or_invalid()?;
        let (_, damage) = damage.split_once(": ").ok_or_invalid()?;
        let (_, armor) = armor.split_once(": ").ok_or_invalid()?;
        Ok(Self {
            hp: hp.parse_wrapped()?,
            damage: damage.parse_wrapped()?,
            armor: armor.parse_wrapped()?,
        })
    }
}

impl Character {
    // self goes first
    fn wins_against(&self, other: &Self) -> bool {
        let damage = self.damage.saturating_sub(other.armor);
        if damage == 0 {
            return false;
        }
        let other_damage = other.damage.saturating_sub(self.armor);
        if other_damage == 0 {
            return true;
        }

        let my_turns = other.hp.div_ceil(damage);
        let their_turns = self.hp.div_ceil(other_damage);

        my_turns <= their_turns
    }
}

#[derive(Debug, Clone)]
struct Equipment {
    cost: usize,
    damage: usize,
    armor: usize,
}

struct Problem {
    boss: Character,
    weapons: Vec<Equipment>,
    armor: Vec<Equipment>,
    rings: Vec<Equipment>,

    max_cost: Option<usize>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            boss: contents.parse_wrapped()?,
            weapons: vec![
                Equipment {
                    cost: 8,
                    damage: 4,
                    armor: 0,
                },
                Equipment {
                    cost: 10,
                    damage: 5,
                    armor: 0,
                },
                Equipment {
                    cost: 25,
                    damage: 6,
                    armor: 0,
                },
                Equipment {
                    cost: 40,
                    damage: 7,
                    armor: 0,
                },
                Equipment {
                    cost: 74,
                    damage: 8,
                    armor: 0,
                },
            ],
            armor: vec![
                Equipment {
                    cost: 13,
                    damage: 0,
                    armor: 1,
                },
                Equipment {
                    cost: 31,
                    damage: 0,
                    armor: 2,
                },
                Equipment {
                    cost: 53,
                    damage: 0,
                    armor: 3,
                },
                Equipment {
                    cost: 75,
                    damage: 0,
                    armor: 4,
                },
                Equipment {
                    cost: 102,
                    damage: 0,
                    armor: 5,
                },
            ],
            rings: vec![
                Equipment {
                    cost: 25,
                    damage: 1,
                    armor: 0,
                },
                Equipment {
                    cost: 50,
                    damage: 2,
                    armor: 0,
                },
                Equipment {
                    cost: 100,
                    damage: 3,
                    armor: 0,
                },
                Equipment {
                    cost: 20,
                    damage: 0,
                    armor: 1,
                },
                Equipment {
                    cost: 40,
                    damage: 0,
                    armor: 2,
                },
                Equipment {
                    cost: 80,
                    damage: 0,
                    armor: 3,
                },
            ],
            max_cost: None,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let weapon_choices: Vec<usize> = (0..self.weapons.len()).collect();
        let mut armor_choices: Vec<Option<usize>> = (0..self.armor.len()).map(Some).collect();
        armor_choices.push(None);
        let mut ring_choices: Vec<(Option<usize>, Option<usize>)> = (0..self.rings.len())
            .cartesian_product(0..self.rings.len())
            .map(|(a, b)| {
                if a == b {
                    // add no ring option here
                    (None, None)
                } else {
                    (Some(a), Some(b))
                }
            })
            .collect();
        // add single ring purchases
        for ring_ix in 0..self.rings.len() {
            ring_choices.push((Some(ring_ix), None));
        }

        let mut min_cost = None;
        let mut max_cost = None;
        for weapon_ix in weapon_choices {
            for armor_ix in &armor_choices {
                for (ring1_ix, ring2_ix) in &ring_choices {
                    (min_cost, max_cost) = self.reduce_win_with_equipment(
                        min_cost, max_cost, weapon_ix, *armor_ix, *ring1_ix, *ring2_ix,
                    );
                }
            }
        }

        self.max_cost = max_cost;

        min_cost.ok_or_invalid()
    }

    fn part2(&self) -> Result<Self::Part2> {
        self.max_cost.ok_or_invalid()
    }
}

impl Problem {
    fn reduce_win_with_equipment(
        &mut self,
        min_cost_so_far: Option<usize>,
        max_cost_so_far: Option<usize>,
        weapon_ix: usize,
        armor_ix: Option<usize>,
        ring1_ix: Option<usize>,
        ring2_ix: Option<usize>,
    ) -> (Option<usize>, Option<usize>) {
        let weapon = &self.weapons[weapon_ix];
        let armor = armor_ix.map(|ix| &self.armor[ix]);
        let ring1 = ring1_ix.map(|ix| &self.rings[ix]);
        let ring2 = ring2_ix.map(|ix| &self.rings[ix]);

        let cost = weapon.cost
            + armor.map_or(0, |a| a.cost)
            + ring1.map_or(0, |r| r.cost)
            + ring2.map_or(0, |r| r.cost);

        let damage = weapon.damage
            + armor.map_or(0, |a| a.damage)
            + ring1.map_or(0, |r| r.damage)
            + ring2.map_or(0, |r| r.damage);

        let armor = weapon.armor
            + armor.map_or(0, |a| a.armor)
            + ring1.map_or(0, |r| r.armor)
            + ring2.map_or(0, |r| r.armor);

        let player = Character {
            hp: 100,
            damage,
            armor,
        };

        if player.wins_against(&self.boss) {
            (
                Some(min_cost_so_far.unwrap_or(usize::MAX).min(cost)),
                max_cost_so_far,
            )
        } else {
            (
                min_cost_so_far,
                Some(max_cost_so_far.unwrap_or(usize::MIN).max(cost)),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;
        let player = Character {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        let result = player.wins_against(&problem.boss);

        assert_eq!(true, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Hit Points: 12
Damage: 7
Armor: 2";
}
