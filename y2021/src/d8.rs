use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::inputs::read_contents;

fn parse_line(line: &str) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
    let (input, output) = line.split_once(" | ").unwrap();

    let input = input.split_ascii_whitespace().map(|x| x.chars().collect()).collect();
    let output = output.split_whitespace().map(|x| x.chars().collect()).collect();

    (input, output)
}

pub fn solve_a() {
    let total: usize = read_contents(8)
        .lines()
        .fold(0, |acc, line| {
            acc + parse_line(line).1
                                  .iter()
                                  .map(|x| match x.len() {
                                      2 | 3 | 4 | 7 => 1,
                                      _ => 0
                                  })
                                  .sum::<usize>()
        });

    println!("Solution A: {}", total)
}

pub fn solve_b() {
    let total: usize = read_contents(8)
        .lines()
        .fold(0, |acc, line| {
            let (input, output) = parse_line(line);
            let machine = SignalMachine::new(&input);
            let value = machine.parse_output(&output);

            acc + value
        });

    println!("Solution B: {}", total);
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Position {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

struct SignalMachine {
    char_to_pos: HashMap<char, Position>,
    pos_to_char: HashMap<Position, char>,
}

impl SignalMachine {
    pub fn new(input: &Vec<HashSet<char>>) -> Self {
        let char_to_pos: HashMap<char, Position> = HashMap::new();
        let pos_to_char: HashMap<Position, char> = HashMap::new();
        let mut this = Self { pos_to_char, char_to_pos };

        this.determine_top(input);
        this.determine_right(input);
        this.determine_bottom(input);
        this.determine_middle(input);
        this.determine_left(input);

        this
    }

    pub fn parse_output(&self, signals: &Vec<HashSet<char>>) -> usize {
        signals.iter()
               .enumerate()
               .map(|(i, signal)| {
                   let mul = 10_usize.pow((signals.len() - 1 - i) as u32);

                   mul * self.signal_to_value(signal)
               })
               .sum()
    }

    fn signal_to_value(&self, signal: &HashSet<char>) -> usize {
        match signal.len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            5 => {
                let positions = signal.iter()
                                      .map(|c| *self.char_to_pos.get(c).unwrap())
                                      .collect::<HashSet<_>>();

                if positions.contains(&Position::TopLeft) {
                    5
                } else if positions.contains(&Position::BottomLeft) {
                    2
                } else if positions.contains(&Position::BottomRight) && positions.contains(&Position::TopRight) {
                    3
                } else {
                    panic!("Invalid signal: '{}'", signal.iter().collect::<String>())
                }
            }
            6 => {
                let positions = signal.iter()
                                      .map(|c| *self.char_to_pos.get(c).unwrap())
                                      .collect::<HashSet<_>>();

                if !positions.contains(&Position::Middle) {
                    0
                } else if !positions.contains(&Position::TopRight) {
                    6
                } else if !positions.contains(&Position::BottomLeft) {
                    9
                } else {
                    println!("{:?}", self.char_to_pos);
                    panic!("Invalid signal: '{}'", signal.iter().collect::<String>())
                }
            }
            _ => panic!("Invalid signal: '{}'", signal.iter().collect::<String>())
        }
    }

    fn insert(&mut self, char: char, position: Position) {
        self.char_to_pos.insert(char, position);
        self.pos_to_char.insert(position, char);
    }

    fn determine_top(&mut self, input: &Vec<HashSet<char>>) {
        let one = input.iter().find(|v| v.len() == 2).unwrap();
        let seven = input.iter().find(|v| v.len() == 3).unwrap();

        let top_char = seven.iter()
                            .find(|&c| !one.contains(c))
                            .expect("Could not determine top position");

        self.insert(*top_char, Position::Top);
    }

    fn determine_right(&mut self, input: &Vec<HashSet<char>>) {
        let one = input.iter().find(|v| v.len() == 2).unwrap();

        let six = input.iter()
                       .find(|v| v.len() == 6 && v.intersection(one).count() == 1)
                       .unwrap();

        let bottom_right_char = one.iter()
                                   .find(|&v| six.contains(v))
                                   .expect("Could not determine bottom right");

        self.insert(*bottom_right_char, Position::BottomRight);

        let top_right_char = one.iter()
                                .find(|&v| v != bottom_right_char)
                                .expect("Could not determine bottom right");
        self.insert(*top_right_char, Position::TopRight);
    }

    fn determine_bottom(&mut self, input: &Vec<HashSet<char>>) {
        let four = input.iter().find(|v| v.len() == 4).unwrap();
        let top_char = *self.pos_to_char
                            .get(&Position::Top)
                            .expect("Could not retrieve top position char, make sure to run `determine_top` first");

        let bottom_char = input.iter()
                               .find(|v| v.len() == 6 && v.intersection(four).count() == 4)   // this find the number 9
                               .unwrap()
                               .iter()
                               .find(|&c| *c != top_char && !four.contains(c))
                               .unwrap();
        self.insert(*bottom_char, Position::Bottom);
    }

    fn determine_middle(&mut self, input: &Vec<HashSet<char>>) {
        // these are all the positions that 3 has but not 2 and 5
        let positions = [Position::Top,
            Position::TopRight,
            Position::BottomRight,
            Position::Bottom].iter()
                             .map(|p| *self.pos_to_char.get(p).unwrap())
                             .collect::<HashSet<_>>();

        let three = input.iter()
                         .find(|v| v.len() == 5 && v.intersection(&positions).count() == 4)
                         .unwrap();
        let middle_char = three.iter()
                               .find(|&v| !positions.contains(v))
                               .unwrap();

        self.insert(*middle_char, Position::Middle);
    }

    fn determine_left(&mut self, input: &Vec<HashSet<char>>) {
        // this is the 3 positions
        let positions = [Position::Top,
            Position::TopRight,
            Position::Middle,
            Position::BottomRight,
            Position::Bottom].iter()
                             .map(|p| *self.pos_to_char.get(p).unwrap())
                             .collect::<HashSet<_>>();

        let nine = input.iter()
                        .find(|v| v.len() == 6 && v.intersection(&positions).count() == 5)
                        .unwrap();

        let top_left = nine.iter()
                           .find(|&v| !positions.contains(v))
                           .unwrap();
        self.insert(*top_left, Position::TopLeft);

        let eight = input.iter()
                         .find(|v| v.len() == 7)
                         .unwrap();

        let bottom_left = eight.iter()
                               .find(|&v| !nine.contains(v))
                               .unwrap();

        self.insert(*bottom_left, Position::BottomLeft);
    }
}


#[cfg(test)]
mod tests {
    use crate::d8::{parse_line, SignalMachine};

    #[test]
    fn test_signal_machine() {
        for (input, exp) in [
            ("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", 5353),
            ("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe", 8394),
            ("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc", 9781),
            ("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg", 1197),
            ("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb", 9361),
            ("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea", 4873),
            ("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb", 8418),
            ("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe", 4548),
            ("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef", 1625),
            ("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb", 8717),
            ("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", 4315),
        ] {
            let (input, output) = parse_line(input);
            let machine = SignalMachine::new(&input);

            let ans = machine.parse_output(&output);
            assert_eq!(ans, exp);
        }
    }
}
