use std::collections::{HashSet, VecDeque};

use crate::inputs::read_contents;

fn form_numbers(input: &str) -> Vec<usize> {
    input.lines()
        .map(|line| line.parse().expect(&format!("Could not parse '{line}' as integer")))
        .collect()
}

pub fn solve_a() {
    let numbers: Vec<usize> = form_numbers(&read_contents(9));
    let ans = find_first_invalid_number(&numbers, 25);
    println!("Solution A: {}", ans);
}

fn find_first_invalid_number(numbers: &Vec<usize>, preamble_length: usize) -> usize {
    let mut preamble = numbers[..preamble_length].iter().cloned().collect::<HashSet<_>>();

    'outer: for i in preamble_length..numbers.len() {
        let target = numbers.get(i).unwrap();

        for n in &numbers[(i - preamble_length)..(i - 1)] {
            if let Some(complement) = target.checked_sub(*n) {
                if complement != *n && preamble.contains(n) && preamble.contains(&complement) {
                    preamble.remove(&numbers[i - preamble_length]);
                    preamble.insert(*target);

                    continue 'outer;
                }
            }
        }

        return *target;
    }

    panic!("Could not find any invalid number");
}

pub fn solve_b() {
    let numbers: Vec<usize> = form_numbers(&read_contents(9));
    let target = find_first_invalid_number(&numbers, 25);
    let ans = find_numbers_summing_to_target(&numbers, target);

    println!("Solution B: {}", ans);
}

fn find_numbers_summing_to_target(numbers: &Vec<usize>, target: usize) -> usize {
    let mut queue: VecDeque<(usize, usize, usize)> = numbers.iter().cloned().enumerate()
        .filter(|(_, v)| *v < target)
        .map(|(i, v)| (i, v, 1))
        .collect();

    while let Some((index, sum, length)) = queue.pop_front() {
        if sum == target && length > 2 {
            let nums = &numbers[index..=(index + length)];
            return nums.iter().max().unwrap() + nums.iter().min().unwrap();
        } else if sum > target || (sum == target && length <= 2) {
            continue;
        }
        queue.push_front((index, sum + numbers[index + length], length + 1));
    }

    panic!("Could not find any numbers that sum to {}", target)
}


#[cfg(test)]
mod tests {
    use super::{find_first_invalid_number, form_numbers};

    #[test]
    fn test_find_first_invalid_number() {
        let numbers = form_numbers("35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576");

        let ans = find_first_invalid_number(&numbers, 5);
        assert_eq!(ans, 127);
    }
}
