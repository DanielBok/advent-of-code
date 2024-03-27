use std::cmp::{max, min};
use std::collections::BTreeSet;

use regex::Regex;

const PUZZLE_INPUT: &str = "target area: x=206..250, y=-105..-57";

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl From<&str> for Target {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"x=(?<x_min>-?\d+)..(?<x_max>-?\d+), y=(?<y_min>-?\d+)..(?<y_max>-?\d+)").unwrap();
        let caps = re.captures(input).unwrap();
        let parse_item = |name| caps.name(name).unwrap().as_str().parse::<i32>().unwrap();

        Target {
            x_min: parse_item("x_min"),
            x_max: parse_item("x_max"),
            y_min: parse_item("y_min"),
            y_max: parse_item("y_max"),
        }
    }
}

impl Target {
    fn highest_point(&self) -> i32 {
        let y = max(self.y_min.abs(), self.y_max.abs()) - 1;
        y * (y + 1) / 2
    }

    fn solve_quad(v: i32) -> f32 {
        (-1.0 + ((1 + 4 * 2 * v) as f32).sqrt()) / 2.0
    }

    fn num_initial_conditions(&self) -> i32 {
        let mut set = BTreeSet::new();
        let min_time = Self::solve_quad(self.x_min).ceil() as i32;
        let max_time = (
            Self::solve_quad({
                let y = min(self.y_min.abs(), self.y_max.abs()) - 1;
                y * (y + 1) / 2
            }) as i32 + 1
        ) * 2 - 1;
        let vy_limit = min(self.y_min.abs(), self.y_max.abs()) - 1;

        for t in 1..=max_time {
            let mut vxs = vec![];
            for vx in min_time..=self.x_max {
                let px = (0..t).fold((0, vx), |(p, v), _| (p + v, max(v - 1, 0))).0;
                if self.x_min <= px && px <= self.x_max {
                    vxs.push(vx);
                }
            }


            let mut vys = vec![];
            for vy in self.y_min..vy_limit {
                let py = (0..t)
                    .fold((0, vy), |(p, v), _| (p + v, v - 1)).0;

                if self.y_min <= py && py <= self.y_max {
                    vys.push(vy);
                }
            }

            for vx in vxs.iter() {
                for vy in vys.iter() {
                    set.insert((*vx, *vy));
                }
            }
        }

        let max_time = Self::solve_quad(self.x_max).floor() as i32;
        let ans = set.len() as i32 + (max_time - min_time + 1) * (self.y_max - self.y_min + 1);

        ans
    }
}

pub fn solve_a() {
    let target = Target::from(PUZZLE_INPUT);
    let ans = target.highest_point();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let target = Target::from(PUZZLE_INPUT);
    let ans = target.num_initial_conditions();

    println!("Solution B: {}", ans);
}