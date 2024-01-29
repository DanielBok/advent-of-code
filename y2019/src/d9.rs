use std::collections::VecDeque;
use crate::int_code::{CommandMap, decode_op, ParameterMode};

const PUZZLE_INPUT: &str = "1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,37,1000,1101,856,0,1029,1101,286,0,1025,1101,39,0,1004,1101,861,0,1028,1101,845,0,1026,1102,28,1,1002,1102,1,0,1020,1101,0,892,1023,1101,0,291,1024,1101,35,0,1018,1101,0,27,1006,1102,1,26,1011,1101,33,0,1019,1102,31,1,1014,1102,1,36,1010,1102,23,1,1007,1101,0,32,1016,1101,29,0,1008,1101,20,0,1001,1102,1,25,1015,1101,38,0,1017,1101,0,24,1012,1102,1,22,1005,1101,1,0,1021,1101,0,21,1003,1102,1,838,1027,1102,1,30,1013,1101,895,0,1022,1101,0,34,1009,109,7,1208,0,22,63,1005,63,201,1001,64,1,64,1105,1,203,4,187,1002,64,2,64,109,-6,2102,1,5,63,1008,63,24,63,1005,63,223,1105,1,229,4,209,1001,64,1,64,1002,64,2,64,109,17,21102,40,1,-6,1008,1012,40,63,1005,63,255,4,235,1001,64,1,64,1106,0,255,1002,64,2,64,109,-15,21108,41,41,9,1005,1012,277,4,261,1001,64,1,64,1106,0,277,1002,64,2,64,109,11,2105,1,10,4,283,1105,1,295,1001,64,1,64,1002,64,2,64,109,-9,21101,42,0,8,1008,1013,44,63,1005,63,315,1105,1,321,4,301,1001,64,1,64,1002,64,2,64,109,13,1206,3,337,1001,64,1,64,1106,0,339,4,327,1002,64,2,64,109,-10,1208,0,29,63,1005,63,361,4,345,1001,64,1,64,1106,0,361,1002,64,2,64,109,2,2108,27,-4,63,1005,63,383,4,367,1001,64,1,64,1105,1,383,1002,64,2,64,109,-4,1207,2,30,63,1005,63,405,4,389,1001,64,1,64,1105,1,405,1002,64,2,64,109,22,1205,-8,417,1106,0,423,4,411,1001,64,1,64,1002,64,2,64,109,-27,2108,19,0,63,1005,63,443,1001,64,1,64,1106,0,445,4,429,1002,64,2,64,109,13,21108,43,45,-1,1005,1013,461,1106,0,467,4,451,1001,64,1,64,1002,64,2,64,109,1,21107,44,45,4,1005,1019,485,4,473,1105,1,489,1001,64,1,64,1002,64,2,64,109,-8,2102,1,-7,63,1008,63,37,63,1005,63,515,4,495,1001,64,1,64,1106,0,515,1002,64,2,64,109,1,2107,38,-4,63,1005,63,533,4,521,1105,1,537,1001,64,1,64,1002,64,2,64,109,4,21107,45,44,1,1005,1013,553,1106,0,559,4,543,1001,64,1,64,1002,64,2,64,109,-7,2107,21,-4,63,1005,63,575,1106,0,581,4,565,1001,64,1,64,1002,64,2,64,109,9,1205,7,599,4,587,1001,64,1,64,1105,1,599,1002,64,2,64,109,-11,2101,0,-3,63,1008,63,40,63,1005,63,619,1105,1,625,4,605,1001,64,1,64,1002,64,2,64,109,1,2101,0,-2,63,1008,63,28,63,1005,63,651,4,631,1001,64,1,64,1106,0,651,1002,64,2,64,109,1,21102,46,1,7,1008,1012,44,63,1005,63,671,1106,0,677,4,657,1001,64,1,64,1002,64,2,64,109,4,1201,-7,0,63,1008,63,28,63,1005,63,699,4,683,1105,1,703,1001,64,1,64,1002,64,2,64,109,-6,1207,-3,36,63,1005,63,719,1105,1,725,4,709,1001,64,1,64,1002,64,2,64,109,-4,1201,6,0,63,1008,63,23,63,1005,63,745,1106,0,751,4,731,1001,64,1,64,1002,64,2,64,109,8,1202,-6,1,63,1008,63,20,63,1005,63,777,4,757,1001,64,1,64,1105,1,777,1002,64,2,64,109,5,1202,-5,1,63,1008,63,25,63,1005,63,801,1001,64,1,64,1105,1,803,4,783,1002,64,2,64,109,8,21101,47,0,-6,1008,1014,47,63,1005,63,829,4,809,1001,64,1,64,1106,0,829,1002,64,2,64,109,1,2106,0,6,1001,64,1,64,1106,0,847,4,835,1002,64,2,64,109,11,2106,0,-4,4,853,1105,1,865,1001,64,1,64,1002,64,2,64,109,-15,1206,3,883,4,871,1001,64,1,64,1106,0,883,1002,64,2,64,109,14,2105,1,-8,1105,1,901,4,889,1001,64,1,64,4,64,99,21102,1,27,1,21102,1,915,0,1106,0,922,21201,1,57564,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,1,942,0,1105,1,922,22101,0,1,-1,21201,-2,-3,1,21101,957,0,0,1105,1,922,22201,1,-1,-2,1106,0,968,21202,-2,1,-2,109,-3,2106,0,0";

