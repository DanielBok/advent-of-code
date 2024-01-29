use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

const PUZZLE_INPUT: &str = "#..#....#...#.#..#.......##.#.####
#......#..#.#..####.....#..#...##.
.##.......#..#.#....#.#..#.#....#.
###..#.....###.#....##.....#...#..
...#.##..#.###.......#....#....###
.####...##...........##..#..#.##..
..#...#.#.#.###....#.#...##.....#.
......#.....#..#...##.#..##.#..###
...###.#....#..##.#.#.#....#...###
..#.###.####..###.#.##..#.##.###..
...##...#.#..##.#............##.##
....#.##.##.##..#......##.........
.#..#.#..#.##......##...#.#.#...##
.##.....#.#.##...#.#.#...#..###...
#.#.#..##......#...#...#.......#..
#.......#..#####.###.#..#..#.#.#..
.#......##......##...#..#..#..###.
#.#...#..#....##.#....#.##.#....#.
....#..#....##..#...##..#..#.#.##.
#.#.#.#.##.#.#..###.......#....###
...#.#..##....###.####.#..#.#..#..
#....##..#...##.#.#.........##.#..
.#....#.#...#.#.........#..#......
...#..###...#...#.#.#...#.#..##.##
.####.##.#..#.#.#.#...#.##......#.
.##....##..#.#.#.......#.....####.
#.##.##....#...#..#.#..###..#.###.
...###.#..#.....#.#.#.#....#....#.
......#...#.........##....#....##.
.....#.....#..#.##.#.###.#..##....
.#.....#.#.....#####.....##..#....
.####.##...#.......####..#....##..
.#.#.......#......#.##..##.#.#..##
......##.....##...##.##...##......";

#[derive(PartialEq, Hash, Eq)]
struct Gradient {
    dx: i32,
    dy: i32,
}

impl Gradient {
    fn new(dx: i32, dy: i32) -> Gradient {
        if dx == 0 {
            Gradient { dx, dy: dy / dy.abs() }
        } else if dy == 0 {
            Gradient { dx: dx / dx.abs(), dy }
        } else {
            let gcd = Gradient::gcd(dx, dy);
            Gradient { dx: dx / gcd, dy: dy / gcd }
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a.abs() } else { Gradient::gcd(b, a % b) }
    }

    fn to_degrees(&self) -> f32 {
        match (self.dx, self.dy) {
            (0, -1) => 360.0,
            (1, 0) => 270.0,
            (0, 1) => 180.0,
            (-1, 0) => 90.0,
            (dx, dy) => {
                let angle = (dy.abs() as f32).atan2(dx.abs() as f32).to_degrees();

                if dx > 0 {
                    270.0 + if dy < 0 { angle } else { -angle }
                } else {
                    90.0 - if dy < 0 { angle } else { -angle }
                }
            }
        }
    }
}

impl Debug for Gradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gradient(dx={}, dy={}, degrees={})", self.dx, self.dy, self.to_degrees())
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn gradient(&self, other: &Point) -> Gradient {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        Gradient::new(dx, dy)
    }

    /// Returns the manhattan distance
    fn distance(&self, other: &Point) -> i32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        return dx.abs() + dy.abs();
    }
}


fn create_map(input: &str) -> Vec<Point> {
    let mut map: Vec<Point> = Vec::new();

    for (y, row) in input.split("\n").enumerate() {
        for (x, element) in row.trim().chars().enumerate() {
            if element == '#' {
                map.push(Point::new(x as i32, y as i32));
            }
        }
    }

    map
}

pub fn solve_a() {
    let map = create_map(PUZZLE_INPUT);
    let (_, ans) = num_satellites_viewed_from_best_station(&map);

    assert_eq!(ans, 334);
    println!("Solution A: {}", ans);
}

fn num_satellites_viewed_from_best_station(map: &Vec<Point>) -> (&Point, usize) {
    let mut max_station_seen = 0;
    let mut best_point = &map[0];
    for source in map {
        let can_see = num_viewable_satellites(map, source);
        if can_see > max_station_seen {
            max_station_seen = can_see;
            best_point = source;
        }
    }

    (best_point, max_station_seen)
}

