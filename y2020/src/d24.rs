use std::collections::{HashMap, HashSet};

use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::inputs::read_contents;

#[derive(Copy, Clone)]
enum Direction {
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

#[derive(Eq, PartialEq, Debug)]
enum TileColor {
    White,
    Black,
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input.lines()
         .map(|line| {
             let chars = line.chars().collect_vec();

             let mut directions = vec![];
             let mut i = 0;
             while i < chars.len() {
                 let ch = chars[i];
                 directions.push(match ch {
                     'e' => {
                         i += 1;
                         Direction::East
                     }
                     'w' => {
                         i += 1;
                         Direction::West
                     }
                     _ if ch == 'n' || ch == 's' => {
                         let ch2 = chars[i + 1];

                         assert!(ch2 == 'e' || ch2 == 'w', "Second character must be 'e' or 'w', got '{}'", ch2);
                         i += 2;
                         match (ch, ch2) {
                             ('n', 'e') => Direction::NorthEast,
                             ('n', 'w') => Direction::NorthWest,
                             ('s', 'e') => Direction::SouthEast,
                             ('s', 'w') => Direction::SouthWest,
                             _ => panic!("Invalid combination: '{}{}'", ch, ch2)
                         }
                     }
                     _ => panic!("Invalid character: {}", ch)
                 });
             }

             directions
         })
         .collect()
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point(i32, i32);

impl Point {
    fn origin() -> Point {
        Point(0, 0)
    }

    fn move_point(&self, dir: Direction) -> Point {
        match dir {
            Direction::East => Point(self.0 + 2, self.1),
            Direction::West => Point(self.0 - 2, self.1),
            Direction::NorthEast => Point(self.0 + 1, self.1 + 1),
            Direction::SouthEast => Point(self.0 + 1, self.1 - 1),
            Direction::NorthWest => Point(self.0 - 1, self.1 + 1),
            Direction::SouthWest => Point(self.0 - 1, self.1 - 1),
        }
    }

    fn neighbours(&self) -> [Point; 6] {
        [
            self.move_point(Direction::East),
            self.move_point(Direction::West),
            self.move_point(Direction::NorthEast),
            self.move_point(Direction::SouthEast),
            self.move_point(Direction::NorthWest),
            self.move_point(Direction::SouthWest),
        ]
    }
}


pub fn solve_a() {
    let direction_list = parse_input(&read_contents(24));
    let map = flip_tiles(&direction_list);
    let ans = count_tiles(&map, TileColor::Black);
    println!("Solution A: {}", ans);
}

fn flip_tiles(direction_list: &Vec<Vec<Direction>>) -> HashMap<Point, TileColor> {
    let mut map = HashMap::new();

    for directions in direction_list {
        let mut point = Point::origin();
        for d in directions {
            point = point.move_point(*d);
        }

        if let Some(color) = map.get_mut(&point) {
            *color = match color {
                TileColor::White => TileColor::Black,
                TileColor::Black => TileColor::White,
            };
        } else {
            map.insert(point, TileColor::Black);
        }
    }

    map
}

fn count_tiles(map: &HashMap<Point, TileColor>, color: TileColor) -> usize {
    map.values().filter(|c| color == **c).count()
}

pub fn solve_b() {
    let direction_list = parse_input(&read_contents(24));
    let map = flip_tiles(&direction_list);
    let ans = simulate_tile_flips(map, 100);
    println!("Solution B: {}", ans);
}

fn simulate_tile_flips(map: HashMap<Point, TileColor>, days: usize) -> usize {
    let mut map = map;

    for _ in (0..days).progress() {
        let mut next_map = HashMap::new();
        let points_to_check = get_points_to_check(&mut map);

        for p in points_to_check {
            let n = num_black_neighbours(&map, &p);
            let color = map.get(&p).unwrap();

            let next_color = match color {
                TileColor::White => if n == 2 { TileColor::Black } else { TileColor::White },
                TileColor::Black => if n == 0 || n > 2 { TileColor::White } else { TileColor::Black },
            };

            next_map.insert(p, next_color);
        }
        map = next_map;
    }

    count_tiles(&map, TileColor::Black)
}


fn get_points_to_check(map: &mut HashMap<Point, TileColor>) -> HashSet<Point> {
    let points = map.keys().into_iter().cloned().collect_vec();

    let mut points_to_check = HashSet::new();

    for p in points {
        if *map.get(&p).unwrap() == TileColor::Black {
            for nb in p.neighbours() {
                map.entry(nb).or_insert(TileColor::White);
            }

            points_to_check.extend(p.neighbours());
            points_to_check.insert(p);
        }
    }

    points_to_check
}

fn num_black_neighbours(map: &HashMap<Point, TileColor>, point: &Point) -> usize {
    point.neighbours()
         .into_iter()
         .filter(|p| map.get(p).map_or_else(|| false, |c| *c == TileColor::Black))
         .count()
}


#[cfg(test)]
mod tests {
    use super::{count_tiles, flip_tiles, parse_input, simulate_tile_flips, TileColor};

    const TEST_INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_flip_tiles() {
        let direction_list = parse_input(TEST_INPUT);

        let map = flip_tiles(&direction_list);
        let ans = count_tiles(&map, TileColor::Black);
        assert_eq!(ans, 10);
    }

    #[test]
    fn test_simulate_tile_flips() {
        let direction_list = parse_input(TEST_INPUT);

        let map = flip_tiles(&direction_list);
        let ans = simulate_tile_flips(map, 100);
        assert_eq!(ans, 2208);
    }
}