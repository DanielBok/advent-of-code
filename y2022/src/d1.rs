use std::cmp::max;
use std::collections::BinaryHeap;
use crate::inputs::read_contents;

fn get_calories(input: &str) -> Vec<Vec<usize>> {
    input.replace("\r", "")
         .split("\n\n")
         .map(|lines| {
             lines.lines()
                  .map(|line| line.parse().unwrap())
                  .collect()
         })
         .collect()
}

pub fn solve_a() {
    let ans = get_calories(&read_contents(1))
        .iter()
        .fold(0, |highest, input| max(input.iter().sum(), highest));

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let mut heap: BinaryHeap<usize> = BinaryHeap::new();
    get_calories(&read_contents(1))
        .iter()
        .for_each(|v| {
            heap.push(v.iter().sum());
        });

    let ans = heap.pop().unwrap() +
        heap.pop().unwrap() +
        heap.pop().unwrap();

    println!("Solution B: {}", ans);
}