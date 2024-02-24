use std::collections::HashMap;

use crate::inputs::read_content;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn move_point(&mut self, face: &Facing) {
        match face {
            Facing::Up => { self.y -= 1; }
            Facing::Down => { self.y += 1; }
            Facing::Left => { self.x -= 1; }
            Facing::Right => { self.x += 1 }
        }
    }
}

#[derive(Debug)]
enum GridState {
    Clean,
    Infected,
    Weakened,
    Flagged,
}


impl GridState {
    fn new(c: char) -> Self {
        match c {
            '.' => GridState::Clean,
            '#' => GridState::Infected,
            _ => panic!("Invalid GridState character: {}", c)
        }
    }
}

#[derive(Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

trait Virus {
    fn new() -> Self;
    fn point(&self) -> Point;

    fn set_point(&mut self, point: Point);

    fn single_step(&mut self, state: &mut GridState) -> usize;
    fn turn(&mut self, turn: Turn);

    fn forward(&mut self);
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
    Reverse,
}

struct VirusA {
    p: Point,
    face: Facing,
}


impl Virus for VirusA {
    fn new() -> Self {
        VirusA { p: Point { x: 0, y: 0 }, face: Facing::Up }
    }

    fn point(&self) -> Point {
        self.p.clone()
    }

    fn set_point(&mut self, point: Point) {
        self.p = point;
    }

    fn single_step(&mut self, state: &mut GridState) -> usize {
        let n = match state {
            GridState::Clean => {
                *state = GridState::Infected;
                self.turn(Turn::Left);
                1
            }
            GridState::Infected => {
                *state = GridState::Clean;
                self.turn(Turn::Right);
                0
            }
            _ => panic!("Virus A does not handle state: {:?}", state)
        };

        self.forward();
        n
    }

    fn turn(&mut self, turn: Turn) {
        self.face = match turn {
            Turn::Left =>
                match self.face {
                    Facing::Up => Facing::Left,
                    Facing::Left => Facing::Down,
                    Facing::Down => Facing::Right,
                    Facing::Right => Facing::Up
                }
            Turn::Right =>
                match self.face {
                    Facing::Up => Facing::Right,
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                },
            _ => panic!("Virus A does not implement turn type: {:?}", turn)
        };
    }

    fn forward(&mut self) {
        self.p.move_point(&self.face);
    }
}

struct VirusB {
    p: Point,
    face: Facing,
}

impl Virus for VirusB {
    fn new() -> Self {
        VirusB { p: Point { x: 0, y: 0 }, face: Facing::Up }
    }

    fn point(&self) -> Point {
        self.p.clone()
    }

    fn set_point(&mut self, point: Point) {
        self.p = point;
    }

    fn single_step(&mut self, state: &mut GridState) -> usize {
        let n = match state {
            GridState::Clean => {
                *state = GridState::Weakened;
                self.turn(Turn::Left);
                0
            }
            GridState::Weakened => {
                *state = GridState::Infected;
                1
            }
            GridState::Infected => {
                *state = GridState::Flagged;
                self.turn(Turn::Right);
                0
            }
            GridState::Flagged => {
                *state = GridState::Clean;
                self.turn(Turn::Reverse);
                0
            }
        };

        self.forward();
        n
    }

    fn turn(&mut self, turn: Turn) {
        self.face = match turn {
            Turn::Left =>
                match self.face {
                    Facing::Up => Facing::Left,
                    Facing::Left => Facing::Down,
                    Facing::Down => Facing::Right,
                    Facing::Right => Facing::Up,
                }
            Turn::Right =>
                match self.face {
                    Facing::Up => Facing::Right,
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                }
            Turn::Reverse => {
                match self.face {
                    Facing::Up => Facing::Down,
                    Facing::Left => Facing::Right,
                    Facing::Down => Facing::Up,
                    Facing::Right => Facing::Left,
                }
            }
        };
    }

    fn forward(&mut self) {
        self.p.move_point(&self.face);
    }
}

struct InfiniteGrid<V: Virus> {
    grid: HashMap<Point, GridState>,
    virus: V,
    infections: usize,
}

impl<V: Virus> InfiniteGrid<V> {
    fn new(input: &str, virus: V) -> Self {
        let mut grid = HashMap::new();
        let input = input.trim();
        let n = input.lines().count() as i64;

        let mut virus = virus;
        virus.set_point(Point { x: n / 2, y: n / 2 });

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.insert(Point { x: x as i64, y: y as i64 }, GridState::new(c));
            }
        }

        InfiniteGrid { virus, grid, infections: 0 }
    }

    fn single_step(&mut self) {
        let state = self.grid.entry(self.virus.point())
            .or_insert(GridState::Clean);

        self.infections += self.virus.single_step(state);
    }

    fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.single_step();
        }
    }
}


pub fn solve_a() {
    let mut grid = InfiniteGrid::new(&read_content(22), VirusA::new());
    grid.run(10000);

    println!("Solution A: {}", grid.infections);
}

pub fn solve_b() {
    let mut grid = InfiniteGrid::new(&read_content(22), VirusB::new());
    grid.run(10_000_000);

    println!("Solution B: {}", grid.infections);
}


#[cfg(test)]
mod tests {
    use super::{GridState, InfiniteGrid, Point, Virus, VirusA, VirusB};

    impl GridState {
        fn to_char(&self) -> char {
            match self {
                GridState::Clean => '.',
                GridState::Infected => '#',
                GridState::Flagged => 'F',
                GridState::Weakened => 'W'
            }
        }
    }

    impl<V: Virus> InfiniteGrid<V> {
        fn print_grid(&self) {
            let mut min_x = 0;
            let mut max_x = 0;
            let mut min_y = 0;
            let mut max_y = 0;

            for &Point { x, y } in self.grid.keys() {
                if x < min_x { min_x = x };
                if x > max_x { max_x = x };
                if y < min_y { min_y = y };
                if y > max_y { max_y = y };
            }

            let mut output = Vec::new();
            for y in min_y..=max_y {
                let mut row = Vec::new();
                for x in min_x..=max_x {
                    row.push(match self.grid.get(&Point { x, y }) {
                        Some(s) => s.to_char(),
                        None => '.'
                    });
                }
                output.push(row.iter().collect::<String>());
            }
            println!("{}\n", output.join("\n"));
        }
    }

    #[test]
    fn test_virus_a() {
        let inp = "..#
#..
...";
        for (n, exp) in [
            (7, 5),
            (70, 41),
            (10000, 5587)
        ] {
            let mut grid = InfiniteGrid::new(inp, VirusA::new());
            grid.run(n);
            
            if n < 100 {
                grid.print_grid();
            }
            assert_eq!(grid.infections, exp);
        }
    }
    
    #[test]
    fn test_virus_b() {
        let inp = "..#
#..
...";
        let mut grid = InfiniteGrid::new(inp, VirusB::new());
        grid.run(10_000_000);
        assert_eq!(grid.infections, 2511944);
    }
}