pub fn solve_a() {
    let mut program = IntCodeProgram::from_str(PUZZLE_INPUT);
    program.add_input(1);
    program.run_to_end();

    let ans = *program.get_last_output().unwrap();
    assert_eq!(ans, 2316632620);
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let mut program = IntCodeProgram::from_str(PUZZLE_INPUT);
    program.add_input(2);
    program.run_to_end();

    let ans = *program.get_last_output().unwrap();
    assert_eq!(ans, 78869);
    println!("Solution B: {}", ans);
}

struct IntCodeProgram {
    command: CommandMap,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    finished: bool,
    ptr: usize,
    offset: i64,
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
            offset: 0,
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
                9 => {
                    self.offset += self.get_value(p1, self.ptr + 1);
                    self.ptr += 2;
                }

                99 => {
                    self.finished = true;
                    break;
                }
                _ => panic!("Invalid op code {op}")
            }
        }
    }

    fn get_value(&self, mode: ParameterMode, ptr: usize) -> i64 {
        match mode {
            ParameterMode::Position => {
                let pos = self.read_memory(ptr) as usize;
                self.read_memory(pos)
            }
            ParameterMode::Immediate => {
                self.read_memory(ptr)
            }
            ParameterMode::Relative => {
                let pos = self.read_memory(ptr) + self.offset;
                self.read_memory(pos as usize)
            }
        }
    }

    fn read_memory(&self, index: usize) -> i64 {
        *self.command.get(&index).unwrap_or(&0)
    }

    fn save_value(&mut self, mode: ParameterMode, ptr: usize, value: i64) {
        match mode {
            ParameterMode::Position => {
                let pos = *self.command.get(&ptr).unwrap() as usize;
                self.command.insert(pos, value);
            }
            ParameterMode::Immediate => { panic!("Save value cannot be in Immediate mode") }
            ParameterMode::Relative => {
                let pos = *self.command.get(&ptr).unwrap() + self.offset;
                self.command.insert(pos as usize, value);
            }
        };
    }

    pub fn get_last_output(&self) -> Option<&i64> {
        self.outputs.iter().last()
    }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::IntCodeProgram;

    #[test]
    fn test_relative_adjustments() {
        const INPUT: i64 = 1;
        let input_ans = INPUT.to_string();

        for (inp, exp) in [
            ("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
            ("104,1125899906842624,99", "1125899906842624"),
            ("1102,34915192,34915192,7,4,7,99,0", "1219070632396864"),
            ("9,1,109,4,104,2,99,10", "2"),
            ("9,1,109,4,4,2,99,10", "109"),
            ("9,1,109,4,204,2,99,10", "10"),
            ("109,-1,4,1,99", "-1"),
            ("109,-1,104,1,99", "1"),
            ("109,-1,204,1,99", "109"),
            ("109,1,9,2,204,-6,99", "204"),
            ("109,1,109,9,204,-6,99", "204"),
            ("109,1,209,-1,204,-106,99", "204"),
            ("109,1,3,3,204,2,99", input_ans.as_str()),
            ("109,1,203,2,204,2,99", input_ans.as_str()),
        ] {
            let mut program = IntCodeProgram::from_str(inp);
            program.add_input(INPUT);
            program.run_to_end();

            assert_eq!(exp, program.outputs.iter().join(","), "case {} failed", inp);
        }
    }
}