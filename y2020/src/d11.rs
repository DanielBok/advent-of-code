use std::collections::HashMap;

use crate::inputs::read_contents;

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn adjacent(&self, xlim: i32, ylim: i32) -> Vec<Point> {
        let mut nbs = Vec::new();
        for dy in -1..=1 {
            let y = self.1 + dy;
            if y < 0 || y > ylim { continue; }

            for dx in -1..=1 {
                let x = self.0 + dx;
                if (dx == 0 && dy == 0) || x < 0 || x > xlim { continue; }
                nbs.push(Point(x, y));
            }
        }

        nbs
    }

    fn move_point(&self, dx: i32, dy: i32, xlim: i32, ylim: i32) -> Option<Point> {
        let y = self.1 + dy;
        let x = self.0 + dx;

        if x < 0 || x > xlim || y < 0 || y > ylim { None } else { Some(Point(x, y)) }
    }
}


trait Grid {
    fn new(input: &str) -> Self;
    fn next_state(&mut self);
    fn num_occupied_seats(&self) -> usize;
}

struct GridA {
    grid: HashMap<Point, State>,
    xlim: i32,
    ylim: i32,
    occupied_seats: usize,
}

impl Grid for GridA {
    fn new(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut xlim = 0;
        let mut ylim = 0;

        let mut occupied_seats = 0;
        for (y, line) in input.lines().enumerate() {
            if y > ylim { ylim = y; }

            for (x, c) in line.chars().enumerate() {
                if x > xlim { xlim = x; }

                grid.insert(Point(x as i32, y as i32), match c {
                    'L' => State::Empty,
                    '.' => State::Floor,
                    '#' => {
                        occupied_seats += 1;
                        State::Occupied
                    }
                    _ => { panic!("Invalid state: '{}'", c) }
                });
            }
        }

        Self { grid, xlim: xlim as i32, ylim: ylim as i32, occupied_seats }
    }

    fn next_state(&mut self) {
        let mut next_grid = HashMap::new();

        let mut occupied_seats = 0;
        for (pt, curr_state) in self.grid.iter() {
            let next_state = match curr_state {
                State::Floor => { *curr_state }
                State::Empty => {
                    if pt.adjacent(self.xlim, self.ylim)
                        .iter()
                        .all(|np| matches!(self.grid.get(np).unwrap(), State::Empty | State::Floor )) {
                        State::Occupied
                    } else { *curr_state }
                }
                State::Occupied => {
                    if pt.adjacent(self.xlim, self.ylim)
                        .iter()
                        .filter(|np| matches!(self.grid.get(np).unwrap(), State::Occupied))
                        .count() >= 4 {
                        State::Empty
                    } else { *curr_state }
                }
            };

            if matches!(next_state, State::Occupied) {
                occupied_seats += 1;
            }

            next_grid.insert(*pt, next_state);
        }
        self.grid = next_grid;
        self.occupied_seats = occupied_seats;
    }

    fn num_occupied_seats(&self) -> usize {
        self.occupied_seats
    }
}


pub fn solve_a() {
    let mut grid = GridA::new(&read_contents(11));
    let ans = long_run_grid_occupancy_count(&mut grid, 1_000_000);
    println!("Solution A: {}", ans);
}

fn long_run_grid_occupancy_count<T: Grid>(grid: &mut T, limit: usize) -> usize {
    let mut last_count = grid.num_occupied_seats();
    for _ in 0..limit {
        grid.next_state();
        if last_count == grid.num_occupied_seats() {
            return last_count;
        } else {
            last_count = grid.num_occupied_seats();
        }
    }
    panic!("Could not find solution after running {} iterations", limit);
}

pub fn solve_b() {
    let mut grid = GridB::new(&read_contents(11));
    let ans = long_run_grid_occupancy_count(&mut grid, 1000);
    println!("Solution B: {}", ans);
}

struct GridB {
    grid: HashMap<Point, State>,
    occupied_seats: usize,
    neighbors: HashMap<Point, Vec<Point>>,
}

