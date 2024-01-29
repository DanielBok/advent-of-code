use std::collections::HashMap;

pub type CommandMap = HashMap<usize, i64>;


#[derive(Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn new(code: i64) -> ParameterMode {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Invalid parameter mode: {code}")
        }
    }
}

pub fn decode_op(code: i64) -> (i64, ParameterMode, ParameterMode, ParameterMode) {
    let op = code % 100;

    let p1 = ParameterMode::new(code / 100 % 10);
    let p2 = ParameterMode::new(code / 1000 % 10);
    let p3 = ParameterMode::new(code / 10000);

    (op, p1, p2, p3)
}
