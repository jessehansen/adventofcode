use aoc_common::*;
use std::fmt;
use std::str::FromStr;

fn main() {
    run_vec(parse, part1, part2);
}

#[derive(Clone, Debug)]
struct ReactorCore {
    on: Vec<Cuboid>,
}

impl ReactorCore {
    fn new() -> ReactorCore {
        ReactorCore { on: Vec::new() }
    }

    fn process(&mut self, instruction: &Instruction) {
        match instruction {
            On(cuboid) => self.turn_on(cuboid),
            Off(cuboid) => self.turn_off(cuboid),
        }
    }

    fn turn_on(&mut self, cuboid: &Cuboid) {
        // turn this cuboid off first, then we can simply add this cuboid to the on list
        self.turn_off(cuboid);
        self.on.push(*cuboid);
    }

    fn turn_off(&mut self, cuboid: &Cuboid) {
        'outer: loop {
            for i in 0..self.on.len() {
                let already_on = self.on[i];
                if cuboid.overlaps(&already_on) {
                    self.on.splice(i..=i, cuboid.difference(&already_on));
                    // need to start over since we changed "on"
                    continue 'outer;
                }
            }
            break;
        }
    }

    fn volume_on(&self) -> u64 {
        self.on.iter().map(|x| x.volume()).sum()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Point3D {
    type Err = ();

    fn from_str(point: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<i32> = point.split(',').map(|x| x.parse().unwrap()).collect();

        Ok(Point3D {
            x: parsed[0],
            y: parsed[1],
            z: parsed[2],
        })
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Cuboid {
    origin: Point3D,
    terminex: Point3D,
}

impl Cuboid {
    fn overlaps(&self, rhs: &Cuboid) -> bool {
        !(self.terminex.x < rhs.origin.x
            || rhs.terminex.x < self.origin.x
            || self.terminex.y < rhs.origin.y
            || rhs.terminex.y < self.origin.y
            || self.terminex.z < rhs.origin.z
            || rhs.terminex.z < self.origin.z)
    }

    // splits the cuboid in two at the x-coordinate
    fn split_x(&self, x: i32) -> (Cuboid, Cuboid) {
        if x < self.origin.x || x > self.terminex.x {
            panic!(
                "cannot split_x when x is not inside cuboid, x={}, origin={}, terminex={}",
                x, self.origin, self.terminex
            );
        }

        (
            Cuboid {
                origin: self.origin,
                terminex: Point3D {
                    x: x - 1,
                    y: self.terminex.y,
                    z: self.terminex.z,
                },
            },
            Cuboid {
                origin: Point3D {
                    x,
                    y: self.origin.y,
                    z: self.origin.z,
                },
                terminex: self.terminex,
            },
        )
    }

    // splits the cuboid in two at the y-coordinate
    fn split_y(&self, y: i32) -> (Cuboid, Cuboid) {
        if y < self.origin.y || y > self.terminex.y {
            panic!(
                "cannot split_y when y is not inside cuboid, y={}, origin={}, terminex={}",
                y, self.origin, self.terminex
            );
        }

        (
            Cuboid {
                origin: self.origin,
                terminex: Point3D {
                    x: self.terminex.x,
                    y: y - 1,
                    z: self.terminex.z,
                },
            },
            Cuboid {
                origin: Point3D {
                    x: self.origin.x,
                    y,
                    z: self.origin.z,
                },
                terminex: self.terminex,
            },
        )
    }

    // splits the cuboid in two at the y-coordinate
    fn split_z(&self, z: i32) -> (Cuboid, Cuboid) {
        if z < self.origin.z || z > self.terminex.z {
            panic!(
                "cannot split_z when z is not inside cuboid, z={}, origin={}, terminex={}",
                z, self.origin, self.terminex
            );
        }

        (
            Cuboid {
                origin: self.origin,
                terminex: Point3D {
                    x: self.terminex.x,
                    y: self.terminex.y,
                    z: z - 1,
                },
            },
            Cuboid {
                origin: Point3D {
                    x: self.origin.x,
                    y: self.origin.y,
                    z,
                },
                terminex: self.terminex,
            },
        )
    }

    // returns the portion of rhs that is not contained by self as a set of cuboids
    fn difference(&self, rhs: &Cuboid) -> Vec<Cuboid> {
        let mut result = vec![];
        let mut remainder = *rhs;

        if remainder.origin.x < self.origin.x {
            let (c1, c2) = remainder.split_x(self.origin.x);
            result.push(c1);
            remainder = c2;
        }

        if remainder.origin.y < self.origin.y {
            let (c1, c2) = remainder.split_y(self.origin.y);
            result.push(c1);
            remainder = c2;
        }

        if remainder.origin.z < self.origin.z {
            let (c1, c2) = remainder.split_z(self.origin.z);
            result.push(c1);
            remainder = c2;
        }

        if remainder.terminex.x > self.terminex.x {
            let (c1, c2) = remainder.split_x(self.terminex.x + 1);
            remainder = c1;
            result.push(c2);
        }

        if remainder.terminex.y > self.terminex.y {
            let (c1, c2) = remainder.split_y(self.terminex.y + 1);
            remainder = c1;
            result.push(c2);
        }

        if remainder.terminex.z > self.terminex.z {
            let (c1, c2) = remainder.split_z(self.terminex.z + 1);
            remainder = c1;
            result.push(c2);
        }

        assert!(self.overlaps(&remainder));

        result
    }

    // since reactor cores are discrete, volume is the number of contained points
    fn volume(&self) -> u64 {
        let width = self.terminex.x - self.origin.x + 1;
        let depth = self.terminex.y - self.origin.y + 1;
        let height = self.terminex.z - self.origin.z + 1;

        width as u64 * depth as u64 * height as u64
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(ranges: &str) -> Result<Self, Self::Err> {
        let bounds: Vec<(i32, i32)> = ranges
            .split(',')
            .map(|x| {
                let bounds: Vec<i32> = x[2..].split("..").map(|y| y.parse().unwrap()).collect();
                (bounds[0], bounds[1])
            })
            .collect();

        Ok(Cuboid {
            origin: Point3D {
                x: bounds[0].0,
                y: bounds[1].0,
                z: bounds[2].0,
            },
            terminex: Point3D {
                x: bounds[0].1,
                y: bounds[1].1,
                z: bounds[2].1,
            },
        })
    }
}

enum Instruction {
    On(Cuboid),
    Off(Cuboid),
}

impl Instruction {
    fn is_initialization(&self) -> bool {
        let cuboid = match self {
            On(c) => c,
            Off(c) => c,
        };

        !(cuboid.origin.x < -50
            || cuboid.origin.y < -50
            || cuboid.origin.z < -50
            || cuboid.terminex.x > 50
            || cuboid.terminex.y > 50
            || cuboid.terminex.z > 50)
    }
}

use Instruction::*;

impl FromStr for Instruction {
    type Err = ();

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let (on_off, cuboid) = instruction.split_once(' ').unwrap();
        let cuboid: Cuboid = cuboid.parse().unwrap();
        match on_off {
            "on" => Ok(On(cuboid)),
            "off" => Ok(Off(cuboid)),
            _ => panic!(),
        }
    }
}

fn parse(contents: &str) -> Vec<Instruction> {
    contents.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(instructions: &[Instruction]) -> u64 {
    let mut core = ReactorCore::new();
    for instruction in instructions {
        if instruction.is_initialization() {
            core.process(instruction);
        }
    }
    core.volume_on()
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut core = ReactorCore::new();
    for instruction in instructions {
        core.process(instruction);
    }
    core.volume_on()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 39);
    }

    #[test]
    fn large_sample_part1() {
        let parsed = parse(LARGE_SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 590784);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(LARGER_SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 2758514936282235);
    }

    const SAMPLE: &str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
";

    const LARGE_SAMPLE: &str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
";

    const LARGER_SAMPLE: &str = "\
on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
";
}
