use std::collections::HashMap;

use once_cell::sync::OnceCell;
use regex::Regex;

use crate::inputs::read_content;

static LINE_RE: OnceCell<Regex> = OnceCell::new();


enum Command {
    Inc,
    Dec,
}

impl Command {
    fn new(cmd: &str) -> Command {
        match cmd {
            "inc" => Command::Inc,
            "dec" => Command::Dec,
            _ => { panic!("Invalid Command: '{}'", cmd) }
        }
    }
}

enum Operator {
    LessEqual,
    Less,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
}

impl Operator {
    fn new(operator: &str) -> Operator {
        match operator {
            ">" => Operator::Greater,
            ">=" => Operator::GreaterEqual,
            "<" => Operator::Less,
            "<=" => Operator::LessEqual,
            "==" => Operator::Equal,
            "!=" => Operator::NotEqual,
            _ => {
                panic!("Invalid operator: {}", operator)
            }
        }
    }
}

struct Instruction {
    register: String,
    command: Command,
    amount: i64,
    cond_register: String,
    cond_operator: Operator,
    cond_amount: i64,
}

impl Instruction {
    fn line_regex() -> &'static Regex {
        LINE_RE.get_or_init(|| Regex::new(r"(?<register>\w+) (?<command>(inc|dec)) (?<amount>-?\d+) if (?<cond_register>\w+) (?<cond_operator>(>=|>|!=|==|<|<=)) (?<cond_amount>-?\d+)").unwrap())
    }

    fn new(line: &str) -> Option<Instruction> {
        let captures = Instruction::line_regex().captures(line).expect(format!("Could not parse line: {}", line).as_str());

        let register = captures.name("register")?.as_str().to_string();
        let command = Command::new(captures.name("command")?.as_str());
        let amount = captures.name("amount")?.as_str().parse::<i64>().unwrap();
        let cond_register = captures.name("cond_register")?.as_str().to_string();
        let cond_operator = Operator::new(captures.name("cond_operator")?.as_str());
        let cond_amount = captures.name("cond_amount")?.as_str().parse::<i64>().unwrap();

        Some(Instruction {
            register,
            command,
            amount,
            cond_register,
            cond_operator,
            cond_amount,
        })
    }
}

struct Registry {
    registers: HashMap<String, i64>,
    highest_value: i64,
}

impl Registry {
    fn new() -> Registry {
        Registry { registers: HashMap::new(), highest_value: 0 }
    }

    fn run_instruction(&mut self, ins: &Instruction) {
        let cond_value = self.registers.entry(ins.cond_register.clone()).or_insert(0);
        let condition_met = match ins.cond_operator {
            Operator::LessEqual => { *cond_value <= ins.cond_amount }
            Operator::Less => { *cond_value < ins.cond_amount }
            Operator::NotEqual => { *cond_value != ins.cond_amount }
            Operator::Equal => { *cond_value == ins.cond_amount }
            Operator::Greater => { *cond_value > ins.cond_amount }
            Operator::GreaterEqual => { *cond_value >= ins.cond_amount }
        };

        if condition_met {
            let delta = match ins.command {
                Command::Inc => { ins.amount }
                Command::Dec => { -ins.amount }
            };

            let value = self.registers.entry(ins.register.clone()).or_insert(0);
            *value += delta;

            if *value > self.highest_value {
                self.highest_value = *value;
            }
        }
    }

    fn max_value(&self) -> i64 {
        *self.registers.values().max().unwrap()
    }
}

fn run_registry() -> Registry {
    let mut registry = Registry::new();

    let input = read_content(8);
    for line in input.lines() {
        if let Some(ins) = Instruction::new(line) {
            registry.run_instruction(&ins);
        } else {
            panic!("Could not parse line as Instruction: {}", line);
        }
    }

    registry
}

pub fn solve_a() {
    let registry = run_registry();
    let ans = registry.max_value();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let registry = run_registry();

    println!("Solution B: {}", registry.highest_value);
}