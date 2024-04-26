use crate::inputs::read_contents;

// this is not necessary, but is here due to a previous implementation
enum Instruction {
    Noop,
    Add(i64),
    Div(i64),
}

struct Block {
    div_z: i64,
    add_x: i64,
    add_y: i64,
}

fn parse_blocks(input: &str) -> Vec<Block> {
    let instructions: Vec<Instruction> = {
        fn parse_values(line: &str) -> Option<i64> {
            match line.split_once(" ") {
                None => None,
                Some((_, value)) => {
                    if value.len() == 1 && value.chars().next().unwrap().is_alphabetic() {
                        None
                    } else {
                        Some(value.parse::<i64>().expect(&format!("Could not parse value: {}", value)))
                    }
                }
            }
        }

        input.lines()
             .map(|line| {
                 let (cmd, rest) = line.split_once(" ").unwrap();
                 let value = parse_values(rest);
                 if let Some(value) = value {
                     match cmd {
                         "add" => Instruction::Add(value),
                         "div" => Instruction::Div(value),
                         _ => Instruction::Noop
                     }
                 } else {
                     Instruction::Noop
                 }
             })
             .collect()
    };

    assert_eq!(instructions.len() % 18, 0);

    (0..instructions.len())
        .step_by(18)
        .map(|i| {
            let div_z = match &instructions[i + 4] {
                Instruction::Div(i) => *i,
                _ => unreachable!()
            };

            let add_x = match &instructions[i + 5] {
                Instruction::Add(i) => *i,
                _ => unreachable!()
            };

            let add_y = match &instructions[i + 15] {
                Instruction::Add(i) => *i,
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
    let blocks = parse_blocks(&read_contents(24));

    let ans = search(&blocks, &SearchType::Largest);
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let blocks = parse_blocks(&read_contents(24));

    let ans = search(&blocks, &SearchType::Smallest);
    println!("Solution B: {}", ans);
}