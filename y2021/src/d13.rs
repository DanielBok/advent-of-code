use std::cmp::{max, min};
use std::collections::HashSet;
use itertools::Itertools;
use crate::inputs::read_contents;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point(usize, usize);

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let input = input.replace("\r", "");

    let (top, bottom) = input.split_once("\n\n").unwrap();

    let points = top.lines()
                    .map(|line| {
                        let (x, y) = line.split_once(",").unwrap();
                        Point(x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect();

    let folds = bottom.lines()
                      .map(|line| {
                          let (c, v) = line.split_whitespace()
                                           .skip(2)
                                           .next()
                                           .expect(&format!("Could not split line '{}' to 3 parts", line))
                                           .split_once('=')
                                           .expect("Could not split past '=' sign");
                          match c {
                              "x" => Fold::X(v.parse().unwrap()),
                              "y" => Fold::Y(v.parse().unwrap()),
                              _ => panic!("Invalid line: '{}'", line)
                          }
                      })
                      .collect();

    (points, folds)
}

pub fn solve_a() {
    let (points0, folds) = parse_input(&read_contents(13));
    let points1 = fold_once(&points0, &folds[0]);

    let ans = points1.len();
    println!("Solution A: {}", ans);
}

fn get_boundaries(points: &HashSet<Point>) -> [usize; 4] {
    points.iter()
          .fold([usize::MAX, usize::MIN, usize::MAX, usize::MIN],
                |[min_x, max_x, min_y, max_y], p| {
                    let min_x = min(p.0, min_x);
                    let max_x = max(p.0, max_x);
                    let min_y = min(p.1, min_y);
                    let max_y = max(p.1, max_y);

                    [min_x, max_x, min_y, max_y]
                })
}

fn fold_once(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut next_points = HashSet::new();

    let [min_x, max_x, min_y, max_y] = get_boundaries(points);

    match *fold {
        Fold::X(fold_line) => {
            let midpoint = (max_x + min_x + 1) / 2;

            for p in points {
                if p.0 == midpoint { continue; } // points at midpoint are discarded

                let next_point = if fold_line > midpoint {
                    if p.0 < fold_line {
                        Point(p.0, p.1)
                    } else {
                        // move everything that is over midpoint to other side
                        Point(2 * fold_line - p.0, p.1)
                    }
                } else {  // fold_line < midpoint
                    // move everything that is under midpoint to other side and offset the x value
                    if p.0 > fold_line {
                        Point(p.0 - fold_line - 1, p.1)
                    } else {
                        Point(fold_line - p.0 - 1, p.1)
                    }
                };

                next_points.insert(next_point);
            }
        }
        Fold::Y(fold_line) => {
            let midpoint = (max_y + min_y + 1) / 2;
            for p in points {
                if p.1 == midpoint { continue; } // points at midpoint are discarded
                let next_point = if fold_line > midpoint {
                    if p.1 < fold_line {
                        Point(p.0, p.1)
                    } else {
                        // move everything that is over midpoint to other side
                        Point(p.0, 2 * fold_line - p.1)
                    }
                } else {  // fold_line < midpoint
                    // move everything that is under midpoint to other side and offset the y value
                    if p.1 > fold_line {
                        Point(p.0, p.1 - fold_line - 1)
                    } else {
                        Point(p.0, fold_line - p.1 - 1)
                    }
                };
                next_points.insert(next_point);
            }
        }
    }

    next_points
}

pub fn solve_b() {
    let (mut points, folds) = parse_input(&read_contents(13));
    for fold in folds.iter() {
        points = fold_once(&points, fold)
    }

    let ans = draw_points(&points);
    println!("Solution B: \n{}", ans);
}

fn draw_points(points: &HashSet<Point>) -> String {
    let [min_x, max_x, min_y, max_y] = get_boundaries(points);

    let n_cols = max_x - min_x + 1;

    let mut screen = vec![];
    for _ in min_y..=max_y {
        screen.push(vec![' '; n_cols]);
    }

    for p in points {
        let x = p.0 - min_x;
        let y = p.1 - min_y;
        screen[y][x] = '#';
    }

    screen.iter()
          .map(|row| row.iter().rev().collect::<String>())
          .rev()
          .join("\n")
}


#[cfg(test)]
mod tests {
    use super::{fold_once, parse_input};

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_fold_once() {
        let (points0, folds) = parse_input(TEST_INPUT);
        let points1 = fold_once(&points0, &folds[0]);

        assert_eq!(points1.len(), 17);

        let points2 = fold_once(&points1, &folds[1]);
        assert_eq!(points2.len(), 16);
    }
}