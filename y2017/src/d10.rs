use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

fn get_input() -> Vec<usize> {
    vec![70, 66, 255, 2, 48, 0, 54, 48, 80, 141, 244, 254, 160, 108, 1, 41]
}

struct KnotHash {
    position: HashMap<usize, usize>,
    verbose: bool,
}

impl KnotHash {
    fn new(n: usize) -> KnotHash {
        assert!(n > 0, "size must be greater than 0");
        let mut position = HashMap::new();

        for i in 0..n {
            position.insert(i, i);
        }

        KnotHash { position, verbose: false }
    }

    fn len(&self) -> usize {
        self.position.len()
    }

    fn apply_knots(&mut self, knot_lengths: &[usize], times: usize) {
        assert!(times > 0, "times must be > 0");
        let mut skip_size = 0;
        let mut current = 0;

        let n = self.len();

        for _ in 0..times {
            for &knot_length in knot_lengths {
                if self.verbose {
                    println!("State: \nP: {}\nC: {}\nS: {}\n\n", self, current, skip_size);
                }

                self.apply_knot(current, knot_length);
                current = (current + knot_length + skip_size) % n;
                skip_size += 1;
            }
        }

        if self.verbose {
            println!("State: \nP: {}\nC: {}\nS: {}\n\n", self, current, skip_size);
        }
    }

    fn apply_knot(&mut self, current: usize, knot_length: usize) {
        let mut temp = vec![0; knot_length];
        let n = self.len();

        for (ri, ci) in (current..(current + knot_length)).enumerate() {
            temp[knot_length - ri - 1] = self.position[&(ci % n)];
        }

        for (ci, v) in (current..(current + knot_length)).zip(temp) {
            self.position.insert(ci % n, v);
        }
    }

    fn hash(&self) -> String {
        assert_eq!(self.len() % 16, 0, "Hash only works when KnotHash input size is a multiple of 16");

        (0..self.len())
            .step_by(16)
            .map(|i| {
                ((i + 1)..(i + 16)).fold(self.position[&i], |acc, x| {
                    acc ^ self.position[&x]
                })
            })
            .map(|v| format!("{:02x}", v))
            .collect::<String>()
    }
}

impl Display for KnotHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = (0..self.len()).map(|i| self.position[&i].to_string()).join(", ");
        write!(f, "{}", v)
    }
}


pub fn solve_a() {
    let inputs = get_input();
    let mut sk = KnotHash::new(256);
    sk.apply_knots(&inputs, 1);

    let ans = sk.position[&0] * sk.position[&1];

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let input = format_input(&get_input()
        .iter()
        .map(|v| v.to_string())
        .join(",")
    );
    let ans = knot_hash(&input);

    assert_eq!(&ans, "decdf7d377879877173b7f2fb131cf1b");
    println!("Solution B: {}", ans);
}

pub fn format_input(input: &str) -> Vec<usize> {
    let mut inp = input.chars().map(|c| c as usize).collect_vec();
    inp.extend([17, 31, 73, 47, 23]);

    inp
}

pub fn knot_hash(input: &Vec<usize>) -> String {
    let mut sk = KnotHash::new(256);
    sk.apply_knots(&input, 64);

    sk.hash()
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::d10::KnotHash;

    #[test]
    fn test_apply_knots() {
        let mut sk = KnotHash::new(5);
        sk.verbose = true;
        let inputs = [3, 4, 1, 5];

        sk.apply_knots(&inputs, 1);

        let v = (0..sk.len()).map(|i| sk.position[&i]).collect_vec();
        assert_eq!(v, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn test_hash() {
        for (hash, exp) in [
            ("", "a2582a3a0e66e6e86e3812dcb672a272"),
            ("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"),
            ("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"),
            ("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"),
        ] {
            let mut input = hash.chars()
                .map(|v| v as usize)
                .collect_vec();


            input.extend([17, 31, 73, 47, 23]);

            let mut sk = KnotHash::new(256);
            sk.apply_knots(&input, 64);
            let ans = sk.hash();

            assert_eq!(ans, exp, "Hash: '{}'", hash);
        }
    }
}