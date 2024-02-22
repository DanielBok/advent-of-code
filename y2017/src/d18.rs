use std::collections::{HashMap, VecDeque};

use crate::inputs::read_content;

#[derive(Debug)]
enum Instruction {
    Snd(InsVal),
    Rcv(InsVal),
    Set(InsVal, InsVal),
    Mul(InsVal, InsVal),
    Add(InsVal, InsVal),
    Mod(InsVal, InsVal),
    Jgz(InsVal, InsVal),
}

#[derive(Debug)]
enum InsVal {
    Integer(i64),
    Char(char),
}

impl InsVal {
    fn new(input: &str) -> Self {
        match input.parse::<i64>() {
            Ok(v) => { InsVal::Integer(v) }
            Err(_) => {
                assert_eq!(input.len(), 1);
                InsVal::Char(input.chars().next().unwrap())
            }
        }
    }

    fn get_char(&self) -> Option<char> {
        match self {
            InsVal::Integer(_) => None,
            InsVal::Char(c) => Some(*c),
        }
    }

    fn get_integer(&self) -> Option<i64> {
        match self {
            InsVal::Char(_) => None,
            InsVal::Integer(v) => Some(*v),
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let parts = line.trim().split(" ").collect::<Vec<_>>();
            let src = InsVal::new(parts[1]);

            match parts[0] {
                "snd" => { Instruction::Snd(src) }
                "rcv" => { Instruction::Rcv(src) }
                "set" => { Instruction::Set(src, InsVal::new(parts[2])) }
                "add" => { Instruction::Add(src, InsVal::new(parts[2])) }
                "mul" => { Instruction::Mul(src, InsVal::new(parts[2])) }
                "mod" => { Instruction::Mod(src, InsVal::new(parts[2])) }
                "jgz" => { Instruction::Jgz(src, InsVal::new(parts[2])) }
                _ => { panic!("Invalid command: {}", parts[0]); }
            }
        })
        .collect()
}

struct Register {
    register: HashMap<char, i64>,
}

impl Register {
    fn new() -> Self {
        Register { register: HashMap::new() }
    }

    fn get(&mut self, ins_val: &InsVal) -> i64 {
        if let Some(c) = ins_val.get_char() {
            self.register.get(&c).map_or(0, |v| *v)
        } else {
            ins_val.get_integer().unwrap()
        }
    }

    fn get_mut(&mut self, ins_val: &InsVal) -> &mut i64 {
        let c = ins_val.get_char().unwrap();
        if !self.register.contains_key(&c) {
            self.register.insert(c, 0);
        }

        self.register.get_mut(&c).unwrap()
    }

    fn set(&mut self, src: &InsVal, value: &InsVal) {
        let c = src.get_char().unwrap();

        let value = match value {
            InsVal::Integer(v) => *v,
            InsVal::Char(_) => { self.get(value) }
        };


        self.register.insert(c, value);
    }
}

pub fn solve_a() {
    let instructions = parse_instructions(&read_content(18));
    let ans = get_first_recovered_frequency(&instructions);

    assert_eq!(ans, 2951);
    println!("Solution A: {}", ans);
}

fn get_first_recovered_frequency(instructions: &Vec<Instruction>) -> i64 {
    let mut i = 0;
    let mut register = Register::new();
    let mut last_frequency = 0;

    while i < instructions.len() {
        match &instructions[i] {
            Instruction::Snd(x) => {
                last_frequency = register.get(x);
                i += 1;
            }
            Instruction::Rcv(x) => {
                if register.get(x) != 0 {
                    break;
                } else {
                    i += 1;
                }
            }
            Instruction::Set(x, v) => {
                register.set(x, v);
                i += 1;
            }
            Instruction::Mul(x, y) => {
                let y = register.get(y);
                let v = register.get_mut(x);
                *v *= y;
                i += 1;
            }
            Instruction::Add(x, y) => {
                let y = register.get(y);
                let v = register.get_mut(x);
                *v += y;
                i += 1;
            }
            Instruction::Mod(x, y) => {
                let y = register.get(y);
                let v = register.get_mut(x);
                *v %= y;
                i += 1;
            }
            Instruction::Jgz(x, y) => {
                if register.get(x) <= 0 {
                    i += 1;
                } else {
                    let y = register.get(y);
                    if y == 0 {
                        panic!("Jump can't be 0!")
                    } else if y < 0 {
                        if let Some(next_i) = i.checked_sub(y.abs() as usize) {
                            i = next_i;
                        } else {
                            break;
                        }
                    } else {
                        // y > 0
                        i += y as usize;
                    }
                }
            }
        }
    };

    last_frequency
}

