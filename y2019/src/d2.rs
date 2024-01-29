use std::ops::{Index, IndexMut};

use crate::int_code::{CommandMap, ParameterMode};

const DEFAULT_COMMAND: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,6,19,23,1,10,23,27,2,27,13,31,1,31,6,35,2,6,35,39,1,39,5,43,1,6,43,47,2,6,47,51,1,51,5,55,2,55,9,59,1,6,59,63,1,9,63,67,1,67,10,71,2,9,71,75,1,6,75,79,1,5,79,83,2,83,10,87,1,87,5,91,1,91,9,95,1,6,95,99,2,99,10,103,1,103,5,107,2,107,6,111,1,111,5,115,1,9,115,119,2,119,10,123,1,6,123,127,2,13,127,131,1,131,6,135,1,135,10,139,1,13,139,143,1,143,13,147,1,5,147,151,1,151,2,155,1,155,5,0,99,2,0,14,0";

pub fn solve_a() {
    let mut program = IntCodeProgram::from_str(DEFAULT_COMMAND);
    program[1] = 12;
    program[2] = 2;

    program.run();
    assert_eq!(6327510, program[0]);
    println!("Solution A: {}", program[0]);
}

pub fn solve_b() {
    let target = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = IntCodeProgram::from_str(DEFAULT_COMMAND);
            program[1] = noun;
            program[2] = verb;
            program.run();

            if program[0] == target {
                let ans = program[1] * 100 + program[2];

                assert_eq!(ans, 4112);
                println!("Solution B: {}", ans);
                return;
            }
        }
    }

    panic!("Could not find solution")
}


struct IntCodeProgram {
    command: CommandMap,
}

impl IntCodeProgram {
    pub fn from_str(input: &str) -> IntCodeProgram {
        let command = input.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .enumerate()
            .collect::<CommandMap>();

        IntCodeProgram {
            command
        }
    }

    pub fn run(&mut self) {
        let mut idx = 0;

        loop {
            let op = self.command.get(&idx).unwrap();

            match op {
                1 | 2 => {
                    let v1 = self.get_value(ParameterMode::Position, idx + 1);
                    let v2 = self.get_value(ParameterMode::Position, idx + 2);
                    let pos = self.get_value(ParameterMode::Immediate, idx + 3);

                    let value = match op {
                        1 => v1 + v2,
                        2 => v1 * v2,
                        _ => 0,  // this is never going to happen!
                    };

                    self.save_value(pos as usize, value);
                    idx += 4;
                }
                99 => {
                    break;
                }
                _ => panic!("Invalid op code {op}")
            }
        }
    }

    fn get_value(&self, mode: ParameterMode, idx: usize) -> i64 {
        match mode {
            ParameterMode::Position => {
                let pos = *self.command.get(&idx).unwrap() as usize;
                *self.command.get(&pos).unwrap()
            }
            ParameterMode::Immediate => {
                *self.command.get(&idx).unwrap()
            }
            _ => { panic!("Unhandled parameter mode: {mode:?}") }
        }
    }

    fn save_value(&mut self, pos: usize, value: i64) {
        self.command.insert(pos, value);
    }
}

impl Index<usize> for IntCodeProgram {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.command.get(&index).unwrap()
    }
}

impl IndexMut<usize> for IntCodeProgram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.command.get_mut(&index).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::IntCodeProgram;

    impl IntCodeProgram {
        pub fn as_vector(&self) -> Vec<i64> {
            let max_value = *self.command.keys().max().unwrap();

            (0..=max_value).map(|k| *self.command.get(&k).unwrap_or(&0))
                .collect::<Vec<_>>()
        }
    }

    #[test]
    fn test_run_program() {
        for (inp, expected) in [
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ] {
            let mut program = IntCodeProgram::from_str(inp);
            program.run();

            assert_eq!(program.as_vector()
                           .iter().map(|x| x.to_string())
                           .collect::<Vec<String>>()
                           .join(","),
                       expected);
        }
    }
}