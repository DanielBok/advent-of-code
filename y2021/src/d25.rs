use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use aoc_macros::hashmap;

use crate::inputs::read_contents;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum SeaCucumber {
    East,
    South,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point(usize, usize);

struct Grid {
    location: HashMap<SeaCucumber, Vec<Point>>,
    occupied: HashSet<Point>,
    row_lim: usize,
    col_lim: usize,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut location = hashmap!(
            SeaCucumber::East => vec![],
            SeaCucumber::South => vec![],
        );
        let mut occupied = HashSet::new();
        let row_lim = input.lines().count();
        let col_lim = input.lines().next().unwrap().len();

        for (row_num, line) in input.lines().enumerate() {
            for (col_num, c) in line.chars().enumerate() {
                match c {
                    '>' => {
                        let pt = Point(col_num, row_num);
                        location.entry(SeaCucumber::East).and_modify(|v| v.push(pt));
                        occupied.insert(pt);
                    }
                    'v' => {
                        let pt = Point(col_num, row_num);
                        location.entry(SeaCucumber::South).and_modify(|v| v.push(pt));
                        occupied.insert(pt);
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }

        Grid { location, occupied, row_lim, col_lim }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Grid {
    fn to_string(&self) -> String {
        let mut map = (0..self.row_lim)
            .fold(HashMap::new(), |mut acc, y| {
                for x in 0..self.col_lim {
                    acc.insert((x, y), '.');
                }
                acc
            });

        for (dir, points) in &self.location {
            for p in points {
                match dir {
                    SeaCucumber::East => {
                        map.insert((p.0, p.1), '>');
                    }
                    SeaCucumber::South => {
                        map.insert((p.0, p.1), 'v');
                    }
                }
            }
        }

        let mut rows = vec![];
        for y in 0..self.row_lim {
            let mut row = vec![];
            for x in 0..self.col_lim {
                row.push(map[&(x, y)]);
            }
            rows.push(row.iter().collect::<String>());
        }

        rows.join("\n")
    }

    fn next_grid(&self) -> Option<Grid> {
        let mut has_change = false;
        let mut location = hashmap!(
            SeaCucumber::East => vec![],
            SeaCucumber::South => vec![],
        );

        let mut next_occupied = HashSet::new();

        for dir in [
            SeaCucumber::East,
            SeaCucumber::South,
        ] {
            let locations = &self.location[&dir];
            let south_occupied = match dir {
                SeaCucumber::East => { HashSet::new() }
                SeaCucumber::South => {
                    let v = self.location[&SeaCucumber::South].iter().cloned().collect::<HashSet<_>>();
                    next_occupied
                        .union(&v)
                        .cloned()
                        .collect()
                }
            };

            for pt in locations {
                match dir {
                    SeaCucumber::East => {
                        let next_pt = if pt.0 + 1 == self.col_lim { Point(0, pt.1) } else { Point(pt.0 + 1, pt.1) };
                        if !self.occupied.contains(&next_pt) {
                            next_occupied.insert(next_pt);
                            location.entry(dir).and_modify(|v| v.push(next_pt));
                            has_change = true;
                        } else {
                            next_occupied.insert(*pt);
                            location.entry(dir).and_modify(|v| v.push(*pt));
                        }
                    }
                    SeaCucumber::South => {
                        let next_pt = if pt.1 + 1 == self.row_lim { Point(pt.0, 0) } else { Point(pt.0, pt.1 + 1) };

                        if !south_occupied.contains(&next_pt) {
                            next_occupied.insert(next_pt);
                            location.entry(dir).and_modify(|v| v.push(next_pt));
                            has_change = true;
                        } else {
                            next_occupied.insert(*pt);
                            location.entry(dir).and_modify(|v| v.push(*pt));
                        }
                    }
                };
            }
        }

        if has_change {
            Some(Grid {
                location,
                occupied: next_occupied,
                row_lim: self.row_lim,
                col_lim: self.col_lim,
            })
        } else {
            None
        }
    }

    fn get_stop_step(&self) -> usize {
        let mut count = 1;
        let mut next_grid = self.next_grid();
        while let Some(grid) = next_grid {
            count += 1;
            next_grid = grid.next_grid();
        }

        count
    }
}


pub fn solve_a() {
    let grid = Grid::from(read_contents(25).as_str());
    let ans = grid.get_stop_step();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    println!("Completed AOC 2021")
}

#[cfg(test)]
mod tests {
    use crate::d25::Grid;

    const TEST_INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_next_grid() {
        for (times, exp) in [
            (1, "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"),
            (2, ">.v.v>>..v
v.v.>>vv..
>v>.>.>.v.
>>v>v.>v>.
.>..v....v
.>v>>.v.v.
v....v>v>.
.vv..>>v..
v>.....vv."),
            (3, "v>v.v>.>v.
v...>>.v.v
>vv>.>v>..
>>v>v.>.v>
..>....v..
.>.>v>v..v
..v..v>vv>
v.v..>>v..
.v>....v.."),
            (4, "v>..v.>>..
v.v.>.>.v.
>vv.>>.v>v
>>.>..v>.>
..v>v...v.
..>>.>vv..
>.v.vv>v.v
.....>>vv.
vvv>...v.."),
            (5, "vv>...>v>.
v.v.v>.>v.
>.v.>.>.>v
>v>.>..v>>
..v>v.v...
..>.>>vvv.
.>...v>v..
..v.v>>v.v
v.v.>...v."),
            (10, "..>..>>vv.
v.....>>.v
..v.v>>>v>
v>.>v.>>>.
..v>v.vv.v
.v.>>>.v..
v.v..>v>..
..v...>v.>
.vv..v>vv."),
            (20, "v>.....>>.
>vv>.....v
.>v>v.vv>>
v>>>v.>v.>
....vv>v..
.v.>>>vvv.
..v..>>vv.
v.v...>>.v
..v.....v>"),
            (30, ".vv.v..>>>
v>...v...>
>.v>.>vv.>
>v>.>.>v.>
.>..v.vv..
..v>..>>v.
....v>..>v
v.v...>vv>
v.v...>vvv"),
            (40, ">>v>v..v..
..>>v..vv.
..>>>v.>.v
..>>>>vvv>
v.....>...
v.v...>v>>
>vv.....v>
.>v...v.>v
vvv.v..v.>"),
            (50, "..>>v>vv.v
..v.>>vv..
v.>>v>>v..
..>>>>>vv.
vvv....>vv
..v....>>>
v>.......>
.vv>....v>
.>v.vv.v.."),
            (55, "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv...>..>
>vv.....>.
.>v.vv.v.."),
            (56, "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv....>.>
>vv......>
.>v.vv.v.."),
            (57, "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v.."),
        ] {
            let mut grid = Grid::from(TEST_INPUT);
            for _ in 0..times {
                grid = grid.next_grid().expect(&format!("Failed at {}", times));
            }

            assert_eq!(grid.to_string(), exp, "Iteration {} does not match", times)
        }
    }

    #[test]
    fn test_get_stop_step() {
        let grid = Grid::from(TEST_INPUT);
        let ans = grid.get_stop_step();

        assert_eq!(ans, 58)
    }
}