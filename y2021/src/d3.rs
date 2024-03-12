use std::collections::HashMap;

use crate::inputs::read_contents;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
         .map(|line| line.chars()
                         .map(|c| match c {
                             '0' => 0,
                             '1' => 1,
                             _ => panic!("Invalid char: '{}'", c)
                         })
                         .collect())
         .collect()
}

pub fn solve_a() {
    let numbers = parse_input(&read_contents(3));
    let (gamma, epsilon) = get_gamma_epsilon(&numbers);

    let ans = gamma * epsilon;
    println!("Solution A: {}", ans);
}

fn get_gamma_epsilon(numbers: &Vec<Vec<u8>>) -> (usize, usize) {
    let mut counter: HashMap<usize, usize> = HashMap::new();

    for num in numbers {
        for (i, &n) in num.iter().enumerate() {
            counter.entry(i)
                   .and_modify(|v| *v += if n == 1 { 1 } else { 0 })
                   .or_insert(if n == 1 { 1 } else { 0 });
        }
    }

    let total = numbers.len();

    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, p) in (0..=*counter.keys().max().unwrap()).rev().enumerate() {
        if counter[&p] * 2 > total {
            gamma += 2_usize.pow(i as u32);
        } else {
            epsilon += 2_usize.pow(i as u32);
        }
    }

    (gamma, epsilon)
}

pub fn solve_b() {
    let numbers = parse_input(&read_contents(3));
    let (oxygen_rating, co2_rating) = get_oxygen_and_co2_ratings(&numbers);
    let ans = oxygen_rating * co2_rating;

    println!("Solution B: {}", ans);
}

fn get_oxygen_and_co2_ratings(numbers: &Vec<Vec<u8>>) -> (usize, usize) {
    let oxygen_rating = find_rating(&numbers, |row0, row1| if row1.len() >= row0.len() { row1 } else { row0 });
    let co2_rating = find_rating(&numbers, |row0, row1| if row0.len() <= row1.len() { row0 } else { row1 });

    (oxygen_rating, co2_rating)
}

fn find_rating<F>(numbers: &Vec<Vec<u8>>,
                  conditional: F) -> usize
    where F: Fn(Vec<usize>, Vec<usize>) -> Vec<usize>
{
    let mut numbers = numbers.clone();

    let mut index = 0;
    while numbers.len() > 1 {
        let (rows0, rows1) = count_values(&numbers, index);
        numbers = form_next_numbers_vector(conditional(rows0, rows1), &numbers);
        index += 1;
    }
    assert!(!numbers.is_empty(), "No results left");

    binary_to_value(&numbers[0])
}

fn count_values(numbers: &Vec<Vec<u8>>, index: usize) -> (Vec<usize>, Vec<usize>) {
    let mut rows1 = vec![];
    let mut rows0 = vec![];

    for (i, vec) in numbers.iter().enumerate() {
        if vec[index] == 1 {
            rows1.push(i);
        } else {
            rows0.push(i);
        }
    }

    (rows0, rows1)
}

fn form_next_numbers_vector(index: Vec<usize>, original: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut next = vec![];
    for i in index {
        next.push(original[i].clone())
    }

    next
}

fn binary_to_value(binary: &Vec<u8>) -> usize {
    let mut value = 0;

    for (i, p) in binary.iter().rev().enumerate() {
        if *p == 1 {
            value += 2_usize.pow(i as u32)
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::{get_gamma_epsilon, get_oxygen_and_co2_ratings, parse_input};

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_gamma_epsilon() {
        let numbers = parse_input(TEST_INPUT);
        let (gamma, epsilon) = get_gamma_epsilon(&numbers);

        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn test_find_ratings() {
        let numbers = parse_input(TEST_INPUT);

        let (oxygen_rating, co2_rating) = get_oxygen_and_co2_ratings(&numbers);
        assert_eq!(oxygen_rating, 23);
        assert_eq!(co2_rating, 10);
    }
}