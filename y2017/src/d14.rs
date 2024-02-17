use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;
use rayon::prelude::*;

use crate::d10::{format_input, knot_hash};

const PUZZLE_INPUT: &str = "uugsqrei";

pub fn solve_a() {
    let total: usize = (0..128).into_par_iter()
        .map(|i| {
            let row = form_row(&format!("{}-{}", PUZZLE_INPUT, i));
            row.chars().filter(|e| *e == '1').count()
        })
        .sum();

    println!("Solution A: {}", total)
}

fn form_row(input: &str) -> String {
    let inp = format_input(input);
    let hash = knot_hash(&inp);

    hash.chars()
        .map(|c|
            format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<String>()
}

pub fn solve_b() {
    let map = form_map();
    let ans = count_regions(&map);

    println!("Solution B: {}", ans);
}

fn form_map() -> Vec<Vec<char>> {
    let map = (0..128).into_par_iter()
        .map(|i| {
            form_row(&format!("{}-{}", PUZZLE_INPUT, i))
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();


    assert!(map.iter().all(|r| r.len() == 128));
    assert_eq!(map.len(), 128);

    map
}

fn count_regions(map: &Vec<Vec<char>>) -> usize {
    let mut unseen = (0..128)
        .cartesian_product(0..128)
        .collect::<BTreeSet<_>>();

    let mut group_num = 0;

    while let Some(pt) = unseen.pop_first() {
        let (x, y) = pt;

        if map[y][x] == '0' {
            continue;
        }

        group_num += 1;

        let mut queue = VecDeque::from([pt]);

        while let Some(curr) = queue.pop_front() {
            let (x, y) = curr;

            if map[y][x] == '1' {
                // add neighbours
                for (nx, ny) in [
                    (x.checked_sub(1), Some(y)),
                    (Some(x + 1), Some(y)),
                    (Some(x), y.checked_sub(1)),
                    (Some(x), Some(y + 1)),
                ] {
                    if let (Some(nx), Some(ny)) = (nx, ny) {
                        let next_pt = (nx, ny);

                        if unseen.contains(&next_pt) {
                            queue.push_back(next_pt);
                        }
                    }
                }
            }
            unseen.remove(&curr);
        }
    }

    group_num
}