pub fn solve_b() {
    let instructions = parse_instructions(&read_content(18));
    let ans = get_num_times_sent(instructions);
    println!("Solution B: {}", ans);
}


fn get_num_times_sent(instructions: Vec<Instruction>) -> usize {
    let mut pa = Program::new(0, &instructions);
    let mut pb = Program::new(1, &instructions);

    loop {
        pa.run(&mut pb);
        pb.run(&mut pa);

        if pa.queue.is_empty() && pb.queue.is_empty() {
            return pb.send_count;
        }
    }
}


struct Program<'a> {
    register: Register,
    queue: VecDeque<i64>,
    i: usize,
    instructions: &'a Vec<Instruction>,
    send_count: usize,
}

impl<'a> Program<'a> {
    fn new(id: i64, instructions: &'a Vec<Instruction>) -> Self {
        let mut register = Register::new();
        register.set(&InsVal::Char('p'), &InsVal::Integer(id));
        let queue = VecDeque::new();

        Program { register, queue, instructions, i: 0, send_count: 0 }
    }

    fn add_to_queue(&mut self, value: i64) {
        self.queue.push_back(value);
    }

    /// Runs the loop till receive step when it times out
    fn run(&mut self, other: &mut Program) {
        while self.i < self.instructions.len() {
            match &self.instructions[self.i] {
                Instruction::Snd(x) => {
                    let v = self.register.get(x);

                    other.add_to_queue(v);
                    self.send_count += 1;
                    self.i += 1;
                }
                Instruction::Rcv(x) => {
                    if let Some(v) = self.queue.pop_front() {
                        self.register.set(x, &InsVal::Integer(v));
                        self.i += 1;
                    } else {
                        return;
                    }
                }
                Instruction::Set(x, v) => {
                    self.register.set(x, v);
                    self.i += 1;
                }
                Instruction::Mul(x, y) => {
                    let y = self.register.get(y);
                    let v = self.register.get_mut(x);
                    *v *= y;
                    self.i += 1;
                }
                Instruction::Add(x, y) => {
                    let y = self.register.get(y);
                    let v = self.register.get_mut(x);
                    *v += y;
                    self.i += 1;
                }
                Instruction::Mod(x, y) => {
                    let y = self.register.get(y);
                    let v = self.register.get_mut(x);
                    *v %= y;
                    self.i += 1;
                }
                Instruction::Jgz(x, y) => {
                    if self.register.get(x) <= 0 {
                        self.i += 1;
                    } else {
                        let y = self.register.get(y);
                        if y == 0 {
                            panic!("Jump can't be 0!")
                        } else if y < 0 {
                            if let Some(next_i) = self.i.checked_sub(y.abs() as usize) {
                                self.i = next_i;
                            } else {
                                return;
                            }
                        } else {
                            // y > 0
                            self.i += y as usize;
                        }
                    }
                }
            }
        };
    }
}


