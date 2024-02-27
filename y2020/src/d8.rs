use std::collections::HashSet;

use crate::inputs::read_contents;

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Noop(i32),
    Jump(i32),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let (op, value) = line.split_once(" ").unwrap();
            let value = value.parse().expect(&format!("Could not parse '{}' to integer", value));

            match op {
                "acc" => Instruction::Acc(value),
                "nop" => Instruction::Noop(value),
                "jmp" => Instruction::Jump(value),
                _ => panic!("Invalid op: '{}'", op)
            }
        })
        .collect()
}

pub fn solve_a() {
    let instructions = parse_instructions(&read_contents(8));
    let mut i = 0;
    let mut acc = 0;

    let mut seen = HashSet::new();

    while !seen.contains(&i) {
        seen.insert(i);
        match &instructions[i] {
            Instruction::Acc(v) => {
                acc += v;
                i += 1;
            }
            Instruction::Noop(_) => { i += 1; }
            Instruction::Jump(v) => {
                if v.is_positive() {
                    i += *v as usize;
                } else {
                    i -= v.abs() as usize
                }
            }
        }
    }

    println!("Solution A: {}", acc);
}

fn get_no_cycle_accumulation(instructions: &Vec<Instruction>) -> Option<i32> {
    let mut i = 0;
    let mut acc = 0;

    let mut seen = HashSet::new();

    while i < instructions.len() {
        if seen.contains(&i) {
            return None;
        }
        seen.insert(i);
        match &instructions[i] {
            Instruction::Acc(v) => {
                acc += v;
                i += 1;
            }
            Instruction::Noop(_) => { i += 1; }
            Instruction::Jump(v) => {
                if v.is_positive() {
                    i += *v as usize;
                } else {
                    i -= v.abs() as usize
                }
            }
        }
    }

    Some(acc)
}


pub fn solve_b() {
    let instructions = parse_instructions(&read_contents(8));

    let ans = 'outer: loop {
        for i in 0..instructions.len() {
            if let Some(ins) = instructions.get(i) {
                let mut instructions = (&instructions).clone();
                match ins {
                    Instruction::Noop(v) if v.is_positive() => {
                        instructions[i] = Instruction::Jump(*v);
                    }
                    Instruction::Jump(v) if v.is_negative() => {
                        instructions[i] = Instruction::Noop(*v);
                    }
                    _ => { continue; }
                }

                if let Some(ans) = get_no_cycle_accumulation(&instructions) {
                    break 'outer ans;
                }
            }
        }
    };

    println!("Solution B: {}", ans);
}