impl Grid for GridB {
    fn new(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut neighbors = HashMap::new();
        let mut xlim = 0;
        let mut ylim = 0;

        let mut occupied_seats = 0;

        for (y, line) in input.lines().enumerate() {
            if y > ylim { ylim = y; }

            for (x, c) in line.chars().enumerate() {
                if x > xlim { xlim = x; }

                grid.insert(Point(x as i32, y as i32), match c {
                    'L' => { State::Empty }
                    '.' => State::Floor,
                    '#' => {
                        occupied_seats += 1;
                        State::Occupied
                    }
                    _ => { panic!("Invalid state: '{}'", c) }
                });
            }
        };
        let xlim = xlim as i32;
        let ylim = ylim as i32;

        for (pt, state) in grid.iter() {
            match state {
                State::Empty | State::Occupied => {
                    let mut pt_nb = Vec::new();

                    for (dx, dy) in [
                        (-1, -1),   // move top left
                        (0, -1),    // move top
                        (1, -1),    // move top right
                        (-1, 0),    // move left
                        (1, 0),     // move right
                        (-1, 1),    // move bottom left
                        (0, 1),     // move bottom
                        (1, 1),     // move bottom right
                    ] {
                        let mut np = pt.clone();
                        while let Some(p) = np.move_point(dx, dy, xlim, ylim) {
                            np = p;
                            if matches!(grid.get(&np).unwrap(), State::Empty | State::Occupied) {
                                pt_nb.push(np);
                                break;
                            }
                        }
                    };

                    neighbors.insert(*pt, pt_nb);
                }
                _ => { continue; }
            }
        }

        Self { grid, occupied_seats, neighbors }
    }

    fn next_state(&mut self) {
        let mut next_grid = HashMap::new();
        let mut occupied_seats = 0;
        for (pt, curr_state) in self.grid.iter() {
            let next_state = match curr_state {
                State::Empty => {
                    if self.neighbors.get(pt).unwrap()
                        .iter().all(|np| matches!(self.grid.get(np).unwrap(), State::Empty)) {
                        State::Occupied
                    } else {
                        *curr_state
                    }
                }
                State::Occupied => {
                    if self.neighbors.get(pt).unwrap()
                        .iter()
                        .filter(|np| matches!(self.grid.get(np).unwrap(), State::Occupied))
                        .count() >= 5 {
                        State::Empty
                    } else {
                        *curr_state
                    }
                }
                _ => { *curr_state }
            };

            if matches!(next_state, State::Occupied) {
                occupied_seats += 1;
            }

            next_grid.insert(*pt, next_state);
        }
        self.grid = next_grid;
        self.occupied_seats = occupied_seats;
    }

    fn num_occupied_seats(&self) -> usize {
        self.occupied_seats
    }
}


#[cfg(test)]
mod tests {
    use super::{Grid, GridA, GridB, long_run_grid_occupancy_count, Point, State};

    impl GridB {
        fn print(&self) {
            let (xlim, ylim) = self.grid.iter().fold((0, 0), |(mut xlim, mut ylim), (p, _)| {
                if p.0 > xlim { xlim = p.0; }
                if p.1 > ylim { ylim = p.1; }

                (xlim, ylim)
            });

            let grid = (0..=ylim).map(|y| (0..=xlim).map(|x| {
                match self.grid.get(&Point(x, y)).unwrap() {
                    State::Floor => '.',
                    State::Empty => 'L',
                    State::Occupied => '#'
                }
            })
                .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n");

            println!("{}\n", grid);
        }
    }

    #[test]
    fn test_grid_a_long_run_grid_occupancy_count() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut grid = GridA::new(input);

        let ans = long_run_grid_occupancy_count(&mut grid, 1000);
        assert_eq!(ans, 37);
    }

    #[test]
    fn test_grid_b_long_run_grid_occupancy_count() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut grid = GridB::new(input);
        grid.print();


        let ans = long_run_grid_occupancy_count(&mut grid, 1000);
        assert_eq!(ans, 26);
    }
}