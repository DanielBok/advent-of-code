use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use itertools::Itertools;

const PUZZLE_INPUT: &str = "####.
#....
#..#.
.#.#.
##.##";

struct Map1D {
    map: HashMap<usize, bool>,
    neighbours: HashMap<usize, Vec<usize>>,
}

impl Map1D {
    fn new(input: &str) -> Map1D {
        let mut map = HashMap::new();
        let mut neighbours = HashMap::new();
        let num_rows = input.lines().count();

        let mut idx = 0;
        for (row, line) in input.lines().enumerate() {
            let num_cols = line.len();

            for (col, c) in line.chars().enumerate() {
                map.insert(idx, match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Invalid input: {}", c)
                });

                let mut nbr = Vec::new();

                if row > 0 {
                    nbr.push(idx - num_cols);
                }
                if row + 1 < num_rows {
                    nbr.push(idx + num_cols)
                }
                if col > 0 {
                    nbr.push(idx - 1);
                }
                if col + 1 < line.len() {
                    nbr.push(idx + 1);
                }

                neighbours.insert(idx, nbr);
                idx += 1;
            }
        }

        Map1D { map, neighbours }
    }

    fn biodiversity_score(&self) -> u64 {
        self.map.iter().fold(0, |acc, (&idx, &has_bug)| {
            acc + if has_bug { 1 << idx } else { 0 }
        })
    }

    fn evolve(&mut self) {
        let mut next_map = self.map.clone();
        for (idx, has_bug) in self.map.iter() {
            let nbrs = self.neighbours.get(idx).unwrap();

            // number of neighbouring bugs
            let num_nbr = nbrs.iter()
                .filter(|&ni| *self.map.get(&ni).unwrap())
                .count();

            if *has_bug {
                // if current tile has bug, next tile has bug only if there is only one bug neighbour
                next_map.insert(*idx, num_nbr == 1);
            } else {
                // if current tile has no bugs, next tile has bug only if there are 1 or 2 bug neighbours
                next_map.insert(*idx, num_nbr == 1 || num_nbr == 2);
            }
        }

        self.map = next_map;
    }
}


struct Eris {
    map: Map1D,
    seen: HashSet<u64>,
}

impl Eris {
    fn new(input: &str) -> Eris {
        let map = Map1D::new(input);
        let seen = HashSet::from([map.biodiversity_score()]);

        Eris { map, seen }
    }

    fn get_first_repeated_map(&mut self) -> u64 {
        loop {
            self.map.evolve();

            let score = self.map.biodiversity_score();

            if self.seen.contains(&score) {
                return score;
            } else {
                self.seen.insert(score);
            }
        }
    }
}

pub fn solve_a() {
    let mut eris = Eris::new(PUZZLE_INPUT);
    let ans = eris.get_first_repeated_map();

    println!("Solution A: {}", ans);
}

struct Level(Vec<bool>);

impl Level {
    fn new() -> Level {
        Level(vec![false; 25])
    }

    fn from_input(input: &str) -> Level {
        // hard code to work on 5x5 map
        assert_eq!(input.chars().filter(|&c| c == '#' || c == '.' || c == '?').count(), 25);

        let mut level = Vec::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if i == 2 && j == 2 {
                    level.push(false);
                } else {
                    level.push(c == '#')
                }
            }
        };
        Level(level)
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        !self.0.iter().any(|x| *x)
    }

    fn get(&self, index: usize) -> bool {
        self.0[index]
    }

    fn evolve(&self, above: &Level, below: &Level) -> Level {
        let mut next = Vec::from([false; 25]);

        for (i, &has_bug) in self.0.iter().enumerate() {
            let num_neighbours = match i {
                0 => {
                    // top left corner
                    vec![
                        above.get(7),
                        above.get(11),
                        self.get(i + 1),
                        self.get(i + 5),
                    ]
                }
                4 => {
                    // top right corner
                    vec![
                        above.get(7),
                        above.get(13),
                        self.get(i - 1),
                        self.get(i + 5),
                    ]
                }
                20 => {
                    // bottom left hand corner
                    vec![
                        above.get(17),
                        above.get(11),
                        self.get(i + 1),
                        self.get(i - 5),
                    ]
                }
                24 => {
                    // bottom right hand corner
                    vec![
                        above.get(17),
                        above.get(13),
                        self.get(i - 1),
                        self.get(i - 5),
                    ]
                }
                1..=3 => {
                    // top row
                    vec![
                        above.get(7),
                        self.get(i - 1),
                        self.get(i + 1),
                        self.get(i + 5),
                    ]
                }
                21..=23 => {
                    // bottom row
                    vec![
                        above.get(17),
                        self.get(i - 1),
                        self.get(i + 1),
                        self.get(i - 5),
                    ]
                }
                _ if i % 5 == 0 => {
                    // left column
                    vec![
                        above.get(11),
                        self.get(i + 1),
                        self.get(i - 5),
                        self.get(i + 5),
                    ]
                }
                _ if i % 5 == 4 => {
                    // right column
                    vec![
                        above.get(13),
                        self.get(i - 1),
                        self.get(i - 5),
                        self.get(i + 5),
                    ]
                }
                12 => {
                    // middle cell
                    vec![false; 4]
                }
                7 => {
                    // top of middle cell
                    vec![
                        self.get(i - 1),
                        self.get(i + 1),
                        self.get(i - 5),
                        below.get(0),
                        below.get(1),
                        below.get(2),
                        below.get(3),
                        below.get(4),
                    ]
                }
                17 => {
                    // bottom of middle cell
                    vec![
                        self.get(i - 1),
                        self.get(i + 1),
                        self.get(i + 5),
                        below.get(20),
                        below.get(21),
                        below.get(22),
                        below.get(23),
                        below.get(24),
                    ]
                }
                11 => {
                    // left of middle cell
                    vec![
                        self.get(i - 1),
                        self.get(i + 5),
                        self.get(i - 5),
                        below.get(0),
                        below.get(5),
                        below.get(10),
                        below.get(15),
                        below.get(20),
                    ]
                }
                13 => {
                    // right of middle cell
                    vec![
                        self.get(i + 1),
                        self.get(i + 5),
                        self.get(i - 5),
                        below.get(4),
                        below.get(9),
                        below.get(14),
                        below.get(19),
                        below.get(24),
                    ]
                }
                _ => {
                    vec![
                        self.get(i + 1),
                        self.get(i - 1),
                        self.get(i + 5),
                        self.get(i - 5),
                    ]
                }
            }.into_iter().filter(|v| *v).count();

            next[i] = if has_bug {
                num_neighbours == 1
            } else {
                num_neighbours == 1 || num_neighbours == 2
            };
        }

        Level(next)
    }

    fn to_string(&self) -> String {
        (0..25).step_by(5)
            .map(|si| {
                (si..si + 5).map(|i| if i == 12 {
                    '?'
                } else if self.get(i) {
                    '#'
                } else {
                    '.'
                }).collect::<String>()
            })
            .join("\n")
    }

    fn num_bugs(&self) -> usize {
        self.0.iter().filter(|x| **x).count()
    }
}


impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct Map2D {
    levels: HashMap<i64, Level>,
}

impl Map2D {
    fn new(input: &str) -> Map2D {
        Map2D { levels: HashMap::from([(0, Level::from_input(input))]) }
    }

    fn evolve_one_step(&mut self) {
        // add 2 levels per iteration
        self.levels.insert(self.levels.keys().max().unwrap() + 1, Level::new());
        self.levels.insert(self.levels.keys().min().unwrap() - 1, Level::new());

        let mut next_levels = HashMap::new();
        for (&idx, level) in self.levels.iter() {
            let null_level = Level::new();
            let above = self.levels.get(&(idx + 1)).unwrap_or(&null_level);
            let below = self.levels.get(&(idx - 1)).unwrap_or(&null_level);

            let next = level.evolve(above, below);
            next_levels.insert(idx, next);
        }

        self.levels = next_levels;
    }

    fn evolve(&mut self, n: usize) {
        for _ in 0..n {
            self.evolve_one_step();
        }
    }

    fn count_bugs(&self) -> usize {
        self.levels.values().fold(0, |acc, v| acc + v.num_bugs())
    }
}


pub fn solve_b() {
    let mut map = Map2D::new(PUZZLE_INPUT);
    map.evolve(200);

    let ans = map.count_bugs();
    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use super::{Eris, Level, Map1D, Map2D};

    #[test]
    fn test_biodiversity_score() {
        let input = ".....
.....
.....
#....
.#...";
        let map = Map1D::new(input);
        assert_eq!(map.biodiversity_score(), 2129920);
    }

    #[test]
    fn test_evolve() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let mut map = Map1D::new(input);

        for (i, exp) in ["#..#.
####.
###.#
##.##
.##..", "#####
....#
....#
...#.
#.###", "#....
####.
...##
#.##.
.##.#", "####.
....#
##..#
.....
##..."].into_iter().enumerate() {
            map.evolve();

            let exp_map = Map1D::new(exp);
            assert_eq!(map.map, exp_map.map, "Evolution {} not similar", i + 1);
        }
    }

    #[test]
    fn test_get_first_repeated_score() {
        let input = "....#
#..#.
#..##
..#..
#....";
        let mut eris = Eris::new(input);
        let ans = eris.get_first_repeated_map();

        assert_eq!(ans, 2129920);
    }

    #[test]
    fn test_map2d_evolve() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let mut map = Map2D::new(input);
        map.evolve(10);

        let expected = HashMap::from([
            (-5, "####.
#..#.
#.?#.
####.
....."),
            (-4, ".###.
#..#.
#.?..
##.#.
....."),
            (-3, "..###
.....
#.?..
#....
#...#"),
            (-2, "###..
##.#.
#.?..
.#.##
#.#.."),
            (-1, ".##..
#..##
..?.#
##.##
#####"),
            (0, ".#...
.#.##
.#?..
.....
....."),
            (1, "#..##
...##
..?..
...#.
.####"),
            (2, ".#.##
....#
..?.#
...##
.###."),
            (3, "#.#..
.#...
..?..
.#...
#.#.."),
            (4, "...#.
...##
..?..
...##
...#."),
            (5, "..#..
.#.#.
..?.#
.#.#.
..#..")]);

        for depth in map.levels.keys().sorted() {
            let level = map.levels.get(depth).unwrap();
            if !level.is_empty() {
                let exp = Level::from_input(*expected.get(depth).unwrap()).to_string();
                assert_eq!(level.to_string(), exp, "Depth: {}", depth);
            }
        }
    }

    #[test]
    fn test_map2d_count_bugs() {
        let input = "....#
#..#.
#..##
..#..
#....";

        let mut map = Map2D::new(input);
        map.evolve(10);

        assert_eq!(map.count_bugs(), 99);
    }
}