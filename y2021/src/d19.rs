use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};

use regex::Regex;

use crate::inputs::read_contents;

struct Scanner {
    beacons: HashSet<Beacon>,
}

impl From<&str> for Scanner {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"(?<x>-?\d+),(?<y>-?\d+),(?<z>-?\d+)").unwrap();

        let beacons = input.trim()
                           .lines()
                           .filter(|line| !line.starts_with("---"))
                           .map(|line| {
                               let caps = re.captures(line).expect(&format!("Could not parse line: '{}'", line));
                               Beacon::new(
                                   caps.name("x").unwrap().as_str().parse().unwrap(),
                                   caps.name("y").unwrap().as_str().parse().unwrap(),
                                   caps.name("z").unwrap().as_str().parse().unwrap(),
                               )
                           })
                           .collect();

        Scanner { beacons }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
struct Delta(isize, isize, isize);


#[derive(Hash, Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
struct Beacon(isize, isize, isize);


impl Sub for &Beacon {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        Delta(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add<&Delta> for &Beacon {
    type Output = Beacon;

    fn add(self, rhs: &Delta) -> Self::Output {
        Beacon(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Beacon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self(x, y, z)
    }

    fn manhattan_distance(&self, rhs: &Beacon) -> isize {
        let p = self - rhs;
        p.0.abs() + p.1.abs() + p.2.abs()
    }
}

fn max_manhattan_distance(beacons: &Vec<Beacon>) -> isize {
    // find pairwise beacon distance within the same scanner
    beacons.iter()
           .filter_map(|p1| {
               beacons.iter()
                      .filter_map(|p2| {
                          if p1 == p2 { None } else { Some(p1.manhattan_distance(p2)) }
                      })
                      .max()
           })
           .max()
           .unwrap()
}

fn parse_input(input: &str) -> VecDeque<Scanner> {
    let input = input.replace("\r", "");

    input.split("\n\n")
         .map(Scanner::from)
         .collect()
}

// The 24 transforms that we need to use to compare
const TRANSFORMS: [&'static dyn Fn(&Beacon) -> Beacon; 24] = [
    &|beacon: &Beacon| { beacon.clone() },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*z, *y, -*x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*x, *y, -*z) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*z, *y, *x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*x, -*y, *z) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*z, -*y, -*x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*x, -*y, -*z) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*z, -*y, *x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*x, -*z, *y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*y, -*z, -*x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*x, -*z, -*y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*y, -*z, *x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*x, *z, -*y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*y, *z, -*x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*x, *z, *y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*y, *z, *x) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*z, *x, *y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*y, *x, -*z) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*z, *x, -*y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*y, *x, *z) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*z, -*x, *y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*y, -*x, *z) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(*z, -*x, -*y) },
    &|_beacon @ Beacon(x, y, z): &Beacon| { Beacon::new(-*y, -*x, -*z) },
];

fn find_delta_vectors(beacons: &HashSet<Beacon>) -> HashMap<Beacon, BTreeSet<Delta>> {
    beacons.iter()
           .map(|p1| {
               (p1.to_owned(), beacons.iter()
                                      .filter_map(|p2| {
                                          if p1 == p2 { None } else { Some(p2 - p1) }
                                      })
                                      .collect())
           })
           .collect()
}

fn find_overlapping_transform(beacons: &HashSet<Beacon>, other_beacons: &HashSet<Beacon>) -> Option<(HashSet<Beacon>, Delta)> {
    let origin_vectors = find_delta_vectors(beacons);

    for transform in TRANSFORMS {
        let transformed_beacons = other_beacons.iter().map(transform).collect::<HashSet<Beacon>>();
        let comparison_vectors = find_delta_vectors(&transformed_beacons);

        for (b1, b1_vectors) in &origin_vectors {
            for (b2, b2_vectors) in &comparison_vectors {
                // if the intersection is more than 12, it likely means that the 2 points are exactly the same

                if b1_vectors.intersection(b2_vectors).count() >= 11 {
                    // we need to calculate the offset and move the points from the second beacon
                    let offset = b1 - b2;

                    // these are beacon locations that have been offset to be relative to the origin
                    let beacons = transformed_beacons.iter().map(|b| b + &offset).collect::<HashSet<_>>();
                    return Some((beacons, offset));
                }
            }
        }
    }
    None
}

fn coalesce_all_points(mut scanners: VecDeque<Scanner>) -> (HashSet<Beacon>, Vec<Beacon>) {
    let limit = scanners.len() * 5;
    let mut centers = vec![Beacon(0, 0, 0)];
    let mut beacons = scanners.pop_front().unwrap().beacons;

    let mut count = 0;
    while let Some(scanner) = scanners.pop_front() {
        if let Some((new_beacons, center)) = find_overlapping_transform(&beacons, &scanner.beacons) {
            beacons.extend(new_beacons);
            centers.push(&Beacon(0, 0, 0) + &center);
        } else {
            scanners.push_back(scanner);
            count += 1;

            if count > limit {
                panic!("Exceeded pushback limit");
            }
        }
    }

    (beacons, centers)
}

pub fn solve_a() {
    let scanners = parse_input(&read_contents(19));
    let (points, _) = coalesce_all_points(scanners);

    println!("Solution A: {}", points.len());
}

pub fn solve_b() {
    let scanners = parse_input(&read_contents(19));
    let (_, centers) = coalesce_all_points(scanners);

    println!("Solution B: {}", max_manhattan_distance(&centers));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Beacon, coalesce_all_points, find_delta_vectors, max_manhattan_distance, parse_input, Scanner, TRANSFORMS};

    const TEST_INPUT: &str = "--- scanner 0 ---
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
30,-46,-14";

    #[test]
    fn test_coalesce_all_points() {
        let scanners = parse_input(TEST_INPUT);
        assert_eq!(scanners.len(), 5);
        let (points, centers) = coalesce_all_points(scanners);
        assert_eq!(points.len(), 79);
        assert_eq!(max_manhattan_distance(&centers), 3621);
    }

    #[test]
    fn test_small_can_find_patterns() {
        let beacon1 = Scanner::from("0,2,0
4,1,0
3,3,0").beacons;
        let beacon2 = Scanner::from("-1,-1,0
-5,0,0
-2,1,0").beacons;

        let origins = find_delta_vectors(&beacon1);

        for transform in TRANSFORMS {
            let transformed_beacons = beacon2.iter().map(transform).collect::<HashSet<Beacon>>();
            let comparison_vectors = find_delta_vectors(&transformed_beacons);

            for (b1, delta1) in &origins {
                for (b2, delta2) in &comparison_vectors {
                    let c = delta1.intersection(delta2).count();
                    if c > 1 {
                        match b1 {
                            Beacon(0, 2, 0) => assert_eq!(*b2, Beacon(-5, 0, 0)),
                            Beacon(3, 3, 0) => assert_eq!(*b2, Beacon(-2, 1, 0)),
                            Beacon(4, 1, 0) => assert_eq!(*b2, Beacon(-1, -1, 0)),
                            _ => { unreachable!() }
                        }
                    }
                }
            }
        }
    }
}