#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::thread;

    use super::{get_first_recovered_frequency, get_num_times_sent, Instruction, InsVal, parse_instructions, Register};

    #[test]
    fn test_get_first_recovered_frequency() {
        let instructions = parse_instructions("set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2");

        let ans = get_first_recovered_frequency(&instructions);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_program() {
        let instructions = parse_instructions("snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d");

        let ans = get_num_times_sent(instructions);
        assert_eq!(ans, 3);
    }

    struct Program {
        sender: Sender<i64>,
        receiver: Receiver<i64>,
        instructions: Arc<Vec<Instruction>>,
        register: Register,
        send_count: usize,
        i: usize,

    }

    impl Program {
        fn new(id: i64, sender: Sender<i64>, receiver: Receiver<i64>, instructions: Arc<Vec<Instruction>>) -> Self {
            let mut register = Register::new();
            register.set(&InsVal::Char('p'), &InsVal::Integer(id));

            Program { sender, receiver, instructions, register, send_count: 0, i: 0 }
        }

        /// Returns true if program can still run but is waiting for inputs, otherwise
        /// false if program has halted
        fn run(&mut self) -> bool {
            while self.i < self.instructions.len() {
                match &self.instructions[self.i] {
                    Instruction::Snd(x) => {
                        let v = self.register.get(x);

                        self.sender.send(v).expect("Could not send to other channel");
                        self.send_count += 1;
                        self.i += 1;
                    }
                    Instruction::Rcv(x) => {
                        match self.receiver.try_recv() {
                            Ok(v) => {
                                self.register.set(x, &InsVal::Integer(v));
                                self.i += 1;
                            }
                            Err(_) => {
                                return true;
                            }
                        }
                    }
                    Instruction::Set(x, v) => {
                        self.register.set(x, v);
                        self.i += 1;
                    }
                    Instruction::Mul(x, y) => {
                        let y = self.register.get(y);
                        let v = self.register.get_mut(x);
                        *v *= y;
                        self.i += 1;
                    }
                    Instruction::Add(x, y) => {
                        let y = self.register.get(y);
                        let v = self.register.get_mut(x);
                        *v += y;
                        self.i += 1;
                    }
                    Instruction::Mod(x, y) => {
                        let y = self.register.get(y);
                        let v = self.register.get_mut(x);
                        *v %= y;
                        self.i += 1;
                    }
                    Instruction::Jgz(x, y) => {
                        if self.register.get(x) <= 0 {
                            self.i += 1;
                        } else {
                            let y = self.register.get(y);
                            if y == 0 {
                                panic!("Jump can't be 0!")
                            } else if y < 0 {
                                if let Some(next_i) = self.i.checked_sub(y.abs() as usize) {
                                    self.i = next_i;
                                } else {
                                    return false;
                                }
                            } else {
                                // y > 0
                                self.i += y as usize;
                            }
                        }
                    }
                }
            };
            false
        }
    }

    #[test]
    fn test_get_num_times_sent_parallel() {
        // parallel version of solve B
        
        let instructions = Arc::new(parse_instructions("set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 316
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19"));
        let (sa, ra) = channel();
        let (sb, rb) = channel();
        let wa = Arc::new(RwLock::new(false));
        let wb = Arc::new(RwLock::new(false));

        let mut pa = Program::new(0, sb, ra, instructions.clone());
        let mut pb = Program::new(1, sa, rb, instructions.clone());

        let thread_handles = Vec::from([
            {
                let wa = wa.clone();
                let wb = wb.clone();

                thread::spawn(move || {
                    while pa.run() {
                        *wa.write().unwrap() = true;
                        if *wa.read().unwrap() && *wb.read().unwrap() { break; } else { *wa.write().unwrap() = false; }
                    }
                    *wa.write().unwrap() = true;
                    pa.send_count
                })
            },
            {
                thread::spawn(move || {
                    while pb.run() {
                        *wb.write().unwrap() = true;
                        if *wa.read().unwrap() && *wb.read().unwrap() { break; } else { *wb.write().unwrap() = false; }
                    }
                    *wb.write().unwrap() = true;
                    pb.send_count
                })
            }
        ]);

        // only need the last value
        let ans = thread_handles
            .into_iter()
            .map(|th| th.join().unwrap())
            .last()
            .unwrap();

        assert_eq!(ans, 7366);
    }
}