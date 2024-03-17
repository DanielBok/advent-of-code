use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::inputs::read_contents;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Point(i32, i32);

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point(value.0 as i32, value.1 as i32)
    }
}

impl Point {
    fn neighbours(&self) -> [Point; 4] {
        [
            Point(self.0 + 1, self.1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0, self.1 - 1),
        ]
    }
}

fn get_input_map(input: &str) -> HashMap<Point, usize> {
    let mut map = HashMap::new();

    input.lines()
         .enumerate()
         .for_each(|(row_no, line)| {
             line.chars()
                 .enumerate()
                 .for_each(|(col_no, c)| {
                     let height = c.to_digit(10).unwrap() as usize;
                     map.insert(Point::from((col_no, row_no)), height);
                 })
         });

    map
}

pub fn solve_a() {
    let point_map = get_input_map(&read_contents(9));

    let mut risk = 0;
    for (point, height) in point_map.iter() {
        if point.neighbours().iter().all(|nb| {
            point_map.get(nb).map_or(true, |nb_height| nb_height > height)
        }) {
            risk += height + 1;
        }
    }

    println!("Solution A: {}", risk);
}

pub fn solve_b() {
    let point_map = get_input_map(&read_contents(9));
    let mut unseen: HashSet<Point> = point_map.iter()
                                              .filter_map(|(p, height)| if *height < 9 { Some(p) } else { None })
                                              .cloned()
                                              .collect();

    let mut seen: HashSet<Point> = HashSet::new();
    let mut basin_sizes = Vec::new();

    while let Some(first_point) = unseen.iter().next() {
        let mut basin_size = 0;
        let mut queue = VecDeque::from([first_point.clone()]);
        while let Some(pt) = queue.pop_front() {
            if !seen.insert(pt.clone()) { continue; }
            basin_size += 1;

            for p in pt.neighbours() {
                if !seen.contains(&p) && point_map.get(&p).map_or(false, |v| *v < 9) {
                    queue.push_back(p);
                }
            }
        }
        basin_sizes.push(basin_size);
        unseen.retain(|p| !seen.contains(p));
    }

    let ans = basin_sizes.iter()
                         .sorted()
                         .rev()
                         .take(3)
                         .product::<usize>();

    println!("Solution B: {}", ans);
}