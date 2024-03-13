use std::collections::{HashMap, VecDeque};

use crate::inputs::read_contents;

fn parse_input(input: &str) -> Vec<usize> {
    input.split(",")
         .map(|x| x.parse::<usize>().unwrap())
         .collect()
}

pub fn solve_a() {
    let fishes = parse_input(&read_contents(6));
    let ans = count_total_spawns(&fishes, 80);

    println!("Solution A: {}", ans);
}

fn count_total_spawns(fishes: &[usize], end_day: usize) -> usize {
    let mut queue = fishes.iter()
                          .cloned()
                          .collect::<VecDeque<_>>();

    let mut cache: HashMap<usize, usize> = HashMap::new();
    let mut total = queue.len();

    while let Some(start_day) = queue.pop_front() {
        total += count_spawns(end_day, start_day, &mut cache);
    }

    total
}

fn count_spawns(end_day: usize, start_day: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if start_day > end_day {
        return 0;
    } else if let Some(num_spawns) = cache.get(&start_day) {
        return *num_spawns;
    }

    let num_spawns = (end_day - start_day).div_ceil(7) +
        ((start_day + 1)..=end_day).step_by(7)
                                   .map(|next_start_day| count_spawns(end_day, next_start_day + 8, cache))
                                   .sum::<usize>();

    cache.insert(start_day, num_spawns);

    num_spawns
}

pub fn solve_b() {
    let fishes = parse_input(&read_contents(6));
    let ans = count_total_spawns(&fishes, 256);
    
    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::{count_total_spawns, parse_input};

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_count_total_spawns() {
        let fishes = parse_input(TEST_INPUT);

        for (end_day, exp) in [
            (18, 26),
            (80, 5934),
            (256, 26984457539)
        ] {
            let ans = count_total_spawns(&fishes, end_day);
            assert_eq!(ans, exp);
        }
    }
}