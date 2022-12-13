use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    monkeys: Vec<Monkey>,
    divisors: Vec<u32>,
}

struct KeepAway {
    monkeys: Vec<Monkey>,
    divisors: Vec<u32>,
    worry: bool,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        let monkeys = parse_line_groups::<Monkey>(contents)?;
        let divisors: Vec<u32> = monkeys.iter().map(|x| x.decision.divisible_by).collect();

        Ok(Problem { monkeys, divisors })
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    decision: Decision,
    inspection_count: u32,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(monkey: &str) -> Result<Self> {
        let (_id, rest) = monkey.split_once('\n').ok_or(anyhow!("bad decision"))?;
        let (items, rest) = rest.split_once('\n').ok_or(anyhow!("bad decision"))?;
        let (prefix, items) = items.split_at(18);
        if prefix != "  Starting items: " {
            bail!("malformed monkey item list, prefix '{prefix}' not expected");
        }
        let (operation, decision) = rest.split_once('\n').ok_or(anyhow!("bad decision"))?;

        Ok(Monkey {
            items: items
                .split(", ")
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<Item>>(),
            operation: operation.parse()?,
            decision: decision.parse()?,
            inspection_count: 0,
        })
    }
}

#[derive(Clone, Debug)]
struct Item {
    worry_level: u32,
    monkey_remainders: Vec<u32>,
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(item: &str) -> Result<Self> {
        let worry_level = item.parse()?;

        Ok(Item {
            worry_level,
            monkey_remainders: vec![],
        })
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Plus(u32),
    Double,
    Times(u32),
    Square,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(operation: &str) -> Result<Self> {
        let (prefix, parts) = operation.split_at(23);
        if prefix != "  Operation: new = old " {
            bail!("malformed operation, prefix '{prefix}' not expected");
        }

        let (operation, operand) = parts.split_once(' ').ok_or(anyhow!("bad decision"))?;
        Ok(match operation {
            "+" if operand == "old" => Operation::Double,
            "+" => Operation::Plus(operand.parse()?),
            "*" if operand == "old" => Operation::Square,
            "*" => Operation::Times(operand.parse()?),
            _ => bail!("Invalid operation"),
        })
    }
}

impl Operation {
    fn execute(&self, old: u32) -> u32 {
        match self {
            Operation::Plus(value) => old + value,
            Operation::Double => old + old,
            Operation::Times(value) => old * value,
            Operation::Square => old * old,
        }
    }
}

#[derive(Clone)]
struct Decision {
    divisible_by: u32,
    if_true: usize,
    if_false: usize,
}

impl Decision {
    fn where_goes(&self, item: u32) -> usize {
        match item % self.divisible_by {
            0 => self.if_true,
            _ => self.if_false,
        }
    }
}

impl FromStr for Decision {
    type Err = Error;

    fn from_str(decision: &str) -> Result<Self> {
        let (test, outcomes) = decision.split_once('\n').ok_or(anyhow!("bad decision"))?;
        let (prefix, test) = test.split_at(21);
        if prefix != "  Test: divisible by " {
            bail!("malformed decision, prefix '{prefix}' not expected");
        }
        let (if_true, if_false) = outcomes.split_once('\n').ok_or(anyhow!("bad decision"))?;
        let (prefix, if_true) = if_true.split_at(29);
        if prefix != "    If true: throw to monkey " {
            bail!("malformed decision, prefix '{prefix}' not expected");
        }
        let (prefix, if_false) = if_false.split_at(30);
        if prefix != "    If false: throw to monkey " {
            bail!("malformed decision, prefix '{prefix}' not expected");
        }

        Ok(Decision {
            divisible_by: test.parse()?,
            if_true: if_true.parse()?,
            if_false: if_false.trim().parse()?,
        })
    }
}

impl KeepAway {
    fn round(&mut self) -> bool {
        for ix_monkey in 0..self.monkeys.len() {
            let mut throws: Vec<(usize, Item)> = std::vec::Vec::with_capacity(self.monkeys.len());

            let monkey = &mut self.monkeys[ix_monkey];
            for item in monkey.items.drain(..) {
                if !self.worry {
                    // part 1: divide worry level by 3 every throw
                    let new_worry = monkey.operation.execute(item.worry_level) / 3;
                    throws.push((
                        monkey.decision.where_goes(new_worry),
                        Item {
                            worry_level: new_worry,
                            monkey_remainders: vec![],
                        },
                    ));
                } else if false {
                    let new_worry = monkey.operation.execute(item.worry_level);
                    throws.push((
                        monkey.decision.where_goes(new_worry),
                        Item {
                            worry_level: new_worry,
                            monkey_remainders: vec![],
                        },
                    ));
                } else {
                    // part 2: keep track of the remainder for the worry level for each individual
                    // monkey. We don't ever care how big the actual worry level gets, we just need
                    // to know how to calculate where to throw it next
                    //
                    // This works because the remainder is updated in the same way as the whole
                    // number for all supported operations:
                    // 1. Plus: (n*x + r) + z = n*x + (r+z) (modding r results in new "n")
                    // 2. Double (2*(n*x+r)) = 2*n*x + 2*r (2*n*x is always divisible by x)
                    // 3. Multiply: z*(n*x+r) = z*n*x + z*r (z*n*x is always divisible by x)
                    // 4. Square: (n*x+r)^2 = n^2*x^2 + 2*n*x + r^2 (first two things are always
                    //    divisible by x)
                    let monkey_remainders: Vec<u32> = item
                        .monkey_remainders
                        .iter()
                        .enumerate()
                        .map(|(ix, r)| monkey.operation.execute(*r) % self.divisors[ix])
                        .collect();

                    throws.push((
                        monkey.decision.where_goes(monkey_remainders[ix_monkey]),
                        Item {
                            worry_level: 0,
                            monkey_remainders,
                        },
                    ));
                }
                monkey.inspection_count += 1;
            }

            for (catcher, item) in throws {
                self.monkeys[catcher].items.push(item);
            }
        }

        true
    }
}

impl Solution for Problem {
    type Part1 = u32;
    type Part2 = u64;

    fn part1(&mut self) -> Result<Self::Part1> {
        let mut game = KeepAway {
            monkeys: self.monkeys.clone(),
            divisors: self.divisors.clone(),
            worry: false,
        };
        for _ in 0..20 {
            game.round();
        }

        let mut inspections: Vec<u32> = game.monkeys.iter().map(|x| x.inspection_count).collect();
        inspections.sort();

        Ok(inspections[inspections.len() - 1] * inspections[inspections.len() - 2])
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut game = KeepAway {
            monkeys: self.monkeys.clone(),
            divisors: self.divisors.clone(),
            worry: true,
        };
        for monkey in &mut game.monkeys {
            for item in &mut monkey.items {
                item.monkey_remainders = game
                    .divisors
                    .iter()
                    .map(|div| item.worry_level % div)
                    .collect();
            }
        }
        for _ in 0..10_000 {
            game.round();
        }

        let mut inspections: Vec<u64> = game
            .monkeys
            .iter()
            .map(|x| x.inspection_count as u64)
            .collect();
        inspections.sort();

        Ok(inspections[inspections.len() - 1] * inspections[inspections.len() - 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(10605, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(2713310158, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
}
