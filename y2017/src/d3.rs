use std::cmp::max;
use std::collections::HashMap;

const PUZZLE_INPUT: usize = 368078;

pub fn solve_a() {
    let mut level: usize = 1;
    while level.pow(2) < PUZZLE_INPUT {
        level += 2;
    }
    let mut last_value = level.pow(2);
    let n = (level + 1) / 2;  // this is the closest distance from the center to the ring

    // moving around the ring anti-clockwise
    while last_value - level + 1 > PUZZLE_INPUT {
        last_value -= level + 1;
    }
    let mid = last_value + 1 - (level + 1) / 2;
    let ans = if mid > PUZZLE_INPUT { mid - PUZZLE_INPUT } else { PUZZLE_INPUT - mid } + n - 1;

    println!("Solution A {}", ans)
}


#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point(i64, i64);

impl Point {
    fn level(&self) -> i64 {
        max(self.0.abs(), self.1.abs())
    }

    fn neighbors(&self) -> [Point; 4] {
        [
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 0),
        ].map(|(dx, dy)| Point(self.0 + dx, self.1 + dy))
    }

    fn full_neighbours(&self) -> Vec<Point> {
        [self.neighbors(),
            [
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ].map(|(dx, dy)| Point(self.0 + dx, self.1 + dy))
        ].concat()
    }
}


pub fn solve_b() {
    let mut points = HashMap::from([(Point(0, 0), 1_usize)]);

    let mut level = 1;
    let mut last_point = Point(0, 0);
    let ans = loop {
        let next = if level < last_point.level() {
            Point(last_point.0 + 1, last_point.1)
        } else {
            'search: {
                for next in last_point.neighbors() {
                    if next.level() == level && !points.contains_key(&next) {
                        break 'search next;
                    }
                };
                panic!("Could not find next point");
            }
        };

        if next.0 == next.1 && next.0 == level {
            level += 1;
        }

        let value = next.full_neighbours()
            .iter()
            .fold(0, |acc, p| acc + points.get(&p).unwrap_or(&0));

        if value > PUZZLE_INPUT {
            break value;
        }


        points.insert(next, value);
        last_point = next;

    };

    // println!("{:?}", points);
    println!("Solution B: {}", ans);
}