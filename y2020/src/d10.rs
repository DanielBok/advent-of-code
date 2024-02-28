use std::collections::HashMap;

use crate::inputs::read_contents;

fn get_puzzle_input() -> Vec<usize> {
    read_contents(10).lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_a() {
    let mut numbers = get_puzzle_input();

    let ans = threes_times_ones(&mut numbers);
    println!("Solution A: {}", ans);
}

fn threes_times_ones(numbers: &mut Vec<usize>) -> usize {
    numbers.sort();

    let mut ones = 1;
    let mut threes = 1;

    for i in 1..numbers.len() {
        let delta = numbers[i] - numbers[i - 1];

        if delta == 1 {
            ones += 1;
        } else if delta == 3 {
            threes += 1;
        }
    }

    ones * threes
}

pub fn solve_b() {
    let mut numbers = get_puzzle_input();
    let ans = number_of_arrangements(&mut numbers);
    println!("Solution B: {}", ans);
}


fn number_of_arrangements(numbers: &mut Vec<usize>) -> u64 {
    numbers.push(0);
    numbers.sort();

    let last = numbers[numbers.len() - 1] + 3;
    numbers.push(last);
    let mut map: HashMap<usize, u64> = HashMap::from([(0, 1)]);

    for &n in numbers.iter().skip(1) {
        let mut total = 0;
        for i in 1..=3 {
            if let Some(v) = n.checked_sub(i) {
                if let Some(c) = map.get(&v) {
                    total += c;
                }
            }
        }
        map.insert(n, total);
    }

    map[&last]
}

#[cfg(test)]
mod tests {
    use super::{number_of_arrangements, threes_times_ones};

    #[test]
    fn test_solve_a() {
        let mut numbers: Vec<usize> = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3".lines().map(|line| line.parse().unwrap()).collect();

        let ans = threes_times_ones(&mut numbers);
        assert_eq!(ans, 220)
    }

    #[test]
    fn test_number_of_arrangements() {
        for (input, exp) in [
            ("16
10
15
5
1
11
7
19
6
12
4", 8),
            ("28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3", 19208)
        ] {
            let mut numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
            let ans = number_of_arrangements(&mut numbers);
            assert_eq!(ans, exp);
        }
    }
}