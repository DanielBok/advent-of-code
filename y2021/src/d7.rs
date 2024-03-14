use std::cmp::min;

use itertools::{Itertools, MinMaxResult};

use crate::inputs::read_contents;

fn parse_positions(input: &str) -> Vec<usize> {
    input.split(",")
         .map(|line| line.parse().unwrap())
         .collect()
}

fn get_position_with_min_fuel_requirement<F>(positions: &Vec<usize>, fuel_func: F) -> usize
    where F: Fn(&Vec<usize>, usize) -> usize
{
    let (start, end) = match positions.iter().minmax() {
        MinMaxResult::MinMax(x, y) => (*x, *y),
        _ => panic!("Should have two unique start and end")
    };

    (start..=end).fold(usize::MAX, |acc, p| min(fuel_func(positions, p), acc))
}

pub fn solve_a() {
    let positions = parse_positions(&read_contents(7));

    let ans = get_position_with_min_fuel_requirement(&positions, get_fuel_required);
    println!("Solution A: {}", ans);
}

fn get_fuel_required(positions: &Vec<usize>, position: usize) -> usize {
    positions.iter()
             .map(|p| if *p >= position { p - position } else { position - p })
             .sum()
}

pub fn solve_b() {
    let positions = parse_positions(&read_contents(7));

    let ans = get_position_with_min_fuel_requirement(&positions, get_growing_fuel_required);
    println!("Solution A: {}", ans);
}

fn get_growing_fuel_required(positions: &Vec<usize>, position: usize) -> usize {
    positions.iter()
             .map(|&p| {
                 let diff = if p >= position { p - position } else { position - p };

                 if diff == 0 { 0 } else { diff * (diff + 1) / 2 }
             })
             .sum()
}