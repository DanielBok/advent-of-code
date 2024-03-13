use std::collections::HashMap;

use crate::inputs::read_contents;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point(i32, i32);

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        Point(x.parse().unwrap(), y.parse().unwrap())
    }
}

impl Point {
    fn gradient(&self, other: &Point) -> (i32, i32) {
        if self.0 == other.0 {
            return (0, if self.1 < other.1 { 1 } else { -1 });
        } else if self.1 == other.1 {
            return (if self.0 < other.0 { 1 } else { -1 }, 0);
        } else {
            assert_eq!((self.0 - other.0).abs(), (self.1 - other.1).abs());
            (if self.0 < other.0 { 1 } else { -1 },
             if self.1 < other.1 { 1 } else { -1 })
        }
    }
}

enum LineType {
    Vertical,
    Horizontal,
    Diagonal,
}

struct VentLine {
    // p1: Point,
    // p2: Point,
    line: Vec<Point>,
    line_type: LineType,
}

impl VentLine {
    fn new(p1: Point, p2: Point) -> Self {
        let mut start_x = p1.0;
        let mut start_y = p1.1;

        let mut line = vec![Point(start_x, start_y)];

        let (gx, gy) = p1.gradient(&p2);

        let line_type = match (gx, gy) {
            (0, 0) => panic!("Line is going nowhere! Gradient is (0, 0)"),
            (0, _) => LineType::Horizontal,
            (_, 0) => LineType::Vertical,
            _ => LineType::Diagonal,
        };

        while start_x != p2.0 || start_y != p2.1 {
            start_x += gx;
            start_y += gy;
            line.push(Point(start_x, start_y));
        }

        VentLine { line, line_type }
    }
}

fn parse_input(input: &str) -> Vec<VentLine> {
    input.lines()
         .map(|line| {
             let (p1, p2) = line.trim().split_once(" -> ").unwrap();
             let p1 = Point::from(p1);
             let p2 = Point::from(p2);

             VentLine::new(p1, p2)
         })
         .collect()
}

fn count_vent_overlaps<F>(vent_lines: &Vec<VentLine>,
                          filter_func: F) -> HashMap<Point, usize>
    where F: Fn(&VentLine) -> bool {
    let mut overlaps = HashMap::new();

    for vent_line in vent_lines {
        if filter_func(vent_line) {
            for p in vent_line.line.iter() {
                overlaps.entry(p.clone())
                        .and_modify(|v| { *v += 1; })
                        .or_insert(1);
            }
        }
    }

    overlaps
}

pub fn solve_a() {
    let vent_lines = parse_input(&read_contents(5));
    let overlaps = count_vent_overlaps(&vent_lines, keep_non_diagonals);

    let ans = overlaps.into_iter()
                      .filter(|(_, count)| *count >= 2)
                      .count();

    println!("Solution A: {}", ans);
}

fn keep_non_diagonals(v: &VentLine) -> bool {
    !matches!(v.line_type, LineType::Diagonal)
}

pub fn solve_b() {
    let vent_lines = parse_input(&read_contents(5));
    let overlaps = count_vent_overlaps(&vent_lines, |_| true);
    let ans = overlaps.into_iter()
                      .filter(|(_, count)| *count >= 2)
                      .count();

    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{count_vent_overlaps, keep_non_diagonals, parse_input, Point, VentLine};

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_vent_line() {
        for (start, end, exp) in [
            ((0, 9), (5, 9), vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)]),
            ((5, 9), (0, 9), vec![(5, 9), (4, 9), (3, 9), (2, 9), (1, 9), (0, 9)]),
            ((0, 4), (0, 8), vec![(0, 4), (0, 5), (0, 6), (0, 7), (0, 8)]),
            ((0, 8), (0, 4), vec![(0, 8), (0, 7), (0, 6), (0, 5), (0, 4)]),
            ((1, 1), (5, 5), vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]),
            ((5, 5), (1, 1), vec![(5, 5), (4, 4), (3, 3), (2, 2), (1, 1)]),
            ((5, 5), (8, 2), vec![(5, 5), (6, 4), (7, 3), (8, 2)]),
            ((8, 2), (5, 5), vec![(8, 2), (7, 3), (6, 4), (5, 5)]),
        ] {
            let v = VentLine::new(Point(start.0, start.1), Point(end.0, end.1));
            assert_eq!(v.line, exp.iter().map(|(s, e)| Point(*s, *e)).collect_vec());
        }
    }

    #[test]
    fn test_count_vent_overlaps() {
        let vent_lines = parse_input(TEST_INPUT);
        let overlaps = count_vent_overlaps(&vent_lines, keep_non_diagonals);

        let ans = overlaps.into_iter()
                          .filter(|(_, count)| *count >= 2)
                          .count();

        assert_eq!(ans, 5);

        let overlaps = count_vent_overlaps(&vent_lines, |_| true);
        let ans = overlaps.into_iter()
                          .filter(|(_, count)| *count >= 2)
                          .count();

        assert_eq!(ans, 12);
    }
}