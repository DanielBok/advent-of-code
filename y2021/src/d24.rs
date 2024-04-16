use crate::inputs::read_contents;

enum Value {
    Char(char),
    Int(i64),
}

// this is not necessary, but is here due to a previous implementation
enum Instruction {
    Inp(char),
    Add(char, Value),
    Mul(char, Value),
    Div(char, Value),
    Mod(char, Value),
    Eql(char, Value),
}

struct Block {
    div_z: i64,
    add_x: i64,
    add_y: i64,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    fn as_var(var: &str) -> char {
        assert_eq!(var.len(), 1);
        let var = var.chars().next().unwrap();
        assert!(!var.is_digit(10));
        var
    }

    fn as_value(value: &str) -> Value {
        if value.len() == 1 && value.chars().next().unwrap().is_alphabetic() {
            Value::Char(as_var(value))
        } else {
            Value::Int(value.parse::<i64>().expect(&format!("Could not parse value: {}", value)))
        }
    }

    fn parse_values(line: &str) -> (char, Value) {
        let (var, value) = line.split_once(" ").unwrap();
        (as_var(var), as_value(value))
    }

    input.lines()
         .map(|line| {
             let (cmd, rest) = line.split_once(" ").unwrap();
             match cmd {
                 "inp" => Instruction::Inp(as_var(rest)),
                 _ => {
                     let (var, value) = parse_values(rest);
                     match cmd {
                         "add" => Instruction::Add(var, value),
                         "mul" => Instruction::Mul(var, value),
                         "div" => Instruction::Div(var, value),
                         "mod" => Instruction::Mod(var, value),
                         "eql" => Instruction::Eql(var, value),
                         _ => unreachable!()
                     }
                 }
             }
         })
         .collect()
}

fn parse_blocks(instructions: &[Instruction]) -> Vec<Block> {
    assert_eq!(instructions.len() % 18, 0);

    (0..instructions.len())
        .step_by(18)
        .map(|i| {
            let div_z = match &instructions[i + 4] {
                Instruction::Div(_, _v @ Value::Int(i)) => *i,
                _ => unreachable!()
            };

            let add_x = match &instructions[i + 5] {
                Instruction::Add(_, _v @ Value::Int(i)) => *i,
                _ => unreachable!()
            };

            let add_y = match &instructions[i + 15] {
                Instruction::Add(_, _v @ Value::Int(i)) => *i,
                _ => unreachable!()
            };

            assert!(div_z == 1 || div_z == 26);

            Block { div_z, add_x, add_y }
        })
        .collect()
}

impl Block {
    fn calculate(&self, z: i64, w: i64) -> i64 {
        let x = if (z % 26 + self.add_x) == w { 0 } else { 1 };
        let z = z / self.div_z;

        z * (25 * x + 1) + (w + self.add_y) * x
    }
}

enum SearchType {
    Largest,
    Smallest,
}

impl SearchType {
    fn get_range(&self) -> Vec<i64> {
        match self {
            SearchType::Smallest => (1..=9).collect(),
            SearchType::Largest => (1..=9).rev().collect(),
        }
    }
}

fn search(blocks: &[Block], search_type: &SearchType) -> i64 {
    for next_w in search_type.get_range() {
        if let Some(ans) = search_node(blocks, 0, next_w, 0, search_type) {
            return ans
                .iter()
                .fold(0, |acc, n| acc * 10 + *n);
        }
    }

    unreachable!()
}

fn search_node(blocks: &[Block], z: i64, w: i64, index: usize, search_type: &SearchType) -> Option<Vec<i64>> {
    let block = &blocks[index];
    
    // based on block.calculate, if this condition is not true, we diverge from z
    if block.div_z == 26 && z % 26 + block.add_x != w {
        return None;
    }

    let z = blocks[index].calculate(z, w);

    if index == 13 {
        if z == 0 {
            return Some(vec![w]);
        } else {
            return None;
        }
    }

    for next_w in search_type.get_range() {
        if let Some(ans) = search_node(blocks, z, next_w, index + 1, search_type) {
            return Some([vec![w], ans].concat());
        }
    }
    None
}

pub fn solve_a() {
    let instructions = parse_instructions(&read_contents(24));
    let blocks = parse_blocks(&instructions);

    let ans = search(&blocks, &SearchType::Largest);
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let instructions = parse_instructions(&read_contents(24));
    let blocks = parse_blocks(&instructions);

    let ans = search(&blocks, &SearchType::Smallest);
    println!("Solution B: {}", ans);
}