use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::inputs::read_contents;

#[derive(Clone)]
struct Board {
    layout: Vec<usize>,
    selected: Vec<bool>,
    positions: HashMap<usize, usize>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let board = (0..25).step_by(5)
                           .map(|idx| {
                               (idx..idx + 5).map(|i| format!("{:2}", self.layout[i]))
                                             .join(" ")
                           })
                           .join("\n");


        write!(f, "{board}")
    }
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let layout: Vec<usize> = value.split_ascii_whitespace()
                                      .map(|v| v.parse().unwrap())
                                      .collect();

        let selected = vec![false; layout.len()];

        let positions = layout.iter()
                              .enumerate()
                              .map(|(i, v)| (*v, i))
                              .collect();

        Board { layout, selected, positions }
    }
}

impl Board {
    fn remove_number(&mut self, value: usize) -> bool {
        if let Some(index) = self.positions.get(&value) {
            self.selected[*index] = true;
            true
        } else {
            false
        }
    }

    fn is_bingo(&self) -> bool {
        for i in (0..25).step_by(5) {
            if self.selected[i..(i + 5)].iter().all(|v| *v) {
                return true;
            }
        }

        for i in 0..5 {
            if (i..25).step_by(5).all(|idx| self.selected[idx]) {
                return true;
            }
        }
        false
    }

    fn score(&self, winning_number: usize) -> usize {
        let sum = self.layout.iter().zip(&self.selected)
                      .filter(|(_, s)| !*s)
                      .map(|(v, _)| *v)
                      .sum::<usize>();

        sum * winning_number
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Board>) {
    let input = input.trim().replace("\r", "");
    let mut lines = input.split("\n\n");

    let numbers = lines.next()
                       .unwrap()
                       .split(",")
                       .map(|v| v.parse::<usize>().unwrap())
                       .collect();

    let boards = lines.map(Board::from).collect();

    (numbers, boards)
}

pub fn solve_a() {
    let (numbers, mut boards) = parse_input(&read_contents(4));

    let ans = get_winning_bingo(&numbers, &mut boards);
    println!("Solution A: {}", ans);
}

fn get_winning_bingo(numbers: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    for &n in numbers {
        for i in 0..boards.len() {
            let board = boards.get_mut(i).unwrap();
            if board.remove_number(n) && board.is_bingo() {
                return board.score(n);
            }
        }
    }
    panic!("No winning bingo board!")
}

pub fn solve_b() {
    let (numbers, mut boards) = parse_input(&read_contents(4));

    let ans = get_last_bingo_board(&numbers, &mut boards);
    println!("Solution B: {}", ans);
}

fn get_last_bingo_board(numbers: &Vec<usize>, boards: &Vec<Board>) -> usize {
    let mut boards = boards.into_iter()
                           .enumerate()
                           .map(|(i, b)| (i, b.clone()))
                           .collect::<HashMap<usize, Board>>();

    let mut index = boards.keys().cloned().collect::<HashSet<_>>();

    for &n in numbers {
        let mut remove = HashSet::new();

        for i in index.iter() {
            let board = boards.get_mut(i).unwrap();
            if board.remove_number(n) && board.is_bingo() {
                remove.insert(*i);
            }
        }

        if index.len() == 1 {
            let board = boards.get(index.iter().next().unwrap()).unwrap();
            if board.is_bingo() {
                return board.score(n);
            }
        } else {
            for i in remove {
                index.remove(&i);
            }
        }
    }

    panic!("Could not determine last board")
}

#[cfg(test)]
mod tests {
    use super::{get_last_bingo_board, get_winning_bingo, parse_input};

    const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_get_winning_bingo() {
        let (numbers, mut boards) = parse_input(TEST_INPUT);

        let ans = get_winning_bingo(&numbers, &mut boards);
        assert_eq!(ans, 4512);
    }

    #[test]
    fn test_get_last_bingo_board() {
        let (numbers, mut boards) = parse_input(TEST_INPUT);

        let ans = get_last_bingo_board(&numbers, &mut boards);
        assert_eq!(ans, 1924);
    }
}
