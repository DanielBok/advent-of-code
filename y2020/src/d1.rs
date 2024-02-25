use crate::inputs::read_contents;

fn get_numbers() -> Vec<i32> {
    let mut numbers: Vec<i32> = read_contents(1)
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    numbers.sort();
    numbers
}

pub fn solve_a() {
    let numbers = get_numbers();
    let ans = get_product_of_2020_sum(&numbers).unwrap();

    println!("Solution A: {}", ans);
}

fn get_product_of_2020_sum(numbers: &Vec<i32>) -> Option<i32> {
    let mut p1 = 0;
    let mut p2 = numbers.len() - 1;

    while p1 < p2 {
        let v1 = &numbers[p1];
        let v2 = &numbers[p2];

        if v1 + v2 == 2020 {
            return Some(v1 * v2);
        }

        if v1 + v2 > 2020 {
            p2 -= 1;
        } else {
            p1 += 1;
        }
    }
    None
}

pub fn solve_b() {
    let numbers = get_numbers();
    let ans = three_sum(&numbers).unwrap();

    println!("Solution B: {}", ans);
}

fn three_sum(numbers: &Vec<i32>) -> Option<i32> {
    for (p1, v1) in numbers.iter().enumerate() {
        let mut p2 = p1 + 1;
        let mut p3 = numbers.len() - 1;

        while p2 < p3 {
            let v2 = numbers[p2];
            let v3 = numbers[p3];

            let sum = v1 + v2 + v3;
            if sum == 2020 {
                return Some(v1 * v2 * v3);
            }

            if sum > 2020 {
                p3 -= 1;
            } else {
                p2 += 1;
            }
        }
    }

    None
}