use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn get_inputs() -> Vec<usize> {
    "10	3	15	10	5	15	5	15	9	2	5	8	5	2	3	6"
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect()
}

fn hash_inputs(inputs: &Vec<usize>) -> String {
    inputs.iter().map(|x| x.to_string()).join(".")
}

fn get_max_index_and_value(inputs: &Vec<usize>) -> (usize, usize) {
    inputs.iter()
        .enumerate()
        .fold((0, 0), |(aidx, av), (idx, &v)| {
            if v > av {
                (idx, v)
            } else {
                (aidx, av)
            }
        })
}

fn redistribute_blocks(inputs: &mut Vec<usize>, idx: usize, blocks: usize) {
    let mut idx = idx;
    let mut blocks = blocks;

    inputs[idx] = 0;
    while blocks > 0 {
        idx = (idx + 1) % 16;
        inputs[idx] += 1;
        blocks -= 1;
    }
}

pub fn solve_a() {
    let mut inputs = get_inputs();
    let mut seen = HashSet::from([hash_inputs(&inputs)]);

    let mut cycles = 0;
    loop {
        cycles += 1;

        let (idx, blocks) = get_max_index_and_value(&inputs);
        redistribute_blocks(&mut inputs, idx, blocks);

        if !seen.insert(hash_inputs(&inputs)) {
            break;
        }
    };

    println!("Solution A: {}", cycles);
}


pub fn solve_b() {
    let mut inputs = get_inputs();
    let mut seen = HashMap::from([(hash_inputs(&inputs), 0)]);

    let mut cycle = 0;
    let ans = loop {
        cycle += 1;

        let (idx, blocks) = get_max_index_and_value(&inputs);
        redistribute_blocks(&mut inputs, idx, blocks);

        if let Some(prev_cycle) = seen.insert(hash_inputs(&inputs), cycle) {
            break cycle - prev_cycle;
        }
    };

    println!("Solution B: {}", ans);
}