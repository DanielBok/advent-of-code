use std::ops::AddAssign;

use itertools::Itertools;

use crate::inputs::read_content;

fn get_inputs() -> Vec<String> {
    read_content(11)
        .split(",")
        .map(|w| w.to_string())
        .collect_vec()
}

#[derive(Eq, PartialEq, Debug)]
struct Point(i32, i32);


impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Point {
    fn new() -> Point { Point(0, 0) }

    fn move_point(&mut self, direction: &str) {
        match direction {
            "n" => {
                *self += Point(0, 2);
            }
            "nw" => {
                *self += Point(-2, 1);
            }
            "ne" => {
                *self += Point(2, 1);
            }
            "s" => {
                *self += Point(0, -2);
            }
            "sw" => {
                *self += Point(-2, -1);
            }
            "se" => {
                *self += Point(2, -1);
            }
            _ => {
                panic!("Invalid direction: {}", direction);
            }
        }
    }

    fn distance_from_origin(&self) -> usize {
        let x = self.0.abs() as usize;
        let y = self.1.abs() as usize;

        let horizontal = x / 2;

        let vertical = if y > horizontal {
            (y - horizontal) / 2
        } else {
            0
        };

        horizontal + vertical
    }
}

pub fn solve_a() {
    let directions = get_inputs();

    let mut point = Point::new();

    for dir in directions.iter() {
        point.move_point(dir);
    }

    let ans = point.distance_from_origin();
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let directions = get_inputs();
    let mut point = Point::new();

    let mut furthest = 0;

    for dir in directions.iter() {
        point.move_point(dir);

        let dist = point.distance_from_origin();
        if dist > furthest {
            furthest = dist;
        }
    }

    println!("Solution A: {}", furthest);
}

#[cfg(test)]
mod tests {
    use crate::d11::Point;

    #[test]
    fn test_point_movement() {
        for (d, exp) in [
            ("n", Point(0, 2)),
            ("nw", Point(-2, 1)),
            ("ne", Point(2, 1)),
            ("s", Point(0, -2)),
            ("sw", Point(-2, -1)),
            ("se", Point(2, -1))
        ] {
            let mut point = Point::new();
            point.move_point(d);

            assert_eq!(point, exp);
        }
    }

    #[test]
    fn test_distance_from_origin() {
        for (dirs, exp_dist, exp_point) in [
            ("ne,ne,ne", 3, Point(6, 3)),
            ("ne,ne,sw,sw", 0, Point(0, 0)),
            ("ne,ne,s,s", 2, Point(4, -2)),
            ("se,sw,se,sw,sw", 3, Point(-2, -5)),
            ("ne,se,ne,se", 4, Point(8, 0)),
            ("ne,se,ne,se,ne,ne", 6, Point(12, 2)),
            ("ne,se,ne,se,ne,ne,n,n", 6, Point(12, 6)),
            ("ne,se,ne,se,ne,ne,n,n,n", 7, Point(12, 8)),
        ] {
            let mut point = Point::new();

            for dir in dirs.split(",") {
                point.move_point(dir);
            }

            assert_eq!(point, exp_point);
            assert_eq!(point.distance_from_origin(), exp_dist);
        }
    }
}