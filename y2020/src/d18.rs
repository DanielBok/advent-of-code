use std::fmt::{Debug, Formatter};

use crate::inputs::read_contents;

enum Operator { Add, Mul, None }

enum Element {
    Integer(usize),
    Add,
    Mul,
    ParOpen,
    ParClose,
}


impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Element::Integer(v) => format!("{}", v).chars().next().unwrap(),
            Element::Add => '+',
            Element::Mul => '*',
            Element::ParOpen => '(',
            Element::ParClose => ')',
        };

        write!(f, "{c}")
    }
}

fn parse_input(input: &str) -> Vec<Vec<Element>> {
    use Element::*;

    input.lines()
         .map(|line| {
             let mut question = vec![];

             for c in line.chars() {
                 if c.is_whitespace() { continue; }
                 question.push(match c {
                     '(' => ParOpen,
                     ')' => ParClose,
                     '+' => Add,
                     '*' => Mul,
                     _ if c.is_digit(10) => Integer(c.to_digit(10).expect(&format!("Could not parse '{}' to digit", c)) as usize),
                     _ => panic!("Could not parse '{c}' to any elements")
                 });
             }

             question
         })
         .collect()
}

pub fn solve_a() {
    let questions = parse_input(&read_contents(18));
    let ans: usize = questions.iter()
                              .map(|q| calculate(q).0)
                              .sum();
    println!("Solution A: {}", ans);
}

fn calculate(elements: &[Element]) -> (usize, usize) {
    let mut i = 0;

    let mut number = None;
    let mut operator = Operator::None;

    while i < elements.len() {
        match elements[i] {
            Element::Integer(v) => {
                if let Some(num) = number {
                    number = Some(match operator {
                        Operator::Add => { num + v }
                        Operator::Mul => { num * v }
                        _ => panic!("Missing operator for number operations")
                    });
                    operator = Operator::None;
                    i += 1;
                } else {
                    number = Some(v);
                    operator = match elements[i + 1] {
                        Element::Add => Operator::Add,
                        Element::Mul => Operator::Mul,
                        _ => panic!("Invalid operator: {:?}", elements[i + 1])
                    };
                    i += 2;
                }
            }
            Element::ParOpen => {
                let (next, offset) = calculate(&elements[(i + 1)..]);
                if let Some(num) = number {
                    number = Some(match operator {
                        Operator::Add => { num + next }
                        Operator::Mul => { num * next }
                        _ => panic!("Missing operator for number operations")
                    })
                } else {
                    number = Some(next);
                }
                i += offset + 1;
            }
            Element::ParClose => {
                i += 1;
                return (number.unwrap(), i);
            }
            Element::Add => {
                operator = Operator::Add;
                i += 1;
            }
            Element::Mul => {
                operator = Operator::Mul;
                i += 1;
            }
        }
    }

    (number.unwrap(), i)
}

pub fn solve_b() {
    let questions = parse_input(&read_contents(18));
    let ans: usize = questions.iter()
                              .map(|q| calculate_with_priority(q).0)
                              .sum();
    println!("Solution B: {}", ans);
}


fn calculate_with_priority(elements: &[Element]) -> (usize, usize) {
    let mut i = 0;

    let mut numbers = vec![];
    let mut operators = vec![];

    while i < elements.len() {
        let n = numbers.len();

        match elements[i] {
            Element::Integer(v) => {
                if numbers.is_empty() {
                    numbers.push(v);
                    i += 1;
                } else {
                    let operator = operators.pop().unwrap();
                    match operator {
                        Operator::Add => {
                            numbers[n - 1] += v;
                        }
                        Operator::Mul => {
                            numbers.push(v);
                            operators.push(operator);
                        }
                        _ => panic!("None should not exist in operators")
                    }

                    i += 1;
                }
            }
            Element::ParOpen => {
                let (next, offset) = calculate_with_priority(&elements[(i + 1)..]);
                if numbers.is_empty() {
                    numbers.push(next);
                } else {
                    let operator = operators.pop().unwrap();
                    match operator {
                        Operator::Add => {
                            numbers[n - 1] += next;
                        }
                        Operator::Mul => {
                            numbers.push(next);
                            operators.push(operator);
                        }
                        Operator::None => panic!("Should not happen")
                    }
                }
                i += offset + 1;
            }
            Element::ParClose => {
                i += 1;
                break;
            }
            Element::Add => {
                operators.push(Operator::Add);
                i += 1;
            }
            Element::Mul => {
                operators.push(Operator::Mul);
                i += 1;
            }
        }
    }

    let mut number = numbers.pop().unwrap();
    while let Some(op) = operators.pop() {
        let n = numbers.pop().unwrap();
        match op {
            Operator::Add => { number += n }
            Operator::Mul => { number *= n }
            Operator::None => panic!("Shouldn't happen")
        }
    }

    assert_eq!(numbers.len(), 0);
    (number, i)
}

#[cfg(test)]
mod tests {
    use itertools::izip;

    use super::{calculate, calculate_with_priority, parse_input};

    #[test]
    fn test_equation() {
        let input = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        let questions = parse_input(input);

        for (q, exp) in izip!(
            questions,
            [71, 51, 26, 437, 12240, 13632]
        ) {
            let (ans, _) = calculate(&q);
            assert_eq!(ans, exp);
        }
    }

    #[test]
    fn test_calculate_with_priority() {
        let input = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        let questions = parse_input(input);

        for (q, exp) in izip!(
            questions,
            [231, 51, 46, 1445, 669060, 23340]
        ) {
            let (ans, _) = calculate_with_priority(&q);
            assert_eq!(ans, exp);
        }
    }
}