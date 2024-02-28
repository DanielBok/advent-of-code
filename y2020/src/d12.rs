use crate::inputs::read_contents;

enum Face {
    N,
    S,
    E,
    W,
}

impl Face {
    fn rotate_left(&self, value: i32) -> Face {
        match value {
            90 => {
                match self {
                    Face::N => Face::W,
                    Face::S => Face::E,
                    Face::E => Face::N,
                    Face::W => Face::S,
                }
            }
            180 => {
                match self {
                    Face::N => Face::S,
                    Face::S => Face::N,
                    Face::E => Face::W,
                    Face::W => Face::E,
                }
            }
            270 => {
                match self {
                    Face::N => Face::E,
                    Face::S => Face::W,
                    Face::E => Face::S,
                    Face::W => Face::N,
                }
            }
            _ => panic!("Invalid rotation value: {}", value)
        }
    }

    fn rotate_right(&self, value: i32) -> Face {
        let face = self.rotate_left(value);
        match value {
            90 | 270 => match face {
                Face::N => Face::S,
                Face::S => Face::N,
                Face::E => Face::W,
                Face::W => Face::E,
            },
            180 => face,
            _ => panic!("Invalid rotation value: {}", value)
        }
    }
}

enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    F(i32),
    L(i32),
    R(i32),
}

#[derive(Debug)]
struct Position(i32, i32);


impl Position {
    fn rotate(&self, instruction: &Instruction) -> Position {
        match instruction {
            Instruction::L(value) => {
                match value {
                    90 => Position(self.1, -self.0),
                    180 => Position(-self.0, -self.1),
                    270 => Position(-self.1, self.0),
                    _ => panic!("Invalid rotation value: {}", value)
                }
            }
            Instruction::R(value) => {
                match value {
                    180 => Position(-self.0, -self.1),
                    90 => Position(-self.1, self.0),
                    270 => Position(self.1, -self.0),
                    _ => panic!("Invalid rotation value: {}", value),
                }
            }
            _ => panic!("Invalid instruction")
        }
    }
}

struct Ship {
    pos: Position,
    face: Face,
    waypoint: Position,
}

impl Ship {
    fn new() -> Self {
        Ship { pos: Position(0, 0), face: Face::E, waypoint: Position(-10, 1) }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::N(v) => { self.pos.1 += v; }
            Instruction::S(v) => { self.pos.1 -= v; }
            Instruction::E(v) => { self.pos.0 -= v; }
            Instruction::W(v) => { self.pos.0 += v; }
            Instruction::F(v) => {
                match self.face {
                    Face::N => { self.pos.1 += v; }
                    Face::S => { self.pos.1 -= v; }
                    Face::E => { self.pos.0 -= v; }
                    Face::W => { self.pos.0 += v; }
                }
            }
            Instruction::L(v) => {
                self.face = self.face.rotate_left(*v);
            }
            Instruction::R(v) => {
                self.face = self.face.rotate_right(*v);
            }
        }
    }

    fn distance_from_origin(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }

    fn run_instruction_b(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::N(v) => { self.waypoint.1 += v; }
            Instruction::S(v) => { self.waypoint.1 -= v; }
            Instruction::E(v) => { self.waypoint.0 -= v; }
            Instruction::W(v) => { self.waypoint.0 += v; }
            Instruction::F(v) => {
                self.pos.0 += v * self.waypoint.0;
                self.pos.1 += v * self.waypoint.1;
            }
            Instruction::L(_) | Instruction::R(_) => { self.waypoint = self.waypoint.rotate(&instruction) }
        }
    }
}

fn form_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
         .map(|line| {
             let mut cs = line.chars();
             let letter = cs.next().unwrap();
             let v: i32 = cs.collect::<String>().parse().unwrap();

             match letter {
                 'N' => Instruction::N(v),
                 'S' => Instruction::S(v),
                 'E' => Instruction::E(v),
                 'W' => Instruction::W(v),
                 'F' => Instruction::F(v),
                 'L' => Instruction::L(v),
                 'R' => Instruction::R(v),
                 _ => panic!("Could not parse line '{}'. Letter: {}", line, letter)
             }
         })
         .collect()
}

pub fn solve_a() {
    let instructions = form_instructions(&read_contents(12));
    let mut ship = Ship::new();

    for ins in instructions.iter() {
        ship.run_instruction(ins);
    }

    println!("Solution A: {}", ship.distance_from_origin());
}

pub fn solve_b() {
    let instructions = form_instructions(&read_contents(12));
    let mut ship = Ship::new();

    for ins in instructions.iter() {
        ship.run_instruction_b(ins);
    }

    println!("Solution B: {}", ship.distance_from_origin());
}


#[cfg(test)]
mod tests {
    use super::{form_instructions, Instruction, Ship};

    fn get_test_instructions() -> Vec<Instruction> {
        form_instructions("F10
N3
F7
R90
F11")
    }


    #[test]
    fn test_run_instruction() {
        let instructions = get_test_instructions();
        let mut ship = Ship::new();

        for ins in instructions.iter() {
            ship.run_instruction(ins);
        }

        assert_eq!(ship.distance_from_origin(), 25);
    }

    #[test]
    fn test_run_instruction_b() {
        let instructions = get_test_instructions();
        let mut ship = Ship::new();

        for ins in instructions.iter() {
            ship.run_instruction_b(ins);
        }

        assert_eq!(ship.distance_from_origin(), 286);
    }
}