use aoc_common::*;
use nalgebra::{vector, Matrix3, Vector3};
use std::fmt;
use std::str::FromStr;

fn main() {
    run_progressive_vec(parse, part1, part2);
}

static ROTATION_MATRICES: [Matrix3<i32>; 24] = [
    Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, 1),
    Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0),
    Matrix3::new(1, 0, 0, 0, -1, 0, 0, 0, -1),
    Matrix3::new(1, 0, 0, 0, 0, -1, 0, 1, 0),
    Matrix3::new(0, 1, 0, 0, 0, 1, 1, 0, 0),
    Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 1, 0, 0, 0, -1, -1, 0, 0),
    Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
    Matrix3::new(0, 0, 1, 1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0),
    Matrix3::new(0, 0, 1, -1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, 1, 0, -1, 0, 1, 0, 0),
    Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1),
    Matrix3::new(-1, 0, 0, 0, 0, 1, 0, 1, 0),
    Matrix3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1),
    Matrix3::new(-1, 0, 0, 0, 0, -1, 0, -1, 0),
    Matrix3::new(0, -1, 0, 0, 0, -1, 1, 0, 0),
    Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
    Matrix3::new(0, -1, 0, 0, 0, 1, -1, 0, 0),
    Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, -1),
    Matrix3::new(0, 0, -1, -1, 0, 0, 0, 1, 0),
    Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0),
    Matrix3::new(0, 0, -1, 1, 0, 0, 0, -1, 0),
    Matrix3::new(0, 0, -1, 0, -1, 0, -1, 0, 0),
];

#[derive(Copy, Clone, PartialEq, Eq)]
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

    fn as_vector(&self) -> Vector3<i32> {
        vector![self.x, self.y, self.z]
    }

    fn from_vector(v: &Vector3<i32>) -> Point3D {
        Point3D {
            x: v[0],
            y: v[1],
            z: v[2],
        }
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
    other_scanners: Vec<Point3D>,
}

struct OverlappingBeacons {
    ref_beacon: usize,
    other_ref_beacon: usize,
    overlapping_beacon_indices: Vec<(usize, usize)>,
}

impl Scanner {
    fn new(beacons: Vec<Point3D>) -> Scanner {
        let mut result = Scanner {
            beacons,
            dist_matrix: vec![],
            other_scanners: vec![],
        };

        result.calc_dist_matrix();

        result
    }

    fn calc_dist_matrix(&mut self) {
        self.dist_matrix =
            vec![vec![Point3D { x: 0, y: 0, z: 0 }; self.beacons.len()]; self.beacons.len()];

        for i in 0..self.beacons.len() {
            for j in 0..self.beacons.len() {
                if i == j {
                    continue;
                }

                self.dist_matrix[i][j] = self.beacons[i].relative_to(self.beacons[j]);
            }
        }
    }

    fn overlapping_beacons(&self, other: &Scanner) -> Option<OverlappingBeacons> {
        for i in 0..self.beacons.len() {
            let distances = &self.dist_matrix[i];
            // see if we have 12 overlapping distances
            for j in 0..other.beacons.len() {
                let other_distances = &other.dist_matrix[j];

                let overlapping: Vec<(usize, usize)> = (0..(self.beacons.len()))
                    .filter_map(|my_beacon_ix| {
                        let maybe_other_ix = (0..(other.beacons.len())).find(|other_beacon_ix| {
                            distances[my_beacon_ix]
                                .equal_distance_from_origin(other_distances[*other_beacon_ix])
                        });

                        if let Some(other_beacon_ix) = maybe_other_ix {
                            return Some((my_beacon_ix, other_beacon_ix));
                        }
                        None
                    })
                    .collect();

                if overlapping.len() >= 12 {
                    return Some(OverlappingBeacons {
                        ref_beacon: i,
                        other_ref_beacon: j,
                        overlapping_beacon_indices: overlapping,
                    });
                }
            }
        }
        None
    }

    fn find_rotation(&self, other: &Scanner, overlap: &OverlappingBeacons) -> Option<Matrix3<i32>> {
        let my_distances = &self.dist_matrix[overlap.ref_beacon];
        let other_distances = &other.dist_matrix[overlap.other_ref_beacon];
        for rot in ROTATION_MATRICES {
            if overlap
                .overlapping_beacon_indices
                .iter()
                .all(|(my_beacon_ix, other_beacon_ix)| {
                    let rotated = rot * other_distances[*other_beacon_ix].as_vector();
                    my_distances[*my_beacon_ix].as_vector() == rotated
                })
            {
                return Some(rot);
            }
        }
        None
    }

    fn plot_beacons(&mut self, other: &Scanner) -> bool {
        if let Some(overlap) = self.overlapping_beacons(other) {
            if let Some(rot) = self.find_rotation(other, &overlap) {
                let rotated_other_beacon =
                    rot * other.beacons[overlap.other_ref_beacon].as_vector();

                let translation =
                    self.beacons[overlap.ref_beacon].as_vector() - rotated_other_beacon;
                self.other_scanners.push(Point3D::from_vector(&translation));

                for beacon in &other.beacons {
                    let normalized =
                        Point3D::from_vector(&(rot * beacon.as_vector() + translation));
                    if !self.beacons.contains(&normalized) {
                        self.beacons.push(normalized);
                    }
                }
                self.calc_dist_matrix();

                return true;
            }
        }
        false
    }

    fn max_manhattan_distance(&self) -> i32 {
        let mut max = 0;

        for i in 0..self.other_scanners.len() {
            for j in 0..self.other_scanners.len() {
                let a = self.other_scanners[i];
                let b = self.other_scanners[j];
                let dist = (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs();
                if dist > max {
                    max = dist
                }
            }
        }

        max
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

fn part1(scanners: &[Scanner]) -> (usize, Scanner) {
    let mut scanners = scanners.to_owned();
    let mut reference_scanner = scanners.remove(0);

    while !scanners.is_empty() {
        for i in 0..scanners.len() {
            if reference_scanner.plot_beacons(&scanners[i]) {
                scanners.remove(i);
                break;
            }
        }
    }

    (reference_scanner.beacons.len(), reference_scanner)
}

fn part2(_: &[Scanner], scanner: &Scanner) -> i32 {
    scanner.max_manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_beacons() {
        let scanners = parse(SAMPLE);

        let result = scanners[0].overlapping_beacons(&scanners[1]).unwrap();

        assert_eq!(result.overlapping_beacon_indices.len(), 12);
    }

    #[test]
    fn single_plot() {
        let scanners = parse(SAMPLE);

        let mut origin = scanners[0].clone();

        assert!(origin.plot_beacons(&scanners[1]));
        assert_eq!(origin.beacons.len(), 38);
    }

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let (result, _) = part1(&parsed);

        assert_eq!(result, 79);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let (_, scanner) = part1(&parsed);

        let result = part2(&scanner);

        assert_eq!(result, 3621);
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
