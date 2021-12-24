use aoc_common::*;
use std::cmp::{Ord, Ordering, PartialEq};
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

#[derive(Clone)]
struct BurrowMap {
    pop: HashMap<Point2D, AmphipodColor>,
    energy_used: u32,
}

impl BurrowMap {
    fn moves(&self, map: &Grid2D<Cell>) -> Vec<BurrowMap> {
        let mut moves = vec![];
        for location in self.pop.keys().filter(|location| {
            // an amphipod can move if it is either in the hallway, or can move into the hallway
            // without passing another amphipod
            if map[**location] == Hall {
                true
            } else {
                let mut pt = location.up().unwrap();
                while map[pt] != Entry {
                    if self.pop.contains_key(&pt) {
                        return false;
                    }
                    pt = pt.up().unwrap();
                }
                true
            }
        }) {
            for (target, steps) in self.valid_moves(map, location) {
                moves.push(self.move_amphipod(location, &target, steps));
            }
        }
        moves
    }

    fn valid_moves(&self, map: &Grid2D<Cell>, start: &Point2D) -> Vec<(Point2D, u32)> {
        let color = self.pop[start];
        map.iter_horizontal()
            .filter_map(|(pt, cell)| match cell {
                // amphipod can only move into the hallway if it is already in a room
                Hall if matches!(map[*start], Destination(_)) => {
                    self.steps_to(start, &pt).map(|steps| (pt, steps))
                }
                // any amphipod can move into its correct room, so long as there's a way in
                Destination(c) if color == *c => {
                    if let Some(steps) = self.steps_to(start, &pt) {
                        let mut next_down = pt.down(map.bounds.height).unwrap();
                        while map[next_down] != Wall {
                            // everything below this point needs to match the destination color
                            // for this to be a valid move target. We don't want amphipods above
                            // empty spaces
                            match self.pop.get(&next_down) {
                                Some(color) if color != c => {
                                    // wrong color
                                    return None;
                                }
                                None => {
                                    // empty space
                                    return None;
                                }
                                // right color, move down again
                                _ => (),
                            }
                            next_down = next_down.down(map.bounds.height).unwrap();
                        }
                        Some((pt, steps))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    fn steps_to(&self, start: &Point2D, destination: &Point2D) -> Option<u32> {
        let mut cur = *start;
        let mut steps = 0;
        // have to move in the hallway before moving left and right
        while cur.y > 1 {
            match cur.up() {
                Some(next) if self.pop.contains_key(&next) => {
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
                Some(next) if self.pop.contains_key(&next) => {
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
            match cur.right(1000) {
                Some(next) if self.pop.contains_key(&next) => {
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
            match cur.down(1000) {
                Some(next) if self.pop.contains_key(&next) => {
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

    fn is_sorted(&self, map: &Grid2D<Cell>) -> bool {
        self.pop
            .iter()
            .all(|(location, color)| map[*location] == Destination(*color))
    }

    fn move_amphipod(
        &self,
        current_location: &Point2D,
        target_location: &Point2D,
        steps: u32,
    ) -> BurrowMap {
        let color = self.pop[current_location];
        let mut result_pop = HashMap::new();
        for (location, color) in &self.pop {
            result_pop.insert(
                if location != current_location {
                    *location
                } else {
                    *target_location
                },
                *color,
            );
        }
        BurrowMap {
            pop: result_pop,
            energy_used: self.energy_used + steps * color.energy_per_step(),
        }
    }

    fn amphipod_hash(&self) -> String {
        let mut amphipods: Vec<String> = self
            .pop
            .iter()
            .map(|(location, color)| format!("{}:{}", color, location))
            .collect();
        amphipods.sort();

        amphipods.join(";")
    }

    fn insert_fold(&self) -> BurrowMap {
        let mut result_pop = HashMap::new();
        for (location, color) in &self.pop {
            result_pop.insert(
                if location.y > 2 {
                    pt(location.x, location.y + 2)
                } else {
                    *location
                },
                *color,
            );
        }
        result_pop.insert(pt(3, 3), AmphipodColor::Desert);
        result_pop.insert(pt(5, 3), AmphipodColor::Copper);
        result_pop.insert(pt(7, 3), AmphipodColor::Bronze);
        result_pop.insert(pt(9, 3), AmphipodColor::Amber);

        result_pop.insert(pt(3, 4), AmphipodColor::Desert);
        result_pop.insert(pt(5, 4), AmphipodColor::Bronze);
        result_pop.insert(pt(7, 4), AmphipodColor::Amber);
        result_pop.insert(pt(9, 4), AmphipodColor::Copper);

        BurrowMap {
            pop: result_pop,
            energy_used: self.energy_used,
        }
    }

    fn _print_debug(&self, map: &Grid2D<Cell>) {
        let mut result = Grid2D::new_constant(map.bounds, " ");
        map.iter_horizontal().for_each(|(pt, x)| {
            result[pt] = match x {
                Wall => "#",
                _ => ".",
            }
        });

        for (location, color) in &self.pop {
            result[*location] = match color {
                AmphipodColor::Amber => "A",
                AmphipodColor::Bronze => "B",
                AmphipodColor::Copper => "C",
                AmphipodColor::Desert => "D",
            };
        }

        println!("map:\n{}\nenergy_used: {}", result, self.energy_used,)
    }
}

// for BinaryHeap priority queue
impl Ord for BurrowMap {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .energy_used
            .cmp(&self.energy_used)
            .then_with(|| self.amphipod_hash().cmp(&other.amphipod_hash()))
    }
}

impl PartialOrd for BurrowMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BurrowMap {
    fn eq(&self, other: &Self) -> bool {
        self.amphipod_hash() == other.amphipod_hash() && self.energy_used == other.energy_used
    }
}

impl Eq for BurrowMap {}

fn parse(contents: &str) -> (BurrowMap, Grid2D<Cell>) {
    let lines: Vec<&str> = contents.lines().collect();
    let mut map = Grid2D::new_constant(
        Bounds2D {
            width: lines[0].len(),
            height: lines.len(),
        },
        Cell::Void,
    );
    let mut pop = HashMap::new();
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

                    pop.insert(location, AmphipodColor::from_char(class));
                    result
                }
            }
        }
    }

    (
        BurrowMap {
            pop,
            energy_used: 0,
        },
        map,
    )
}

fn easiest_sort(start: &BurrowMap, map: &Grid2D<Cell>) -> u32 {
    let mut dist = HashMap::<String, u32>::new();
    let mut heap = BinaryHeap::new();
    heap.push(start.clone());

    // using a binary heap means we're always looking at the next best unvisited node
    while let Some(state) = heap.pop() {
        if state.is_sorted(map) {
            // state.print_debug(map);
            return state.energy_used;
        }

        // if we're already above the previous best to this point, don't bother continuing
        match dist.get(&state.amphipod_hash()) {
            Some(prev_dist) if state.energy_used > *prev_dist => {
                continue;
            }
            _ => (),
        }

        for next_move in state.moves(map) {
            match dist.get(&next_move.amphipod_hash()) {
                Some(prev_dist) if next_move.energy_used >= *prev_dist => (),
                _ => {
                    dist.insert(next_move.amphipod_hash(), next_move.energy_used);
                    heap.push(next_move);
                }
            }
        }
    }

    panic!();
}

fn part1((start, map): &(BurrowMap, Grid2D<Cell>)) -> u32 {
    easiest_sort(start, map)
}

fn part2((start, _): &(BurrowMap, Grid2D<Cell>)) -> u32 {
    let (_, big_map) = parse(
        "\
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########
",
    );
    let start = start.insert_fold();
    // start.print_debug(&big_map);
    easiest_sort(&start, &big_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 12521);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 44169);
    }

    const SAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
}
