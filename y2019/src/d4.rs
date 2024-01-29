use std::collections::HashMap;
use std::ops::Range;

const RANGE: Range<i32> = 359282..820401;

fn form_digits(num: i32) -> [i32; 6] {
    [
        num / 10_i32.pow(5) % 10,
        num / 10_i32.pow(4) % 10,
        num / 10_i32.pow(3) % 10,
        num / 10_i32.pow(2) % 10,
        num / 10_i32.pow(1) % 10,
        num / 10_i32.pow(0) % 10,
    ]
}

pub fn solve_a() {
    let mut count = 0;
    'outer: for num in RANGE {
        let digits = form_digits(num);

        let mut has_double = false;
        for (v1, v2) in digits[..5].iter().zip(digits[1..].iter()) {
            if v1 == v2 {
                has_double = true
            }
            if v2 < v1 {
                continue 'outer;
            }
        }

        if has_double {
            count += 1;
        }
    }

    assert_eq!(count, 511);
    println!("Solution A: {}", count);
}

pub fn solve_b() {
    let mut count = 0;
    'outer: for num in RANGE {
        let digits = form_digits(num);

        let mut digit_counts: HashMap<i32, i32> = HashMap::new();
        digit_counts.insert(digits[0], 1);

        for (v1, v2) in digits[..5].iter().zip(digits[1..].iter()) {
            digit_counts.entry(*v2)
                .and_modify(|v| { *v += 1; })
                .or_insert(1);

            if v2 < v1 {
                continue 'outer;
            }
        }

        if digit_counts.values().any(|&v| v == 2) {
            count += 1;
        }
    }

    assert_eq!(count, 316);
    println!("Solution B: {}", count);
}