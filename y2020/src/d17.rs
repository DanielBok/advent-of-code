use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

const PUZZLE_INPUT: &str = ".......#
....#...
...###.#
#...###.
....##..
##.#..#.
###.#.#.
....#...";


enum State {
    Active,
    Inactive,
}

trait Point: PartialEq + Eq + Hash + Display + Clone {
    fn from_xy(x: i32, y: i32) -> Self;

    fn generate_neighbours(&self) -> Vec<Self>
        where Self: Sized;
}

struct Space<P: Point> {
    grid: HashMap<P, State>,
    neighbours: HashMap<P, Vec<P>>,
}

impl<P: Point> Space<P> {
    fn from_input(input: &str) -> Space<P> {
        let mut grid = HashMap::new();


        for (y, line) in input.lines().enumerate() {
            let offset = line.len() as i32;
            let y = y as i32 - offset;

            for (x, c) in line.chars().enumerate() {
                let x = x as i32 - offset;

                let state = match c {
                    '#' => State::Active,
                    '.' => State::Inactive,
                    _ => { panic!("Invalid char: {c}") }
                };

                grid.insert(P::from_xy(x, y), state);
            }
        }

        let neighbours = grid.keys()
                             .map(|pt| (pt.clone(), pt.generate_neighbours()))
                             .collect();

        Space { grid, neighbours }
    }

    fn expand_layer(&mut self) {
        let mut new_points = HashSet::new();

        // get all neighbouring points that are not in grid yet
        for neighbours in self.neighbours.values() {
            for nb in neighbours {
                if !self.grid.contains_key(nb) {
                    new_points.insert(nb.clone());
                }
            }
        }

        // iosert neighbouring points into grid and add their neighbours
        for pt in new_points {
            self.grid.insert(pt.clone(), State::Inactive);
            self.neighbours.insert(pt.clone(), pt.generate_neighbours());
        }
    }


    fn simulate_one_step(&mut self) {
        self.expand_layer();

        let mut next_grid = HashMap::new();

        for (pt, state) in self.grid.iter() {
            let neighbours = self.neighbours.get(pt).expect(&format!("{} does not have neighbours", pt));
            let num_active_nbs = neighbours.iter().filter(|nb|
                match self.grid.get(nb) {
                    None => false,
                    Some(state) => matches!(state, State::Active)
                }
            ).count();

            let next_state = match state {
                State::Active => {
                    if num_active_nbs == 2 || num_active_nbs == 3 { State::Active } else { State::Inactive }
                }
                State::Inactive => {
                    if num_active_nbs == 3 { State::Active } else { State::Inactive }
                }
            };

            next_grid.insert(pt.clone(), next_state);
        }

        self.grid = next_grid;
    }

    fn count_active_points(&self) -> usize {
        self.grid.values()
            .filter(|state| matches!(state, State::Active))
            .count()
    }

    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.simulate_one_step();
        }
    }
}


pub fn solve_a() {
    let mut space = Space::<Point3D>::from_input(PUZZLE_INPUT);
    space.simulate(6);
    let ans = space.count_active_points();

    println!("Solution A: {}", ans);
}

#[derive(Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point3D {}

impl Hash for Point3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x, self.y, self.z).hash(state)
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point3D({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point for Point3D {
    fn from_xy(x: i32, y: i32) -> Self {
        Point3D { x, y, z: 0 }
    }

    fn generate_neighbours(&self) -> Vec<Self> {
        let mut neighbours = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 { continue; }
                    neighbours.push(Point3D { x: self.x + dx, y: self.y + dy, z: self.z + dz })
                }
            }
        }
        neighbours
    }
}


impl Display for Space<Point3D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_z = i32::MAX;
        let mut max_z = i32::MIN;

        for pt in self.grid.keys() {
            if pt.x > max_x { max_x = pt.x; }
            if pt.x < min_x { min_x = pt.x; }
            if pt.y > max_y { max_y = pt.y; }
            if pt.y < min_y { min_y = pt.y; }
            if pt.z > max_z { max_z = pt.z; }
            if pt.z < min_z { min_z = pt.z; }
        }

        let mut message = vec![];

        for z in min_z..=max_z {
            let mut layer = vec![];

            for y in min_y..=max_y {
                layer.push((min_x..=max_x).map(|x| {
                    match self.grid.get(&Point3D { x, y, z }).unwrap() {
                        State::Active => '#',
                        State::Inactive => '.'
                    }
                }).collect::<String>());
            }

            message.push(format!("z = {}\n{}\n", z, layer.join("\n")));
        }


        write!(f, "{}", message.join("\n"))
    }
}


pub fn solve_b() {
    let mut space: Space<Point4D> = Space::from_input(PUZZLE_INPUT);
    space.simulate(6);
    let ans = space.count_active_points();

    println!("Solution B: {}", ans);
}

#[derive(Clone)]
struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl PartialEq for Point4D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Eq for Point4D {}

impl Hash for Point4D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x, self.y, self.z, self.w).hash(state)
    }
}

impl Display for Point4D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point4D({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Point for Point4D {
    fn from_xy(x: i32, y: i32) -> Self {
        Point4D { x, y, z: 0, w: 0 }
    }

    fn generate_neighbours(&self) -> Vec<Point4D> {
        let mut neighbours = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 { continue; }
                        neighbours.push(Point4D { x: self.x + dx, y: self.y + dy, z: self.z + dz, w: self.w + dw })
                    }
                }
            }
        }
        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::{Point3D, Point4D, Space};

    const TEST_INPUT: &str = ".#.
..#
###";

    #[test]
    fn test_space3d_print() {
        let mut space: Space<Point3D> = Space::from_input(TEST_INPUT);
        println!("{}", space);

        space.simulate_one_step();
        println!("{}", space);

        space.simulate_one_step();
        println!("{}", space);
    }

    #[test]
    fn test_space3d_simulate() {
        let mut space: Space<Point3D> = Space::from_input(TEST_INPUT);
        space.simulate(6);

        assert_eq!(space.count_active_points(), 112);
    }

    #[test]
    fn test_space4d_simulate() {
        let mut space: Space<Point4D> = Space::from_input(TEST_INPUT);
        space.simulate(6);

        assert_eq!(space.count_active_points(), 848);
    }
}