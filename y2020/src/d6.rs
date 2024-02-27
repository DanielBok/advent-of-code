use std::collections::HashSet;

use crate::inputs::read_contents;

pub fn solve_a() {
    let ans = read_contents(6).replace("\r\n", "\n")
        .split("\n\n")
        .map(|lines| {
            lines.trim().replace("\n", "").chars().collect::<HashSet<_>>().len()
        })
        .sum::<usize>();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let ans = read_contents(6).replace("\r\n", "\n")
        .split("\n\n")
        .map(|lines| {
            lines.trim().lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|acc, s| acc.intersection(&s).cloned().collect::<HashSet<_>>())
                .unwrap()
                .len()
        })
        .sum::<usize>();

    println!("Solution B: {}", ans);
}