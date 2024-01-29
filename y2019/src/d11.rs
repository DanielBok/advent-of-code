use std::collections::{HashMap, VecDeque};

use crate::int_code::{CommandMap, decode_op, ParameterMode};

const PUZZLE_INPUT: &str = "3,8,1005,8,336,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,28,1006,0,36,1,2,5,10,1006,0,57,1006,0,68,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1002,8,1,63,2,6,20,10,1,106,7,10,2,9,0,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,102,1,8,97,1006,0,71,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,1002,8,1,122,2,105,20,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,148,2,1101,12,10,1006,0,65,2,1001,19,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,181,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,204,2,7,14,10,2,1005,20,10,1006,0,19,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,102,1,8,236,1006,0,76,1006,0,28,1,1003,10,10,1006,0,72,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,271,1006,0,70,2,107,20,10,1006,0,81,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,303,2,3,11,10,2,9,1,10,2,1107,1,10,101,1,9,9,1007,9,913,10,1005,10,15,99,109,658,104,0,104,1,21101,0,387508441896,1,21102,1,353,0,1106,0,457,21101,0,937151013780,1,21101,0,364,0,1105,1,457,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,179490040923,1,1,21102,411,1,0,1105,1,457,21101,46211964123,0,1,21102,422,1,0,1106,0,457,3,10,104,0,104,0,3,10,104,0,104,0,21101,838324716308,0,1,21101,0,445,0,1106,0,457,21102,1,868410610452,1,21102,1,456,0,1106,0,457,99,109,2,22101,0,-1,1,21101,40,0,2,21101,0,488,3,21101,478,0,0,1106,0,521,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,483,484,499,4,0,1001,483,1,483,108,4,483,10,1006,10,515,1101,0,0,483,109,-2,2105,1,0,0,109,4,2101,0,-1,520,1207,-3,0,10,1006,10,538,21101,0,0,-3,22102,1,-3,1,21202,-2,1,2,21101,0,1,3,21101,557,0,0,1105,1,562,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,585,2207,-4,-2,10,1006,10,585,22101,0,-4,-4,1106,0,653,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,604,1,0,1106,0,562,21202,1,1,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,623,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,645,21202,-1,1,1,21101,0,645,0,106,0,520,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0";


fn paint(starting_tile: Tile) -> HashMap<(i64, i64), Tile> {
    let mut map = HashMap::from([((0, 0), starting_tile)]);

    let mut robot = Robot::new(&mut map);
    let mut program = IntCodeProgram::from_str(PUZZLE_INPUT, &mut robot);

    program.run_to_end();

    map
}


pub fn solve_a() {
    let ans = paint(Tile::Black).len();
    assert_eq!(ans, 2018);
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let map = paint(Tile::White);

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for (pos, _) in &map {
        if pos.0 > max_x {
            max_x = pos.0;
        } else if pos.0 < min_x {
            min_x = pos.0;
        }

        if pos.1 > max_y {
            max_y = pos.1;
        } else if pos.1 < min_y {
            min_y = pos.1;
        }
    }

    // offsets to get positive index
    let dx = (max_x - min_x + 1) as usize;
    let dy = (max_y - min_y + 1) as usize;

    let mut drawing: Vec<Vec<char>> = Vec::with_capacity(dy);
    for _ in 0..dy {
        drawing.push(Vec::from(['#'].repeat(dx)));
    }

    for ((x, y), tile) in &map {
        if *tile == Tile::Black {
            let x = (*x - min_x) as usize;
            let y = (*y - min_y) as usize;

            drawing[y][x] = ' ';
        }
    }

    drawing.reverse();

    println!("Solution B: APFKRKBR\n");
    for chars in drawing {
        let line = String::from_iter(chars);
        println!("{}", &line[1..line.len() - 3]);
    }
}

enum Face {
    Up,
    Down,
    Left,
    Right,
}

enum Turn {
    Left,
    Right,
}

impl Turn {
    pub fn new(code: i64) -> Turn {
        match code {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Invalid turn code")
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Black,
    White,
}

impl Tile {
    pub fn new(code: i64) -> Tile {
        match code {
            0 => Tile::Black,
            1 => Tile::White,
            _ => panic!("Invalid tile code")
        }
    }

    pub fn tile_code(&self) -> i64 {
        match self {
            Tile::Black => { 0 }
            Tile::White => { 1 }
        }
    }
}

struct Robot<'a> {
    map: &'a mut HashMap<(i64, i64), Tile>,
    pos: (i64, i64),
    face: Face,
}

impl Robot<'_> {
    fn new(map: &mut HashMap<(i64, i64), Tile>) -> Robot {
        Robot {
            map,
            pos: (0, 0),
            face: Face::Up,
        }
    }

    /// starts the painting operations
    pub fn single_step(&mut self, paint: Tile, turn: Turn) {
        let face = &self.face;
        let (x, y) = self.pos;
        self.map.insert((x, y), paint);

        match (face, turn) {
            (Face::Up, Turn::Left) | (Face::Down, Turn::Right) => {
                self.face = Face::Left;
                self.pos = (x - 1, y);
            }
            (Face::Up, Turn::Right) | (Face::Down, Turn::Left) => {
                self.face = Face::Right;
                self.pos = (x + 1, y);
            }
            (Face::Left, Turn::Left) | (Face::Right, Turn::Right) => {
                self.face = Face::Down;
                self.pos = (x, y - 1);
            }
            (Face::Left, Turn::Right) | (Face::Right, Turn::Left) => {
                self.face = Face::Up;
                self.pos = (x, y + 1);
            }
        };
    }

    /// Gets the signal based on the current tile the robot is on
    pub fn get_signal(&self) -> i64 {
        let tile = self.map.get(&self.pos).unwrap_or(&Tile::Black);
        tile.tile_code()
    }
}

struct IntCodeProgram<'a> {
    command: CommandMap,
    outputs: VecDeque<i64>,
    finished: bool,
    ptr: usize,
    offset: i64,
    robot: &'a mut Robot<'a>,
}


impl IntCodeProgram<'_> {
    pub fn from_str<'a>(input: &str, robot: &'a mut Robot<'a>) -> IntCodeProgram<'a> {
        let command = input.split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .enumerate()
            .collect::<CommandMap>();

        IntCodeProgram {
            command,
            outputs: VecDeque::new(),
            finished: false,
            ptr: 0,
            offset: 0,
            robot,
        }
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
                    let value = self.robot.get_signal();

                    self.save_value(p1, self.ptr + 1, value);
                    self.ptr += 2;
                }
                4 => {
                    let value = self.get_value(p1, self.ptr + 1);
                    self.outputs.push_back(value);
                    self.ptr += 2;

                    if self.outputs.len() == 2 {
                        self.robot.single_step(
                            Tile::new(self.outputs[0]),
                            Turn::new(self.outputs[1]),
                        );
                        self.outputs.clear();
                    }

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
}