fn num_viewable_satellites(map: &Vec<Point>, source: &Point) -> usize {
    let mut seen: HashSet<Gradient> = HashSet::new();

    for point in map {
        if point != source {
            seen.insert(source.gradient(point));
        }
    }

    seen.len()
}

pub fn solve_b() {
    let map = create_map(PUZZLE_INPUT);
    let (source, _) = num_satellites_viewed_from_best_station(&map);

    let point = get_asteroid_destroyed(&map, source, 200);
    let ans = point.x * 100 + point.y;

    assert_eq!(ans, 1119);
    println!("Solution B: {}", ans);
}

fn get_asteroid_destroyed<'a>(map: &'a Vec<Point>, source: &'a Point, idx: usize) -> &'a Point {
    assert!(map.len() - 1 > idx, "Index exceeds number of asteroids present");

    let asteroid_map = make_asteroid_map(map, source);
    let keys: Vec<&Gradient> = asteroid_map.iter()
        .map(|(k, _)| k)
        .sorted_by(|&x, &y| {
            y.to_degrees().total_cmp(&x.to_degrees())
        })
        .collect();

    let mut count = 0;
    let mut level = 1;
    while count < idx {
        for &g in &keys {
            let asteroids = asteroid_map.get(g).unwrap();
            if asteroids.len() >= level {
                count += 1;
                if count == idx {
                    return asteroids[level - 1];
                }
            }
        }
        level += 1;
    }

    panic!("Could not find the asteroid number {} to be destroyed", idx);
}

fn make_asteroid_map<'a>(map: &'a Vec<Point>, source: &'a Point) -> HashMap<Gradient, Vec<&'a Point>> {
    let mut asteroid_map: HashMap<Gradient, Vec<&Point>> = HashMap::new();

    for point in map {
        if point == source {
            continue;
        }

        let g = source.gradient(point);

        if asteroid_map.contains_key(&g) {
            asteroid_map.entry(g).and_modify(|v| v.push(point));
        } else {
            asteroid_map.insert(g, vec![point]);
        }
    }

    asteroid_map.iter_mut().for_each(|(_, v)| {
        v.sort_by(|&p1, &p2| Ord::cmp(&source.distance(p1), &source.distance(p2)))
    });

    asteroid_map
}


#[cfg(test)]
mod tests {
    use crate::d10::{create_map, get_asteroid_destroyed, Gradient, num_satellites_viewed_from_best_station, Point};

    #[test]
    fn test_num_satellites_viewed_from_best_station() {
        for (inp, exp_num_stations, exp_point) in [
            (".#..#
.....
#####
....#
...##", 8, Point::new(3, 4)),
            (
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####", 33, Point::new(5, 8)),
            ("#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.", 35, Point::new(1, 2)),
            (".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..", 41, Point::new(6, 3)),
            (".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##", 210, Point::new(11, 13))
        ] {
            let map = create_map(inp);
            let (point, ans) = num_satellites_viewed_from_best_station(&map);

            assert_eq!(point, &exp_point);
            assert_eq!(ans, exp_num_stations, "Did not find correct answer for map\n\n{}", inp);
        }
    }

    #[test]
    fn test_gradient_to_degrees() {
        assert_eq!(Gradient::new(0, -1).to_degrees(), 360.0);
        assert_eq!(Gradient::new(1, -1).to_degrees(), 315.0);
        assert_eq!(Gradient::new(1, 0).to_degrees(), 270.0);
        assert_eq!(Gradient::new(1, 1).to_degrees(), 225.0);
        assert_eq!(Gradient::new(0, 1).to_degrees(), 180.0);
        assert_eq!(Gradient::new(-1, 1).to_degrees(), 135.0);
        assert_eq!(Gradient::new(-1, 0).to_degrees(), 90.0);
        assert_eq!(Gradient::new(-1, -1).to_degrees(), 45.0);
    }

    #[test]
    fn test_get_asteroid_destroyed_by_index() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let source = Point::new(11, 13);
        for (idx, (ex, ey)) in [
            (1, (11, 12)),
            (2, (12, 1)),
            (200, (8, 2)),
        ] {
            let map = create_map(input);
            let point = get_asteroid_destroyed(&map, &source, idx);
            assert_eq!(point, &Point::new(ex, ey));
        }
    }
}