use std::collections::HashSet;

use crate::inputs::read_contents;

struct Rucksack<'a> {
    original: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(value: &'a str) -> Self {
        let (left, right) = value.split_at(value.len() / 2);
        Self { original: value, left, right }
    }
}


fn get_rucksacks(input: &str) -> Vec<Rucksack> {
    input.lines()
         .map(Rucksack::from)
         .collect()
}

fn get_priority(c: char) -> usize {
    (if c.is_ascii_uppercase() {
        c as u8 - 64 + 26
    } else {
        c as u8 - 96
    }) as usize
}

pub fn solve_a() {
    let input = read_contents(3);
    let rucksacks = get_rucksacks(&input);

    let ans: usize = rucksacks.iter()
                              .map(|s| {
                                  let first: HashSet<char> = HashSet::from_iter(s.left.chars());

                                  if let Some(c) = s.right.chars().find(|c| first.contains(c)) {
                                      get_priority(c)
                                  } else {
                                      unreachable!()
                                  }
                              })
                              .sum();
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let input = read_contents(3);
    let rucksacks = get_rucksacks(&input);

    let ans: usize = (0..rucksacks.len())
        .step_by(3)
        .map(|i| {
            let s1: HashSet<char> = HashSet::from_iter(rucksacks[i].original.chars());
            let s2: HashSet<char> = HashSet::from_iter(rucksacks[i + 1].original.chars());
            let s3: HashSet<char> = HashSet::from_iter(rucksacks[i + 2].original.chars());

            let common = s1.iter().find(|c| s2.contains(c) && s3.contains(c)).unwrap();
            get_priority(*common)
        })
        .sum();

    println!("Solution B: {}", ans);
}
