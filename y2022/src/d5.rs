use std::collections::HashMap;

use regex::Regex;

use aoc_macros::hashmap;

use crate::inputs::read_contents;

fn parse_input(inputs: &str) -> (HashMap<usize, Vec<char>>, Vec<CrateProcedure>) {
    let inputs = inputs.replace("\r", "");
    let (top, bottom) = inputs.split_once("\n\n").unwrap();
    let layout = build_layout(top);

    let procedures = parse_instructions(bottom);

    (layout, procedures)
}

fn build_layout(top: &str) -> HashMap<usize, Vec<char>> {
    let crates_layout = top.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = crates_layout.len();
    let mut layout: HashMap<usize, Vec<char>> = hashmap!();

    crates_layout
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, c)| {
            if c.is_digit(10) {
                let pos = c.to_digit(10).unwrap() as usize;
                let order = crates_layout.iter()
                    .take(height - 1)
                    .rev()
                    .filter_map(|vec| {
                        if i < vec.len() && vec[i].is_alphabetic() {
                            Some(vec[i])
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                layout.insert(pos, order);
            }
        });

    layout
}

struct CrateProcedure {
    qty: usize,
    from: usize,
    to: usize,
}

fn parse_instructions(bottom: &str) -> Vec<CrateProcedure> {
    let pat = Regex::new(r"move (?<amt>\d+) from (?<from>\d) to (?<to>\d)").unwrap();
    bottom.lines()
        .map(|line| {
            let caps = pat.captures(line).unwrap();

            CrateProcedure {
                qty: caps.name("amt").unwrap().as_str().parse::<usize>().unwrap(),
                from: caps.name("from").unwrap().as_str().parse::<usize>().unwrap(),
                to: caps.name("to").unwrap().as_str().parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn move_crates_9000(layout: &mut HashMap<usize, Vec<char>>, procedures: &Vec<CrateProcedure>) {
    for p in procedures {
        let from_col = layout.get_mut(&p.from).unwrap();
        let split_point = from_col.len().saturating_sub(p.qty);
        let items_to_move = from_col.split_off(split_point);

        let to_col = layout.get_mut(&p.to).unwrap();
        to_col.extend(items_to_move.iter().rev());
    }
}

fn get_crate_code(layout: &HashMap<usize, Vec<char>>) -> String {
    (0..layout.len())
        .map(|i| {
            layout.get(&(i + 1)).unwrap().last().unwrap()
        })
        .collect::<String>()
}

pub fn solve_a() {
    let (mut layout, procedures) = parse_input(&read_contents(5));
    move_crates_9000(&mut layout, &procedures);

    let ans = get_crate_code(&layout);
    println!("Solution A: {}", ans);
}

fn move_crates_9001(layout: &mut HashMap<usize, Vec<char>>, procedures: &Vec<CrateProcedure>) {
    for p in procedures {
        let from_col = layout.get_mut(&p.from).unwrap();
        let split_point = from_col.len().saturating_sub(p.qty);
        let items_to_move = from_col.split_off(split_point);

        let to_col = layout.get_mut(&p.to).unwrap();
        to_col.extend(items_to_move);
    }
}

pub fn solve_b() {
    let (mut layout, procedures) = parse_input(&read_contents(5));
    move_crates_9001(&mut layout, &procedures);

    let ans = get_crate_code(&layout);
    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use crate::d5::{get_crate_code, parse_input, move_crates_9000, move_crates_9001};

    fn test_input<'a>() -> &'a str {
        "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    #[test]
    fn test_move_crates_9000() {
        let (mut layout, procedures) = parse_input(test_input());
        move_crates_9000(&mut layout, &procedures);

        let code = get_crate_code(&layout);
        assert_eq!(&code, "CMZ");
    }

    #[test]
    fn test_move_crates_9001() {
        let (mut layout, procedures) = parse_input(test_input());
        move_crates_9001(&mut layout, &procedures);

        let code = get_crate_code(&layout);
        assert_eq!(&code, "MCD");
    }
}