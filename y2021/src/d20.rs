use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use crate::inputs::read_contents;

#[derive(Eq, PartialEq, Hash)]
struct Point(isize, isize);

impl Point {
    fn surroundings(&self) -> [Point; 9] {
        [
            Point(self.0 - 1, self.1 - 1),
            Point(self.0, self.1 - 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1),
            Point(self.0 + 1, self.1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0, self.1 + 1),
            Point(self.0 + 1, self.1 + 1),
        ]
    }
}

struct Image {
    image: HashMap<Point, char>,
    enhancer: Vec<char>,
    alternate: bool,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.image
                                               .keys()
                                               .fold((isize::MAX, isize::MIN, isize::MAX, isize::MIN),
                                                     |(min_x, max_x, min_y, max_y), p| {
                                                         let min_x = if p.0 < min_x { p.0 } else { min_x };
                                                         let max_x = if p.0 > max_x { p.0 } else { max_x };
                                                         let min_y = if p.1 < min_y { p.0 } else { min_y };
                                                         let max_y = if p.1 > max_y { p.0 } else { max_y };

                                                         (min_x, max_x, min_y, max_y)
                                                     });

        let image = (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x).map(|x| self.image.get(&Point(x, y)).unwrap())
                               .collect::<String>()
            }).join("\n");

        write!(f, "{}", image)
    }
}

impl From<&str> for Image {
    fn from(input: &str) -> Self {
        let input = input.replace("\r", "");
        let (conversion, image) = input.split_once("\n\n").unwrap();

        let enhancer = conversion.chars().collect_vec();

        let image = image.lines()
                         .enumerate()
                         .fold(HashMap::new(), |mut acc, (rn, line)| {
                             for (cn, c) in line.chars().enumerate() {
                                 acc.insert(Point(cn as isize, rn as isize), c);
                             }
                             acc
                         });

        let alternate = enhancer[0] == '#';
        Self { enhancer, image, alternate }
    }
}

impl Image {
    fn current_image_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();

        for p in self.image.keys() {
            points.extend(p.surroundings());
        }

        points
    }

    fn evolve(&mut self, times: usize) {
        for i in 0..times {
            self.step_once(i);
        }
    }

    fn step_once(&mut self, step: usize) {
        let mut next_image = HashMap::new();

        for pt in self.current_image_points() {
            let state = self.get_next_substate(&pt, step);
            next_image.insert(pt, state);
        }

        self.image = next_image;
    }

    fn get_next_substate(&self, point: &Point, step: usize) -> char {
        let mut v: usize = 0;
        for (i, p) in point.surroundings().iter().enumerate() {
            let c = match self.image.get(p) {
                None => {
                    if self.alternate {
                        if step % 2 == 0 { self.enhancer[511] } else { self.enhancer[0] }
                    } else {
                        '.'
                    }
                }
                Some(c) => *c,
            };
            if c == '#' {
                v += 1 << (8 - i);
            }
        }
        self.enhancer[v]
    }

    fn num_lit_pixels(&self) -> usize {
        self.image
            .values()
            .filter(|c| **c == '#')
            .count()
    }
}


pub fn solve_a() {
    let mut image = Image::from(read_contents(20).as_str());
    image.evolve(2);

    let ans = image.num_lit_pixels();
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let mut image = Image::from(read_contents(20).as_str());
    image.evolve(50);

    let ans = image.num_lit_pixels();
    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::Image;

    const TEST_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_evolve() {
        for (steps, exp) in [
            (2, 35),
            (50, 3351),
        ] {
            let mut image = Image::from(TEST_INPUT);
            image.evolve(steps);
            assert_eq!(image.num_lit_pixels(), exp);
        }
        
        let mut image = Image::from(TEST_INPUT);
        image.evolve(2);
        assert_eq!(image.num_lit_pixels(), 35);
    }
}