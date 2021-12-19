use aoc_common::*;
use std::fmt;
use std::str::FromStr;

fn main() {
    run_vec(parse, part1, part2);
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

struct Rotation3D {
    matrix: [[i32; 3]; 3],
}

impl Rotation3D {
    fn execute(&self, pt: Point3D) -> Point3D {
        let pt = [pt.x, pt.y, pt.z];
        let mx = self.matrix;
        let mut res = [0; 3];
        for i in 0..3 {
            res[i] = pt[0] * mx[i][0] + pt[1] * mx[i][1] + pt[2] * mx[i][2];
        }

        Point3D {
            x: res[0],
            y: res[1],
            z: res[2],
        }
    }

    fn execute_times(&self, pt: Point3D, times: usize) -> Point3D {
        let mut res = pt;
        for _ in 0..times {
            res = self.execute(res);
        }
        res
    }

    fn x90() -> Rotation3D {
        Rotation3D {
            matrix: [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        }
    }

    fn y90() -> Rotation3D {
        Rotation3D {
            matrix: [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
        }
    }

    fn z90() -> Rotation3D {
        Rotation3D {
            matrix: [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        }
    }
}

impl Point3D {
    fn relative_to(&self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn equal_distance_from_origin(&self, other: Point3D) -> bool {
        let mut self_abs = vec![self.x.abs(), self.y.abs(), self.z.abs()];
        let mut other_abs = vec![other.x.abs(), other.y.abs(), other.z.abs()];
        self_abs.sort_unstable();
        other_abs.sort_unstable();

        self_abs == other_abs
    }
}

impl std::ops::Sub for Point3D {
    type Output = Point3D;

    fn sub(self, other: Self) -> Self::Output {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
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

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Point3D>,
    dist_matrix: Vec<Vec<Point3D>>,
}

impl Scanner {
    fn new(beacons: Vec<Point3D>) -> Scanner {
        let mut dist_matrix =
            vec![vec![Point3D { x: 0, y: 0, z: 0 }; beacons.len()]; beacons.len()];

        for i in 0..beacons.len() {
            for j in 0..beacons.len() {
                if i == j {
                    continue;
                }

                dist_matrix[i][j] = beacons[i].relative_to(beacons[j]);
            }
        }

        Scanner {
            beacons,
            dist_matrix,
        }
    }

    fn overlapping_beacons(&self, other: &Scanner) -> Option<(usize, usize, Vec<Point3D>)> {
        println!("finding overlapping beacons");
        for i in 0..self.beacons.len() {
            let distances = &self.dist_matrix[i];
            // see if we have 12 overlapping distances
            for j in 0..other.beacons.len() {
                let other_distances = &other.dist_matrix[j];

                let overlapping: Vec<Point3D> = distances
                    .iter()
                    .filter(|x| {
                        other_distances
                            .iter()
                            .any(|y| x.equal_distance_from_origin(*y))
                    })
                    .cloned()
                    .collect();

                if overlapping.len() >= 12 {
                    return Some((i, j, overlapping));
                }
            }
        }
        None
    }

    fn find_rotation(
        &self,
        other: &Scanner,
        ref_beacon: usize,
        other_ref_beacon: usize,
    ) -> Option<(usize, usize, usize)> {
        let ref_beacon = &self.dist_matrix[ref_beacon];
        let other_ref_beacon = &other.dist_matrix[other_ref_beacon];

        // find rotation
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    if ref_beacon.iter().all(|my_beacon_dist| {
                        if my_beacon_dist.x == 0 && my_beacon_dist.y == 0 && my_beacon_dist.z == 0 {
                            return true;
                        }

                        other_ref_beacon.iter().any(|other_beacon_dist| {
                            let mut rotated =
                                Rotation3D::x90().execute_times(*other_beacon_dist, x);
                            rotated = Rotation3D::y90().execute_times(rotated, y);
                            rotated = Rotation3D::z90().execute_times(rotated, z);
                            *my_beacon_dist == rotated
                        })
                    }) {
                        return Some((x, y, z));
                    }
                }
            }
        }
        None
    }

    fn plot_beacons(&mut self, other: &Scanner) -> bool {
        if let Some((ref_beacon, other_ref_beacon, _overlapping)) = self.overlapping_beacons(other)
        {
            println!(
                "found overlapping beacons: self.beacons[{}] = other.beacons[{}]",
                ref_beacon, other_ref_beacon
            );
            if let Some((x, y, z)) = self.find_rotation(other, ref_beacon, other_ref_beacon) {
                println!("found rotation: ({}, {}, {})", x, y, z);
                let mut rotated_other_beacon =
                    Rotation3D::x90().execute_times(other.beacons[other_ref_beacon], x);
                rotated_other_beacon = Rotation3D::y90().execute_times(rotated_other_beacon, y);
                rotated_other_beacon = Rotation3D::z90().execute_times(rotated_other_beacon, z);

                let translation = self.beacons[ref_beacon] - rotated_other_beacon;
                println!("found other scanner at ({})", translation);

                for beacon in &other.beacons {
                    if !self.beacons.contains(&beacon) {
                        self.beacons.push(*beacon);
                    }
                }
                return true;
            }
        }
        false
    }
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(scanner: &str) -> Result<Self, Self::Err> {
        Ok(Scanner::new(
            scanner
                .lines()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect(),
        ))
    }
}

fn parse(contents: &str) -> Vec<Scanner> {
    contents.split("\n\n").map(|x| x.parse().unwrap()).collect()
}

fn part1(scanners: &[Scanner]) -> usize {
    let mut reference_scanner = scanners[0].clone();
    reference_scanner.plot_beacons(&scanners[1]);

    reference_scanner.beacons.len()
}

fn part2(contents: &[Scanner]) -> usize {
    contents.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_beacons() {
        let scanners = parse(SAMPLE);

        let (_, _, result) = scanners[0].overlapping_beacons(&scanners[1]).unwrap();

        assert_eq!(result.len(), 12);

        // for i in 1..scanners.len() {
        // dbg!(scanners[0].overlapping_beacons(&scanners[i]));
        // }
        // panic!("show me output");
    }

    #[test]
    fn point_rotation() {
        let initial = Point3D { x: 1, y: 2, z: 3 };

        assert_eq!(
            Rotation3D::x90().execute(initial),
            Point3D { x: 1, y: -3, z: 2 }
        );

        assert_eq!(
            Rotation3D::y90().execute(initial),
            Point3D { x: 3, y: 2, z: -1 }
        );

        assert_eq!(
            Rotation3D::z90().execute(initial),
            Point3D { x: -2, y: 1, z: 3 }
        );

        assert_eq!(
            Rotation3D::z90().execute_times(initial, 2),
            Point3D { x: -1, y: -2, z: 3 }
        );

        assert_eq!(
            Rotation3D::z90().execute_times(initial, 3),
            Point3D { x: 2, y: -1, z: 3 }
        );

        assert_eq!(Rotation3D::z90().execute_times(initial, 4), initial);
    }

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 79);
    }

    #[test]
    #[ignore]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 0);
    }

    const SAMPLE: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";
}
