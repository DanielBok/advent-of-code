use std::collections::VecDeque;

use itertools::Itertools;

use crate::int_code::{CommandMap, decode_op, ParameterMode};

const PUZZLE_INPUT: &str = "3,8,1001,8,10,8,105,1,0,0,21,42,67,84,109,122,203,284,365,446,99999,3,9,1002,9,3,9,1001,9,5,9,102,4,9,9,1001,9,3,9,4,9,99,3,9,1001,9,5,9,1002,9,3,9,1001,9,4,9,102,3,9,9,101,3,9,9,4,9,99,3,9,101,5,9,9,1002,9,3,9,101,5,9,9,4,9,99,3,9,102,5,9,9,101,5,9,9,102,3,9,9,101,3,9,9,102,2,9,9,4,9,99,3,9,101,2,9,9,1002,9,3,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,99";


pub fn solve_a() {
    let ans = (0..=4).permutations(5)
        .map(|setting| get_thruster_signal(PUZZLE_INPUT, &setting))
        .max()
        .unwrap();

    assert_eq!(ans, 262086);
    println!("Solution A: {}", ans);
}

fn get_thruster_signal(cmd: &str, setting: &[i64]) -> i64 {
    let mut output = 0;
    for phase in setting {
        let mut program = IntCodeProgram::from_str(cmd);

        program.add_input(*phase)
            .add_input(output)
            .run_to_end();

        output = *program.get_last_output().unwrap();
    }

    output
}

pub fn solve_b() {
    let ans = (5..=9).permutations(5)
        .map(|s| get_feedback_thruster_signal(PUZZLE_INPUT, &s))
        .max()
        .unwrap();

    assert_eq!(ans, 5371621);
    println!("Solution B: {}", ans);
}

fn get_feedback_thruster_signal(cmd: &str, setting: &[i64]) -> i64 {
    let mut amplifiers = setting.iter().map(|phase| {
        let mut program = IntCodeProgram::from_str(cmd);
        program.add_input(*phase);
        program
    }).collect::<Vec<IntCodeProgram>>();

    let mut thrust = 0;
    let mut output = Some(0);

    // while any of the amplifiers has not gotten the exit signal yet
    while amplifiers.iter().any(|amp| !amp.finished) {
        for (amp_id, amp) in amplifiers.iter_mut().enumerate() {
            if let Some(next_input) = output {
                amp.add_input(next_input);  // add the last output from the previous amplifier
                amp.run();

                if let Some(amp_output) = amp.get_last_output() {
                    output = Some(*amp_output);

                    if amp_id == 4 {
                        thrust = *amp_output;
                    }
                } else {
                    output = None;
                }
            } else if !amp.finished {
                // this feels like a hack but it works. Because not all amplifiers might have
                // halted
                return thrust;
            }
        }
    }
    thrust
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

    pub fn add_input(&mut self, x: i64) -> &mut Self {
        self.inputs.push_back(x);
        self
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
    use crate::d7::{get_thruster_signal};

    #[test]
    fn test_get_thruster_signal() {
        for (cmd, setting, exp) in [
            ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
             [4, 3, 2, 1, 0],
             43210),
            ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
             [0, 1, 2, 3, 4],
             54321),
            ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
             [1, 0, 4, 3, 2],
             65210)
        ] {
            assert_eq!(get_thruster_signal(cmd, &setting), exp)
        }
    }

    // #[test]
    // fn test_get_feedback_thruster_signal() {
    //     for (cmd, setting, exp) in [
    //         ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    //          [9, 8, 7, 6, 5],
    //          139629729),
    //         ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
    //          [9, 7, 8, 5, 6],
    //          18216)
    //     ] {
    //         assert_eq!(get_feedback_thruster_signal(cmd, &setting), exp);
    //     }
    // }
}