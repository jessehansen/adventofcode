use aoc_common::*;
use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

fn main() {
    run(parse, part1, part2);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Cell {
    Wall,
    Hall,
    Entry,
    Destination(AmphipodColor),
    Void,
}

impl Cell {
    fn is_destination(&self) -> bool {
        matches!(self, Destination(_))
    }
}

use Cell::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum AmphipodColor {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodColor {
    fn from_char(c: char) -> AmphipodColor {
        match c {
            'A' => AmphipodColor::Amber,
            'B' => AmphipodColor::Bronze,
            'C' => AmphipodColor::Copper,
            'D' => AmphipodColor::Desert,
            _ => panic!(),
        }
    }

    fn next_destination(&self) -> AmphipodColor {
        match self {
            AmphipodColor::Amber => AmphipodColor::Bronze,
            AmphipodColor::Bronze => AmphipodColor::Copper,
            AmphipodColor::Copper => AmphipodColor::Desert,
            AmphipodColor::Desert => AmphipodColor::Amber,
        }
    }

    fn energy_per_step(&self) -> u32 {
        match self {
            AmphipodColor::Amber => 1,
            AmphipodColor::Bronze => 10,
            AmphipodColor::Copper => 100,
            AmphipodColor::Desert => 1000,
        }
    }
}

impl fmt::Display for AmphipodColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AmphipodColor::Amber => "A",
                AmphipodColor::Bronze => "B",
                AmphipodColor::Copper => "C",
                AmphipodColor::Desert => "D",
            }
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Amphipod {
    location: Point2D,
    color: AmphipodColor,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct BurrowMap {
    map: Grid2D<Cell>,
    pop: Vec<Amphipod>,
    energy_used: u32,
    moves_made: Vec<(Point2D, Point2D, u32)>,
}

impl BurrowMap {
    fn moves(&self) -> Vec<BurrowMap> {
        let mut moves = vec![];
        for a in self.pop.iter().filter(|a| {
            // an amphipod can move if it is either in the hallway, or one step away from an
            // entry, or the space above it is empty
            if self.map[a.location] == Hall {
                true
            } else if let Some(pt) = a.location.up() {
                self.map[pt] == Entry || !self.pop.iter().any(|x| x.location == pt)
            } else {
                false
            }
        }) {
            for (target, steps) in self.valid_moves(a) {
                moves.push(self.move_amphipod(a, target, steps));
            }
        }
        moves
    }

    fn valid_moves<'a>(&'a self, a: &'a Amphipod) -> impl Iterator<Item = (Point2D, u32)> + 'a {
        self.map
            .iter_horizontal()
            .filter_map(|(pt, cell)| match cell {
                Hall => self.steps_to(&a.location, &pt).map(|steps| (pt, steps)),
                Destination(c) if a.color == *c => {
                    if let Some(steps) = self.steps_to(&a.location, &pt) {
                        // only move into a destination if we're not screwing up the sort
                        let next_down = pt.down(self.map.bounds.height).unwrap();
                        if self.map[next_down] == Wall
                            || self
                                .pop
                                .iter()
                                .any(|x| x.location == next_down && x.color == *c)
                        {
                            Some((pt, steps))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }

    fn steps_to(&self, start: &Point2D, destination: &Point2D) -> Option<u32> {
        let mut cur = *start;
        let mut steps = 0;
        // have to move in the hallway before moving left and right
        while cur.y > 1 {
            match cur.up() {
                Some(next) if self.pop.iter().any(|x| x.location == next) => {
                    return None;
                }
                Some(next) => {
                    cur = next;
                    steps += 1
                }
                _ => panic!(),
            }
        }
        while cur.x > destination.x {
            match cur.left() {
                Some(next) if self.pop.iter().any(|x| x.location == next) => {
                    return None;
                }
                Some(next) => {
                    cur = next;
                    steps += 1
                }
                _ => panic!(),
            }
        }
        while cur.x < destination.x {
            match cur.right(self.map.bounds.width) {
                Some(next) if self.pop.iter().any(|x| x.location == next) => {
                    return None;
                }
                Some(next) => {
                    cur = next;
                    steps += 1
                }
                _ => panic!(),
            }
        }
        while cur.y < destination.y {
            match cur.down(self.map.bounds.height) {
                Some(next) if self.pop.iter().any(|x| x.location == next) => {
                    return None;
                }
                Some(next) => {
                    cur = next;
                    steps += 1
                }
                _ => panic!(),
            }
        }

        Some(steps)
    }

    fn is_sorted(&self) -> bool {
        self.pop
            .iter()
            .all(|x| self.map[x.location] == Destination(x.color))
    }

    fn move_amphipod(&self, a: &Amphipod, target_location: Point2D, steps: u32) -> BurrowMap {
        let mut result_pop = self.pop.clone();
        result_pop.iter_mut().for_each(|b| {
            if b.location == a.location {
                b.location = target_location;
            }
        });
        let mut moves_made = self.moves_made.clone();
        moves_made.push((
            a.location,
            target_location,
            steps * a.color.energy_per_step(),
        ));
        BurrowMap {
            map: self.map.clone(),
            pop: result_pop,
            energy_used: self.energy_used + steps * a.color.energy_per_step(),
            moves_made,
        }
    }

    fn amphipod_hash(&self) -> String {
        let mut amphipods: Vec<String> = self
            .pop
            .iter()
            .map(|a| format!("{}:{}", a.color, a.location))
            .collect();
        amphipods.sort();

        amphipods.join(";")
    }
}

// for BinaryHeap priority queue
impl Ord for BurrowMap {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .energy_used
            .cmp(&self.energy_used)
            .then_with(|| self.pop.cmp(&other.pop))
    }
}

impl PartialOrd for BurrowMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for BurrowMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Grid2D::new_constant(self.map.bounds, " ");
        self.map.iter_horizontal().for_each(|(pt, x)| {
            result[pt] = match x {
                Wall => "#",
                _ => ".",
            }
        });

        for a in &self.pop {
            result[a.location] = match a.color {
                AmphipodColor::Amber => "A",
                AmphipodColor::Bronze => "B",
                AmphipodColor::Copper => "C",
                AmphipodColor::Desert => "D",
            }
        }

        let mm: Vec<String> = self
            .moves_made
            .iter()
            .map(|(from, to, cost)| format!("{}->{} ({})", from, to, cost))
            .collect();
        write!(
            f,
            "map:\n{}\nenergy_used: {}\nmoves_made: {}",
            result,
            self.energy_used,
            mm.join("; ")
        )
    }
}

fn parse(contents: &str) -> BurrowMap {
    let lines: Vec<&str> = contents.lines().collect();
    let mut map = Grid2D::new_constant(
        Bounds2D {
            width: lines[0].len(),
            height: lines.len(),
        },
        Cell::Void,
    );
    let mut pop = vec![];
    for (y, line) in lines.iter().enumerate() {
        let mut destination = AmphipodColor::Amber;
        for (x, char) in line.chars().enumerate() {
            let location = pt(x, y);
            map[location] = match char {
                '#' => Wall,
                '.' if y + 1 < lines.len() && lines[y + 1].chars().nth(x) == Some('#') => Hall,
                '.' => Entry,
                ' ' => Void,
                class => {
                    let result = Destination(destination);
                    destination = destination.next_destination();

                    pop.push(Amphipod {
                        location,
                        color: AmphipodColor::from_char(class),
                    });
                    result
                }
            }
        }
    }

    BurrowMap {
        map,
        pop,
        energy_used: 0,
        moves_made: vec![],
    }
}

fn easiest_sort(start: &BurrowMap) -> u32 {
    let mut dist = HashMap::<String, u32>::new();
    let mut heap = BinaryHeap::new();
    heap.push(start.clone());

    // using a binary heap means we're always looking at the next best unvisited node
    while let Some(state) = heap.pop() {
        dbg!(&state);
        dbg!(&state.is_sorted());
        dbg!(dist.get(&state.amphipod_hash()));
        if state.is_sorted() {
            return state.energy_used;
        }

        // if we're already above the previous best to this point, don't bother continuing
        match dist.get(&state.amphipod_hash()) {
            Some(prev_dist) if state.energy_used > *prev_dist => {
                continue;
            }
            _ => (),
        }

        println!("calculating next moves");
        for next_move in state.moves() {
            dbg!(&next_move);
            dbg!(dist.get(&next_move.amphipod_hash()));
            match dist.get(&next_move.amphipod_hash()) {
                Some(prev_dist) if next_move.energy_used >= *prev_dist => (),
                _ => {
                    println!("putting next move on the heap");
                    dist.insert(next_move.amphipod_hash(), next_move.energy_used);
                    heap.push(next_move);
                }
            }
        }
    }

    panic!();
}

fn part1(start: &BurrowMap) -> u32 {
    easiest_sort(start)
}

fn part2(contents: &BurrowMap) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
        let mut map = parse(SAMPLE);

        map = map.move_amphipod(&map.pop[2], pt(4, 1), 2);
        dbg!(&map);
        map = map.move_amphipod(&map.pop[1], pt(7, 2), 2);
        dbg!(&map);

        // let d_moves: Vec<(Point2D, u32)> = map.valid_moves(&map.pop[5]).collect();

        // dbg!(d_moves);

        dbg!(map.moves());
        // panic!();
    }

    #[test]
    // #[ignore]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 12521);
    }

    #[test]
    #[ignore]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 0);
    }

    const SAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
}
