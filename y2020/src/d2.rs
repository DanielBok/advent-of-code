use regex::{Captures, Regex};

use crate::inputs::read_contents;

trait Password {
    fn new(cap: regex::Captures) -> Self;
    fn is_valid(&self) -> bool;
}

struct PasswordA {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password for PasswordA {
    fn new(cap: Captures) -> Self {
        let min = cap.get(1).unwrap().as_str().parse().unwrap();
        let max = cap.get(2).unwrap().as_str().parse().unwrap();

        let letter = cap.get(3).unwrap().as_str().chars().next().unwrap();
        let password = cap.get(4).unwrap().as_str().to_string();

        PasswordA { min, max, letter, password }
    }


    fn is_valid(&self) -> bool {
        let num_letter = self.password.chars()
            .filter(|c| *c == self.letter)
            .count();

        num_letter >= self.min && num_letter <= self.max
    }
}

fn get_inputs<T: Password>(inputs: &str) -> Vec<T> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    inputs.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            T::new(cap)
        })
        .collect()
}

pub fn solve_a() {
    let passwords = get_inputs::<PasswordA>(&read_contents(2));

    let ans = passwords.iter()
        .filter(|p| p.is_valid())
        .count();

    println!("Solution A: {}", ans);
}

struct PasswordB {
    pos1: usize,
    pos2: usize,
    letter: char,
    password: Vec<char>,
}

impl Password for PasswordB {
    fn new(cap: Captures) -> Self {
        let pos1 = cap.get(1).unwrap().as_str().parse().unwrap();
        let pos2 = cap.get(2).unwrap().as_str().parse().unwrap();

        let letter = cap.get(3).unwrap().as_str().chars().next().unwrap();
        let password = cap.get(4).unwrap().as_str().chars().collect();

        PasswordB { pos1, pos2, letter, password }
    }

    fn is_valid(&self) -> bool {
        match (self.password.get(self.pos1 - 1), self.password.get(self.pos2 - 1)) {
            (Some(c1), Some(c2)) => (*c1 == self.letter || *c2 == self.letter) && (c1 != c2),
            (_, _) => false
        }
    }
}


pub fn solve_b() {
    let passwords = get_inputs::<PasswordB>(&read_contents(2));

    let ans = passwords.iter()
        .filter(|p| p.is_valid())
        .count();

    println!("Solution B: {}", ans);
}