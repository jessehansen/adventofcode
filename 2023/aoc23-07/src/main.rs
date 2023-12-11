use std::{cmp::Ordering, cmp::Ordering::*, collections::HashMap, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum CardEnum {
    Wild,
    Card(u32),
}
use CardEnum::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
use HandType::*;

impl FromStr for CardEnum {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents.chars().nth(0).ok_or_invalid()? {
            'A' => Ok(Card(14)),
            'K' => Ok(Card(13)),
            'Q' => Ok(Card(12)),
            'J' => Ok(Card(11)),
            'T' => Ok(Card(10)),
            n @ '0'..='9' => Ok(Card(n.to_digit(10).unwrap())),
            c => bail!("invalid card value {c}"),
        }
    }
}

#[derive(Eq, Clone, Debug)]
struct Hand {
    cards: Vec<CardEnum>,
    bid: usize,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Equal => {
                for (my_card, other_card) in self.cards.iter().zip(&other.cards) {
                    let ord = my_card.cmp(other_card);
                    if ord != Equal {
                        return ord;
                    }
                }
                Equal
            }
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

fn calc_hand_type(cards: &[CardEnum]) -> HandType {
    if cards.len() != 5 {
        panic!("Invalid cards");
    }
    let mut counts = HashMap::new();

    let mut wilds = 0;

    for card in cards {
        match card {
            Card(_) => {
                let count = counts.entry(*card).or_insert(0);
                *count += 1;
            }
            Wild => {
                wilds += 1;
            }
        }
    }

    let mut counts_by_highest: Vec<usize> = counts.values().copied().collect();
    counts_by_highest.sort_unstable_by(|a, b| b.cmp(a));

    if counts_by_highest.is_empty() {
        // must be all wilds
        return FiveOfAKind;
    }

    match counts_by_highest[0] + wilds {
        5 => FiveOfAKind,
        4 => FourOfAKind,
        3 => match counts_by_highest[1] {
            2 => FullHouse,
            _ => ThreeOfAKind,
        },
        2 => match counts_by_highest[1] {
            2 => TwoPair,
            _ => OnePair,
        },
        _ => HighCard,
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (cards, bid) = contents.split_once(' ').ok_or_invalid()?;

        let cards = cards.parse_chars()?;
        let bid = bid.parse_wrapped()?;
        let hand_type = calc_hand_type(&cards);

        Ok(Self {
            cards,
            bid,
            hand_type,
        })
    }
}

struct Problem {
    hands: Vec<Hand>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            hands: contents.parse_lines()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        self.hands.sort_unstable();
        Ok(self
            .hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bid)
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut hands = self.hands.clone();
        for hand in hands.iter_mut() {
            for card in hand.cards.iter_mut() {
                if card == &Card(11) {
                    *card = Wild;
                }
            }
            hand.hand_type = calc_hand_type(&hand.cards);
        }

        hands.sort_unstable();

        Ok(hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bid)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type() -> Result<()> {
        assert_eq!(FiveOfAKind, Hand::from_str("AAAAA 1")?.hand_type);
        assert_eq!(FourOfAKind, Hand::from_str("AA8AA 1")?.hand_type);
        assert_eq!(FullHouse, Hand::from_str("23332 1")?.hand_type);
        assert_eq!(ThreeOfAKind, Hand::from_str("TTT98 1")?.hand_type);
        assert_eq!(TwoPair, Hand::from_str("23432 1")?.hand_type);
        assert_eq!(OnePair, Hand::from_str("A23A4 1")?.hand_type);
        assert_eq!(HighCard, Hand::from_str("23456 1")?.hand_type);

        Ok(())
    }
    #[test]
    fn test_hand_sort() -> Result<()> {
        assert!(Wild < Card(2));
        assert!(Hand::from_str("JKKK2 1")? < Hand::from_str("QQQQ2 1")?);

        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(6440, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(5905, result);

        Ok(())
    }

    #[test]
    fn better_sample_part2() -> Result<()> {
        let problem = Problem::from_str(BETTER_SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(6839, result);

        Ok(())
    }

    const SAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const BETTER_SAMPLE: &str = "\
2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
}
