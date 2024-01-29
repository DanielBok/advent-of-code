use std::collections::VecDeque;

use crate::int_code::{CommandMap, decode_op, ParameterMode};

const DEFAULT_COMMAND: &str = "3,225,1,225,6,6,1100,1,238,225,104,0,1101,90,60,224,1001,224,-150,224,4,224,1002,223,8,223,1001,224,7,224,1,224,223,223,1,57,83,224,1001,224,-99,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1102,92,88,225,101,41,187,224,1001,224,-82,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,1101,7,20,225,1101,82,64,225,1002,183,42,224,101,-1554,224,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1102,70,30,224,101,-2100,224,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,2,87,214,224,1001,224,-2460,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,102,36,180,224,1001,224,-1368,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1102,50,38,225,1102,37,14,225,1101,41,20,225,1001,217,7,224,101,-25,224,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,1101,7,30,225,1102,18,16,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,226,226,224,102,2,223,223,1006,224,329,101,1,223,223,1107,677,226,224,102,2,223,223,1006,224,344,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,359,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,7,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,108,677,226,224,1002,223,2,223,1005,224,404,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,419,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,434,1001,223,1,223,1008,677,677,224,1002,223,2,223,1005,224,449,1001,223,1,223,1107,226,677,224,102,2,223,223,1006,224,464,101,1,223,223,107,226,677,224,1002,223,2,223,1006,224,479,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,494,1001,223,1,223,8,677,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1108,677,677,224,102,2,223,223,1005,224,524,1001,223,1,223,1108,226,677,224,1002,223,2,223,1005,224,539,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,554,1001,223,1,223,1007,226,226,224,102,2,223,223,1005,224,569,1001,223,1,223,1008,226,226,224,102,2,223,223,1005,224,584,101,1,223,223,1007,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,108,677,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,629,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,644,101,1,223,223,1107,226,226,224,1002,223,2,223,1005,224,659,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226";

pub fn solve_a() {
    let mut program = IntCodeProgram::from_str(DEFAULT_COMMAND);
    program.add_input(1);
    program.run_to_end();

    let ans = *program.get_last_output().unwrap();

    assert_eq!(ans, 16225258);
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let mut program = IntCodeProgram::from_str(DEFAULT_COMMAND);
    program.add_input(5);
    program.run_to_end();

    let ans = *program.get_last_output().unwrap();

    assert_eq!(ans, 2808771);
    println!("Solution B: {}", ans);
}


struct IntCodeProgram {
    command: CommandMap,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    finished: bool,
    ptr: usize,
}


impl IntCodeProgram {
    pub fn from_str(input: &str) -> IntCodeProgram {
        let command = input.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .enumerate()
            .collect::<CommandMap>();

        IntCodeProgram {
            command,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            finished: false,
            ptr: 0,
        }
    }

    pub fn add_input(&mut self, x: i64) {
        self.inputs.push_back(x);
    }

    /// Runs till HALTED op code.
    pub fn run_to_end(&mut self) {
        while !self.finished {
            self.run();
        }
    }

    /// Runs and stops either at OUTPUT or HALTED op code.
    pub fn run(&mut self) {
        loop {
            let (op, p1, p2, p3) = decode_op(*self.command.get(&self.ptr).unwrap());

            match op {
                1 | 2 => {
                    let v1 = self.get_value(p1, self.ptr + 1);
                    let v2 = self.get_value(p2, self.ptr + 2);

                    let value = match op {
                        1 => v1 + v2,
                        2 => v1 * v2,
                        _ => 0,  // this is never going to happen!
                    };

                    self.save_value(p3, self.ptr + 3, value);
                    self.ptr += 4;
                }
                3 => {
                    let value = self.inputs.pop_front().unwrap();

                    self.save_value(p1, self.ptr + 1, value);
                    self.ptr += 2;
                }
                4 => {
                    let value = self.get_value(p1, self.ptr + 1);
                    self.outputs.push_back(value);
                    self.ptr += 2;
                    break;
                }
                5 => {
                    if self.get_value(p1, self.ptr + 1) != 0 {
                        self.ptr = self.get_value(p2, self.ptr + 2) as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    if self.get_value(p1, self.ptr + 1) == 0 {
                        self.ptr = self.get_value(p2, self.ptr + 2) as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                7 => {
                    let v1 = self.get_value(p1, self.ptr + 1);
                    let v2 = self.get_value(p2, self.ptr + 2);
                    let value = if v1 < v2 { 1 } else { 0 };

                    self.save_value(p3, self.ptr + 3, value);
                    self.ptr += 4;
                }
                8 => {
                    let v1 = self.get_value(p1, self.ptr + 1);
                    let v2 = self.get_value(p2, self.ptr + 2);
                    let value = if v1 == v2 { 1 } else { 0 };

                    self.save_value(p3, self.ptr + 3, value);
                    self.ptr += 4;
                }

                99 => {
                    self.finished = true;
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
            _ => panic!("Invalid ParameterMode: {mode:?}")
        }
    }

    fn save_value(&mut self, mode: ParameterMode, ptr: usize, value: i64) {
        match mode {
            ParameterMode::Position => {
                let pos = *self.command.get(&ptr).unwrap() as usize;
                self.command.insert(pos, value);
            }
            ParameterMode::Immediate => { panic!("Save value cannot be in Immediate mode") }
            ParameterMode::Relative => { panic!("Not implemented") }
        };
    }

    pub fn get_last_output(&self) -> Option<&i64> {
        self.outputs.iter().last()
    }
}


#[cfg(test)]
mod tests {
    use super::IntCodeProgram;

    #[test]
    fn test_part_1() {
        for (cmd, input) in [
            ("3,0,4,0,99", 1)
        ] {
            let mut program = IntCodeProgram::from_str(cmd);
            program.add_input(input);

            program.run_to_end();

            assert_eq!(program.outputs.len(), 1);
            assert_eq!(program.outputs[0], input);
        }
    }

    #[test]
    fn test_part_2() {
        for (cmd, input, expected) in [
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0),
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1, 1),
        ] {
            let mut program = IntCodeProgram::from_str(cmd);
            program.add_input(input);

            program.run_to_end();

            assert_eq!(*program.get_last_output().unwrap(), expected);
        }
    }
}
