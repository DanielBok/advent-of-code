use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

const PUZZLE_INPUT: &str = "59728776137831964407973962002190906766322659303479564518502254685706025795824872901465838782474078135479504351754597318603898249365886373257507600323820091333924823533976723324070520961217627430323336204524247721593859226704485849491418129908885940064664115882392043975997862502832791753443475733972832341211432322108298512512553114533929906718683734211778737511609226184538973092804715035096933160826733751936056316586618837326144846607181591957802127283758478256860673616576061374687104534470102346796536051507583471850382678959394486801952841777641763547422116981527264877636892414006855332078225310912793451227305425976335026620670455240087933409";

fn read_numbers(input: &str) -> Vec<i32> {
    input.chars()
        .map(|c| c.to_digit(10).map(|n| n as i32).unwrap())
        .collect::<Vec<_>>()
}

pub fn solve_a() {
    let mut nums = read_numbers(PUZZLE_INPUT);
    let ans = format!("{:.8}", convolve(&mut nums, 100));
    assert_eq!(ans.as_str(), "76795888");
    println!("Solution A: {}", ans);
}

fn convolve(nums: &mut Vec<i32>, num_phases: usize) -> String {
    let mut multipliers = HashMap::new();
    for i in 0..nums.len() {
        multipliers.insert(i, get_constant_row(i, nums.len()));
    }

    let mut temp = vec![0; nums.len()];
    for _ in 0..num_phases {
        for i in 0..nums.len() {
            let total = nums.iter()
                .zip(multipliers.get(&i).unwrap())
                .map(|(x, m)| *x * *m)
                .sum::<i32>()
                .abs() % 10;

            temp[i] = (total % 10).abs();
        }

        for i in 0..nums.len() {
            nums[i] = temp[i];
        }
    }

    nums.iter().map(|e| e.to_string()).join("")
}

fn get_constant_row(row_num: usize, length: usize) -> VecDeque<i32> {
    const NUMS: [i32; 4] = [0, 1, 0, -1];
    let mut vec = VecDeque::with_capacity(length);
    let mut c = 1;

    while vec.len() < length {
        vec.push_back(NUMS[(c / (row_num + 1)) % 4]);
        c += 1;
    }

    vec
}

pub fn solve_b() {
    let ans = solve_big(PUZZLE_INPUT);
    assert_eq!(ans, "84024125");
    println!("Solution B: {}", ans);
}

fn solve_big(input: &str) -> String {
    let nums = read_numbers(input);

    let start = nums[0..7].iter().fold(0, |acc, v| acc * 10 + v) as usize;
    let stop = nums.len() * 10000;

    let mut nums = {
        let mut new = Vec::new();
        for i in start..stop {
            new.push(nums[i % nums.len()])
        }
        new
    };

    for _ in 0..100 {
        nums = nums.iter()
            .rev()
            .fold((Vec::new(), 0), |(mut acc, mut sum), v| {
                sum += *v;
                acc.push(sum % 10);

                (acc, sum)
            }).0;

        nums.reverse();
    }

    nums[0..8]
        .iter()
        .map(|v| v.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_get_constant_row() {
        use super::get_constant_row;
        assert_eq!(get_constant_row(0, 6), VecDeque::from([1, 0, -1, 0, 1, 0]));
        assert_eq!(get_constant_row(1, 6), VecDeque::from([0, 1, 1, 0, 0, -1]));
        assert_eq!(get_constant_row(2, 6), VecDeque::from([0, 0, 1, 1, 1, 0]));
    }

    #[test]
    fn test_convolve() {
        use super::{convolve, read_numbers};

        for (inp, exp) in [("80871224585914546619083218645595", "24176176"),
            ("19617804207202209144916044189917", "73745418"),
            ("69317163492948606335995924319873", "52432133")] {
            let mut nums = read_numbers(inp);
            let res = convolve(&mut nums, 100);
            assert_eq!(res[..8], *exp);
        }
    }

    #[test]
    fn test_solve_big() {
        use super::solve_big;

        for (inp, exp) in [
            ("03036732577212944063491565474664", "84462026"),
            ("02935109699940807407585447034323", "78725270"),
            ("03081770884921959731165446850517", "53553731"),
        ] {
            let ans = solve_big(inp);
            assert_eq!(ans, exp);
        }
    }
}