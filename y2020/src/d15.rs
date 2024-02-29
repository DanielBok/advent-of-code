use std::collections::HashMap;

fn get_puzzle_input() -> Vec<usize> {
    vec![16, 1, 0, 18, 12, 14, 19]
}

pub fn solve_a() {
    let input = get_puzzle_input();
    let ans = get_number(&input, 2020);

    println!("Solution A: {}", ans);
}

fn get_number(input: &Vec<usize>, position: usize) -> usize {
    assert!(position > 0);
    let mut spoken = HashMap::new();

    for (i, v) in input.iter().enumerate() {
        spoken.insert(*v, i + 1);
    }

    if position <= spoken.len() {
        return spoken.into_iter().find(|(_, v)| *v == position).unwrap().0;
    }

    let mut next_number = 0;
    let start = spoken.len() + 1;
    for i in start..position {
        let num = next_number;
        if let Some(last_pos) = spoken.get(&next_number) {
            next_number = i - last_pos;
        } else {
            next_number = 0;
        }

        spoken.insert(num, i);
    }

    next_number
}

pub fn solve_b() {
    let input = get_puzzle_input();
    let ans = get_number(&input, 30_000_000);

    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use crate::d15::get_number;

    #[test]
    fn test_get_number() {
        for (input, pos, exp) in [
            (vec![0, 3, 6], 4, 0),
            (vec![0, 3, 6], 5, 3),
            (vec![0, 3, 6], 6, 3),
            (vec![0, 3, 6], 7, 1),
            (vec![0, 3, 6], 8, 0),
            (vec![0, 3, 6], 9, 4),
            (vec![0, 3, 6], 10, 0),
            (vec![0, 3, 6], 2020, 436),
            (vec![0, 3, 6], 30_000_000, 175594),
        ] {
            let ans = get_number(&input, pos);
            assert_eq!(ans, exp);
        }
    }
}