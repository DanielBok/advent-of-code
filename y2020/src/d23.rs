use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::ptr;

use indicatif::ProgressIterator;
use itertools::Itertools;

const PUZZLE_INPUT: &str = "562893147";

type NodePtr = *mut Node;

struct Node {
    value: usize,
    prev: NodePtr,
    next: NodePtr,
}

struct List {
    current: NodePtr,
    pointer_map: HashMap<usize, NodePtr>,
    max: usize,
}

fn input_to_vec(input: &str) -> Vec<usize> {
    input.trim().chars().map(|c| { c.to_digit(10).unwrap() as usize }).collect_vec()
}

impl From<Vec<usize>> for List {
    fn from(input: Vec<usize>) -> Self {
        let max = *input.iter().max().unwrap();

        let first = Box::into_raw(Box::new(Node {
            value: input[0],
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }));

        let mut pointer_map: HashMap<usize, NodePtr> = HashMap::from([(input[0], first)]);

        let last = input[1..].iter().fold(first, |prev, &value| {
            let node = Box::into_raw(Box::new(Node { value, prev, next: ptr::null_mut() }));

            unsafe { (*prev).next = node; };
            pointer_map.insert(value, node);

            node
        });

        unsafe {
            (*first).prev = last;
            (*last).next = first;
        }

        List { current: first, pointer_map, max }
    }
}

impl Index<usize> for List {
    type Output = NodePtr;

    fn index(&self, index: usize) -> &Self::Output {
        self.pointer_map.get(&index).unwrap()
    }
}

impl List {
    fn get_destination(&self, exclusions: HashSet<usize>) -> NodePtr {
        let mut value = unsafe { (*self.current).value };
        loop {
            value = value.checked_sub(1).unwrap_or_else(|| self.max);

            if !exclusions.contains(&value) && self.pointer_map.contains_key(&value) {
                break;
            }
        }

        self[value]
    }

    fn move_once(&mut self) {
        unsafe {
            let n1 = (*self.current).next;
            let n2 = (*n1).next;
            let n3 = (*n2).next;
            let n4 = (*n3).next;

            (*self.current).next = n4;
            (*n4).prev = self.current;

            let dst = self.get_destination(HashSet::from([
                (*n1).value,
                (*n2).value,
                (*n3).value,
            ]));

            let dst_next = (*dst).next;

            (*dst).next = n1;
            (*n1).prev = dst;

            (*n3).next = dst_next;
            (*dst_next).prev = n3;

            self.current = (*self.current).next;
        }
    }

    fn move_n(&mut self, n: usize) {
        for _ in (0..n).progress() {
            self.move_once();
        }
    }

    /// Gets number after the "start" value. Ignoring the start value
    fn get_numbers_after(&self, start: usize, limit: usize) -> Vec<usize> {
        let mut curr = unsafe { (*self.pointer_map[&start]).next };

        let limit = min(
            limit,
            self.pointer_map.keys().max().unwrap() - 1,
        );
        let mut order = vec![];
        for _ in 1..=limit {
            unsafe {
                order.push((*curr).value);
                curr = (*curr).next;
            }
        }

        order
    }

    fn len(&self) -> usize {
        self.pointer_map.len()
    }
}


pub fn solve_a() {
    let mut list = List::from(input_to_vec(PUZZLE_INPUT));

    list.move_n(100);
    let ans = list.get_numbers_after(1, list.len()).iter().map(|v| v.to_string()).join("");
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let mut input = input_to_vec(PUZZLE_INPUT);
    input.extend((input.iter().max().unwrap() + 1)..=1_000_000);

    let mut list = List::from(input);
    list.move_n(10_000_000);

    let nums = list.get_numbers_after(1, 2);
    let ans = nums[0] * nums[1];
    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter};

    use itertools::Itertools;

    use super::{input_to_vec, List};

    const TEST_INPUT: &str = "389125467";

    #[test]
    fn test_list_setup() {
        let list = List::from(input_to_vec(TEST_INPUT));

        let mut values = vec![];
        let mut curr = list.current;
        for _ in 0..(TEST_INPUT.len() * 2) {
            unsafe {
                values.push((*curr).value);
                curr = (*curr).next;
            }
        }

        let expected = (0..2).fold(vec![], |mut acc, _| {
            acc.extend(TEST_INPUT.chars().map(|c| c.to_digit(10).unwrap() as usize));
            acc
        });

        assert_eq!(values, expected);
    }

    impl Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let current = unsafe { (*self.current).value };

            let mut values = vec![];
            let mut curr = self.current;
            for _ in 0..self.pointer_map.len() {
                unsafe {
                    let v = (*curr).value;
                    values.push(if v == current { format!("({})", v) } else { v.to_string() });
                    curr = (*curr).next;
                }
            };
            write!(f, "[{}]", values.join(", "))
        }
    }

    #[test]
    fn test_list_move() {
        let mut list = List::from(input_to_vec(TEST_INPUT));

        list.move_n(100);
        let ans = list.get_numbers_after(1, list.len()).iter().map(|v| v.to_string()).join("");
        assert_eq!(ans, "67384529");
    }

    #[test]
    fn test_second_ops() {
        let mut input = input_to_vec(TEST_INPUT);
        input.extend((input.iter().max().unwrap() + 1)..=1_000_000);

        let mut list = List::from(input);
        list.move_n(10_000_000);
        let nums = list.get_numbers_after(1, 2);

        assert_eq!(nums[0], 934001);
        assert_eq!(nums[1], 159792);

        let ans = nums[0] * nums[1];
        assert_eq!(ans, 149245887792);
    }
}