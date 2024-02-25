use std::collections::HashMap;

use itertools::Itertools;

use crate::inputs::read_content;

enum Arg {
    Char(char),
    Integer(i64),
}

impl Arg {
    fn new(x: &str) -> Self {
        if x.len() == 1 {
            let v = x.chars().next().unwrap();
            if v.is_alphabetic() {
                return Arg::Char(v);
            }
        }

        Arg::Integer(x.parse().unwrap())
    }

    fn get_value(&self, register: &HashMap<char, i64>) -> i64 {
        match self {
            Arg::Char(c) => *register.get(c).unwrap(),
            Arg::Integer(v) => *v
        }
    }
}

enum Instruction {
    Set(char, Arg),
    Sub(char, Arg),
    Mul(char, Arg),
    Jnz(Arg, Arg),
}

fn form_instructions(input: String) -> Vec<Instruction> {
    fn get_char(x: &str) -> char {
        assert_eq!(x.len(), 1);
        x.chars().next().unwrap()
    }

    input.lines()
        .map(|line| {
            let (cmd, a1, a2) = line.trim()
                .split(" ")
                .collect_tuple::<(&str, &str, &str)>()
                .unwrap();


            match cmd {
                "set" => Instruction::Set(get_char(a1), Arg::new(a2)),
                "sub" => Instruction::Sub(get_char(a1), Arg::new(a2)),
                "mul" => Instruction::Mul(get_char(a1), Arg::new(a2)),
                "jnz" => Instruction::Jnz(Arg::new(a1), Arg::new(a2)),
                _ => panic!("Invalid command: {}", cmd)
            }
        })
        .collect()
}

fn run_instructions(register: &mut HashMap<char, i64>,
                    instructions: &Vec<Instruction>) -> usize {
    let mut i = 0;

    let mut count = 0;
    while i < instructions.len() {
        let ins = &instructions[i];
        match ins {
            Instruction::Set(c, arg) |
            Instruction::Sub(c, arg) |
            Instruction::Mul(c, arg) => {
                let value = arg.get_value(&register);
                let err = format!("Expect a char but got '{}'. {:?}", c, register);
                let src = register.get_mut(c).expect(&err);

                match ins {
                    Instruction::Set(_, _) => { *src = value; }
                    Instruction::Sub(_, _) => { *src -= value; }
                    Instruction::Mul(_, _) => {
                        *src *= value;
                        count += 1;
                    }
                    Instruction::Jnz(_, _) => {}
                }

                i += 1;
            }
            Instruction::Jnz(a1, a2) => {
                if a1.get_value(&register) == 0 {
                    i += 1;
                } else {
                    let v = a2.get_value(&register);
                    if v < 0 {
                        let v = v.abs() as usize;
                        i -= v;
                    } else {
                        i += v as usize;
                    }
                }
            }
        }
    }

    count
}

pub fn solve_a() {
    let instructions = form_instructions(read_content(23));

    let mut register = HashMap::new();
    for c in 'a'..='h' {
        register.insert(c, 0);
    }

    let ans = run_instructions(&mut register, &instructions);
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let mut b = 81 * 100 + 100_000;
    let c = b + 17_000;
    let d = 2;
    let mut g = -1;  // any number
    let mut h = 0;
    
    while g != 0 {
        let mut f = 1;
    
        for i in d.. {
            if i * i >= b { break; }
            if b % i == 0 {
                f = 0;
                break;
            }
        }
    
        if f == 0 {
            h += 1;
        }
    
        g = b - c;
        b += 17;
    }

    println!("Solution B: {}", h);
}
