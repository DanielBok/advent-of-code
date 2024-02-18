use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::inputs::read_content;

enum Dance {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn get_dance_moves(input: &str) -> Vec<Dance> {
    let spin_re = Regex::new(r"s(\d+)").expect("Could not compile SPIN regex");
    let exc_re = Regex::new(r"x(\d+)/(\d+)").expect("Could not compile EXCHANGE regex");
    let partner_re = Regex::new(r"p([a-z])/([a-z])").expect("Could not compile PARTNER regex");

    input.split(",")
        .map(|line| {
            if let Some(cap) = spin_re.captures(line) {
                Dance::Spin(cap[1].parse().unwrap())
            } else if let Some(cap) = exc_re.captures(line) {
                Dance::Exchange(
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                )
            } else if let Some(cap) = partner_re.captures(line) {
                Dance::Partner(
                    cap[1].chars().next().unwrap(),
                    cap[2].chars().next().unwrap(),
                )
            } else {
                panic!("Could not capture: '{}'", line)
            }
        })
        .collect()
}

fn get_initial_state() -> HashMap<usize, char> {
    "abcdefghijklmnop".chars()
        .enumerate()
        .collect::<HashMap<_, _>>()
}

fn dance(moves: &Vec<Dance>, positions: &mut HashMap<usize, char>) {
    let n = positions.len();

    for m in moves {
        match m {
            Dance::Spin(i) => {
                let old_pos = positions.clone();

                for (p, v) in old_pos {
                    positions.insert((p + i) % n, v);
                }
            }
            Dance::Exchange(i, j) => {
                if i != j {
                    let ci = *positions.get(&i).unwrap();
                    let cj = *positions.get(&j).unwrap();

                    positions.insert(*i, cj);
                    positions.insert(*j, ci);
                }
            }
            Dance::Partner(ci, cj) => {
                if ci != cj {
                    let mut i = n;
                    let mut j = n;

                    for (&p, &c) in positions.iter() {
                        if c == *ci {
                            i = p;
                        } else if c == *cj {
                            j = p;
                        }
                    }

                    positions.insert(i, *cj);
                    positions.insert(j, *ci);
                }
            }
        }
    }
}

fn position_hash(positions: &HashMap<usize, char>) -> String {
    (0..positions.len()).map(|i| positions.get(&i).unwrap()).join("")
}

pub fn solve_a() {
    let moves = get_dance_moves(&read_content(16));

    let mut positions = get_initial_state();
    dance(&moves, &mut positions);

    let ans = position_hash(&positions);

    assert_eq!(&ans, "hmefajngplkidocb");
    println!("Solution A: {}", ans)
}

pub fn solve_b() {
    let moves = get_dance_moves(&read_content(16));
    let mut positions = get_initial_state();
    let ans = get_order(&moves, &mut positions, 1_000_000_000);

    println!("Solution B: {}", ans);
}

fn get_order(moves: &Vec<Dance>, positions: &mut HashMap<usize, char>, limit: usize) -> String {
    let mut seen: HashMap<String, usize> = HashMap::new();

    for i in 1..=limit {
        dance(moves, positions);
        let hash = position_hash(&positions);

        // not the most optimal if (limit % delta == 0)
        if let Some(&k) = seen.get(&hash) {
            let delta = i - k;
            let offset = limit % delta;

            for (k, v) in &seen {
                if *v == offset {
                    return k.to_string();
                }
            }
        } else {
            seen.insert(hash, i);
        }
    }

    position_hash(&positions)
}
