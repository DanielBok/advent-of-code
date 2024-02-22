use std::collections::{HashMap, HashSet};

use crate::inputs::read_content;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn next(&self, dir: Direction) -> Option<Self> {
        let &Point(x, y) = self;

        match dir {
            Direction::UP => { if y == 0 { None } else { Some(Point(x, y - 1)) } }
            Direction::DOWN => { Some(Point(x, y + 1)) }
            Direction::LEFT => { if x == 0 { None } else { Some(Point(x - 1, y)) } }
            Direction::RIGHT => { Some(Point(x + 1, y)) }
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn get_setup() -> (HashMap<Point, char>, Point) {
    let mut map = HashMap::new();
    let mut start = None;
    for (y, line) in read_content(19).lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_whitespace() {
                map.insert(Point(x, y), c);
            }

            if y == 0 && c == '|' {
                start = Some(Point(x, y));
            }
        }
    }

    (map, start.unwrap())
}

fn next_step(map: &HashMap<Point, char>,
             point: Point,
             direction: Direction,
             seen: &HashSet<Point>) -> (Option<(Point, Direction)>, Option<char>) {
    let tile = map.get(&point).unwrap();

    let get_next = || {
        if let Some(next_pt) = point.next(direction) {
            if map.contains_key(&next_pt) {
                return Some((next_pt, direction));
            }
        }
        None
    };

    match tile {
        '|' | '-' => { (get_next(), None) }
        '+' => {
            let next = [Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT]
                .into_iter()
                .find_map(|d| {
                    if let Some(next_pt) = point.next(d) {
                        if map.contains_key(&next_pt) && !seen.contains(&next_pt) {
                            return Some((next_pt, d));
                        }
                    }
                    None
                });
            (next, None)
        }
        _ if tile.is_alphabetic() => {
            (get_next(), Some(*tile))
        }
        _ => { panic!("Invalid point encountered: {:?} {:?}", point, tile) }
    }
}

fn run_through_maze() -> (Vec<char>, usize) {
    let (map, start) = get_setup();
    let mut current = Some((start, Direction::DOWN));
    let mut seen = HashSet::from([start]);

    let mut order = Vec::new();
    let mut count = 0;
    
    while let Some((pt, dir)) = current {
        count += 1;
        seen.insert(pt);

        let (next, c) = next_step(&map, pt, dir, &seen);
        if let Some(c) = c {
            order.push(c);
        }

        current = next;
    }

    (order, count)
}

pub fn solve_a() {
    let (order, _) = run_through_maze();
    let ans = order.iter().collect::<String>();
    
    assert_eq!(&ans, "GEPYAWTMLK");
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let (_, ans) = run_through_maze();
    
    assert_eq!(ans, 17628);
    println!("Solution B: {}", ans);
}