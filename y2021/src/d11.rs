use std::collections::{HashMap, HashSet, VecDeque};

use indicatif::ProgressIterator;

use crate::inputs::read_contents;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn neighbours(&self) -> [Point; 8] {
        [
            Point(self.0 - 1, self.1 - 1),
            Point(self.0, self.1 - 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 - 1, self.1),
            Point(self.0 + 1, self.1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0, self.1 + 1),
            Point(self.0 + 1, self.1 + 1),
        ]
    }
}

struct Grid {
    map: HashMap<Point, usize>,
    flashes: usize,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let map = input.lines()
                       .enumerate()
                       .fold(HashMap::new(), |mut map, (row_no, line)| {
                           for (col_no, energy) in line.chars().enumerate() {
                               let energy = energy.to_digit(10).unwrap();
                               map.insert(Point(col_no as i32, row_no as i32), energy as usize);
                           }
                           map
                       });

        Grid { map, flashes: 0 }
    }
}

impl Grid {
    fn run_till_synchronized(&mut self) -> usize {
        let mut count = 1;
        while self.step() != self.map.len() {
            count += 1;
        }

        count
    }

    fn run_steps(&mut self, steps: usize) {
        for _ in (0..steps).progress() {
            self.step();
        }
    }

    fn step(&mut self) -> usize {
        let mut flashed: HashSet<Point> = HashSet::new();
        let mut to_flash: VecDeque<Point> = VecDeque::new();

        for (point, energy) in self.map.iter_mut() {
            *energy += 1;
            if *energy > 9 {
                to_flash.push_back(*point);
            }
        }

        while let Some(pt) = to_flash.pop_front() {
            if !flashed.insert(pt) {
                continue;
            }

            for nb in pt.neighbours() {
                if flashed.contains(&nb) { continue; }
                if let Some(energy) = self.map.get_mut(&nb) {
                    *energy += 1;
                    if *energy > 9 {
                        to_flash.push_back(nb);
                    }
                }
            }
        }

        let num_flashes = flashed.len();

        self.flashes += num_flashes;
        flashed.iter()
               .for_each(|pt| {
                   *self.map.get_mut(pt).unwrap() = 0;
               });

        num_flashes
    }
}

pub fn solve_a() {
    let mut grid = Grid::from(read_contents(11).as_str());
    grid.run_steps(100);

    println!("Solution A: {}", grid.flashes);
}

pub fn solve_b() {
    let mut grid = Grid::from(read_contents(11).as_str());
    let ans = grid.run_till_synchronized();

    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use super::Grid;

    const SMALL_TEST_INPUT: &str = "11111
19991
19191
19991
11111";

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_step_small() {
        let mut grid = Grid::from(SMALL_TEST_INPUT);
        grid.step();

        assert_eq!(grid.flashes, 9);
    }

    #[test]
    fn test_steps() {
        for (steps, exp) in [(10, 204), (100, 1656)] {
            let mut grid = Grid::from(TEST_INPUT);
            grid.run_steps(steps);

            assert_eq!(grid.flashes, exp);
        }
    }

    #[test]
    fn test_synchronize() {
        let mut grid = Grid::from(TEST_INPUT);
        let steps = grid.run_till_synchronized();

        assert_eq!(steps, 195);
    }
}