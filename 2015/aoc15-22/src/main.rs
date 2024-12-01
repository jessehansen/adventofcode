use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Character {
    hp: usize,
    mana: usize,
    damage: usize,
    armor: usize,
}

impl FromStr for Character {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (hp, damage) = contents.split_once('\n').ok_or_invalid()?;
        let (_, hp) = hp.split_once(": ").ok_or_invalid()?;
        let (_, damage) = damage.split_once(": ").ok_or_invalid()?;
        Ok(Self {
            hp: hp.parse_wrapped()?,
            mana: 0,
            damage: damage.parse_wrapped()?,
            armor: 0,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}
use Spell::*;
static SPELLS: [Spell; 5] = [MagicMissile, Drain, Shield, Poison, Recharge];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Effect(Spell, usize);

impl std::fmt::Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicMissile => write!(f, "Magic Missile"),
            Drain => write!(f, "Drain"),
            Shield => write!(f, "Shield"),
            Poison => write!(f, "Poison"),
            Recharge => write!(f, "Recharge"),
        }
    }
}

impl Spell {
    fn cost(&self) -> usize {
        match self {
            MagicMissile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }

    fn damage(&self) -> usize {
        match self {
            MagicMissile => 4,
            Drain => 2,
            _ => 0,
        }
    }

    fn heal(&self) -> usize {
        match self {
            Drain => 2,
            _ => 0,
        }
    }

    fn armor(&self) -> usize {
        match self {
            Shield => 7,
            _ => 0,
        }
    }

    fn effect(&self) -> Option<Effect> {
        match self {
            Shield => Some(Effect(Shield, 6)),
            Poison => Some(Effect(Poison, 6)),
            Recharge => Some(Effect(Recharge, 5)),
            _ => None,
        }
    }

    fn print(&self) {
        match self {
            MagicMissile => println!("Player casts Magic Missile, dealing 4 damage."),
            Drain => println!("Player casts Drain, dealing 2 damage, and healing 2 hit points."),
            x => println!("Player casts {x}."),
        }
    }
}

impl Effect {
    fn apply(&self, player: &mut Character, boss: &mut Character) -> Option<Effect> {
        match self.0 {
            Poison => {
                // println!("Poison deals 3 damage; its timer is now {0}.", self.1 - 1);
                boss.hp = boss.hp.saturating_sub(3);
            }
            Recharge => {
                // println!(
                //     "Recharge provides 101 mana; its timer is now {0}.",
                //     self.1 - 1
                // );
                player.mana += 101
            }
            _ => {
                // println!("{0}'s timer is now {1}.", self.0, self.1 - 1);
            }
        }
        match self.1 - 1 {
            0 => {
                // println!("{0} wears off.", self.0);
                match self.0 {
                    Shield => player.armor = player.armor.saturating_sub(7),
                    _ => {}
                }
                None
            }
            _ => Some(Effect(self.0, self.1 - 1)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum State {
    Continue {
        mana_spent: usize,
        player: Character,
        boss: Character,
        effects: Vec<Effect>,
    },
    PlayerWin {
        mana_spent: usize,
    },
    BossWin {
        mana_spent: usize,
    },
}
use State::*;

fn process_effects(
    effects: Vec<Effect>,
    player: &mut Character,
    boss: &mut Character,
) -> Vec<Effect> {
    effects
        .into_iter()
        .filter_map(|effect| effect.apply(player, boss))
        .collect()
}

impl State {
    fn turn(&self, spell: &Spell) -> State {
        match self {
            Continue {
                mana_spent,
                player,
                boss,
                effects,
            } => {
                let mut player = player.clone();
                let mut boss = boss.clone();

                /*
                println!("-- Player turn --");
                println!(
                    "- Player has {0} hit points, {1} armor, {2} mana",
                    player.hp, player.armor, player.mana
                );
                println!("- Boss has {0} hit points", boss.hp);
                */

                // player turn
                let mut effects = process_effects(effects.to_owned(), &mut player, &mut boss);
                if boss.hp == 0 {
                    println!("WIN: {mana_spent}");
                    return PlayerWin {
                        mana_spent: *mana_spent,
                    };
                }
                let cost = spell.cost();
                if player.mana < cost {
                    panic!("not enough mana");
                }
                let mana_spent = mana_spent + cost;
                player.mana -= cost;

                /*
                spell.print();
                println!();
                */

                boss.hp = boss.hp.saturating_sub(spell.damage());
                player.hp += spell.heal();
                player.armor += spell.armor();
                if let Some(effect) = spell.effect() {
                    effects.push(effect);
                }

                if boss.hp == 0 {
                    println!("WIN: {mana_spent}");
                    return PlayerWin { mana_spent };
                }

                /*
                println!("-- Boss turn --");
                println!(
                    "- Player has {0} hit points, {1} armor, {2} mana",
                    player.hp, player.armor, player.mana
                );
                println!("- Boss has {0} hit points", boss.hp);
                */

                let effects = process_effects(effects, &mut player, &mut boss);
                if boss.hp == 0 {
                    println!("WIN: {mana_spent}");
                    return PlayerWin { mana_spent };
                }
                let mut damage = boss.damage.saturating_sub(player.armor);
                if damage == 0 {
                    damage = 1;
                }
                /*
                println!("Boss attacks for {damage} damage!");
                println!();
                */
                player.hp = player.hp.saturating_sub(damage);
                if player.hp == 0 {
                    println!("LOSS: {mana_spent}");
                    return BossWin { mana_spent };
                }
                return Continue {
                    mana_spent,
                    player,
                    boss,
                    effects,
                };
            }
            _ => panic!("game over"),
        }
    }
}

impl OptimizationState for State {
    type Score = usize;
    type CacheKey = State;

    fn score(&self) -> Self::Score {
        match self {
            &PlayerWin { mana_spent } => mana_spent,
            &BossWin { mana_spent } => mana_spent,
            &Continue { mana_spent, .. } => mana_spent,
        }
    }

    fn cache_key(&self) -> Self::CacheKey {
        self.clone()
    }
}

struct Problem {
    boss: Character,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            boss: contents.parse_wrapped()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let best_win = dijkstra(
            Continue {
                mana_spent: 0,
                player: Character {
                    hp: 50,
                    mana: 500,
                    damage: 0,
                    armor: 0,
                },
                boss: self.boss.clone(),
                effects: vec![],
            },
            |state| match state {
                Continue { player, .. } => SPELLS
                    .iter()
                    .filter(|spell| {
                        let cost = spell.cost();
                        player.mana >= cost
                    })
                    .map(|spell| state.turn(spell))
                    .collect::<Vec<_>>(),
                _ => vec![],
            },
            |state| match state {
                PlayerWin { .. } => true,
                _ => false,
            },
        );

        best_win
            .map(|state| state.score())
            .ok_or_else(|| anyhow!("no solution"))
    }

    fn part2(&self) -> Result<Self::Part2> {
        bail!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let state = Continue {
            mana_spent: 0,
            player: Character {
                hp: 10,
                mana: 250,
                damage: 0,
                armor: 0,
            },
            boss: Character {
                hp: 13,
                mana: 0,
                damage: 8,
                armor: 0,
            },
            effects: vec![],
        };

        let state = state.turn(&Poison);

        assert_eq!(
            state,
            Continue {
                mana_spent: 173,
                player: Character {
                    hp: 2,
                    mana: 77,
                    damage: 0,
                    armor: 0,
                },
                boss: Character {
                    hp: 10,
                    mana: 0,
                    damage: 8,
                    armor: 0,
                },
                effects: vec![Effect(Poison, 5)],
            }
        );

        let state = state.turn(&MagicMissile);

        assert_eq!(state, PlayerWin { mana_spent: 226 });

        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let state = Continue {
            mana_spent: 0,
            player: Character {
                hp: 10,
                mana: 250,
                damage: 0,
                armor: 0,
            },
            boss: Character {
                hp: 14,
                mana: 0,
                damage: 8,
                armor: 0,
            },
            effects: vec![],
        };

        let state = state.turn(&Recharge);
        let state = state.turn(&Shield);
        let state = state.turn(&Drain);
        let state = state.turn(&Poison);
        let state = state.turn(&MagicMissile);

        assert_eq!(state, PlayerWin { mana_spent: 641 });

        Ok(())
    }
}
