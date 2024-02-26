use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use crate::inputs::read_contents;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Tree,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '#' => Tile::Tree,
            '.' => Tile::Empty,
            _ => { panic!("Invalid tile char: {}", c) }
        }
    }
}

fn setup(input: &str) -> (HashMap<Point, Tile>, usize, usize) {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Point(x, y), Tile::new(c));
        }
    }

    let max_y = input.lines().count() - 1;
    let max_x = input.lines().next().unwrap().len() - 1;

    (map, max_x, max_y)
}

pub fn solve_a() {
    let ans = count_number_trees_encountered(&read_contents(3), 3, 1);
    println!("Solution A: {}", ans);
}

fn count_number_trees_encountered(input: &str, x_offset: usize, y_offset: usize) -> usize {
    let (map, max_x, max_y) = setup(input);

    let mut count = 0;
    let mut point = Point(0, 0);

    while point.1 <= max_y {
        if let Some(tile) = map.get(&point) {
            if *tile == Tile::Tree {
                count += 1;
            }
        }

        point = Point((point.0 + x_offset) % (max_x + 1), point.1 + y_offset)
    }
    count
}

pub fn solve_b() {
    let input = Arc::new(read_contents(3));

    let thread_handles = [(1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)]
        .map(|(x_off, y_off)| {
            let input = input.clone();
            thread::spawn(move || count_number_trees_encountered(&input, x_off, y_off))
        })
        .into_iter()
        .collect::<Vec<_>>();

    let ans: usize = thread_handles.into_iter().map(|t| t.join().unwrap()).product();
    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use crate::d3::count_number_trees_encountered;

    #[test]
    fn test_() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        for (x_off, y_off, exp) in [
            (1, 1, 2),
            (3, 1, 7),
            (5, 1, 3),
            (7, 1, 4),
            (1, 2, 2),
        ] {
            let ans = count_number_trees_encountered(input, x_off, y_off);
            assert_eq!(ans, exp);
        }
    }
}