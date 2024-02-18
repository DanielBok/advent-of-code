use std::thread;

use crossbeam::channel::unbounded;

// Generator A starts with 116
// Generator B starts with 299
const A: u64 = 116;
const B: u64 = 299;

pub fn solve_a() {
    let ans = count_matches(40_000_000, A, B);
    println!("Solution A: {}", ans);
}

fn count_matches(num_pairs: usize, a_start: u64, b_start: u64) -> usize {
    let (sa, ra) = unbounded::<u64>();
    let (sb, rb) = unbounded::<u64>();

    thread::spawn(move || {
        let mut v = a_start;
        for _ in 0..num_pairs {
            v = v * 16807 % 2147483647;
            sa.send(v).unwrap();
        }
    });

    thread::spawn(move || {
        let mut v = b_start;
        for _ in 0..num_pairs {
            v = v * 48271 % 2147483647;
            sb.send(v).unwrap();
        }
    });

    let mut count = 0;
    while let (Ok(a), Ok(b)) = (ra.recv(), rb.recv()) {
        if last_16_bytes_match(a, b) {
            count += 1;
        }
    }

    count
}

fn last_16_bytes_match(a: u64, b: u64) -> bool {
    let a = format!("{:016b}", a);
    let b = format!("{:016b}", b);

    &a[a.len() - 16..] == &b[b.len() - 16..]
}

pub fn solve_b() {
    let ans = count_matches_with_clause(5_000_000, A, B);
    println!("Solution B: {}", ans);
}


fn count_matches_with_clause(num_pairs: usize, a_start: u64, b_start: u64) -> usize {
    let (sa, ra) = unbounded::<u64>();
    let (sb, rb) = unbounded::<u64>();

    thread::spawn(move || {
        let mut v = a_start;
        for _ in 0.. {
            v = v * 16807 % 2147483647;
            if v % 4 == 0 {
                if sa.send(v).is_err() { break; }
            }
        }
    });

    thread::spawn(move || {
        let mut v = b_start;

        for _ in 0.. {
            v = v * 48271 % 2147483647;
            if v % 8 == 0 {
                if sb.send(v).is_err() { break; }
            }
        }
    });

    let mut count = 0;
    let mut pairs = num_pairs;
    while let (Ok(a), Ok(b)) = (ra.recv(), rb.recv()) {
        pairs -= 1;
        if last_16_bytes_match(a, b) {
            count += 1;
        }

        if pairs == 0 {
            // channels are automatically dropped at the end
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::d15::{count_matches, count_matches_with_clause};

    const A: u64 = 65;
    const B: u64 = 8921;

    #[test]
    fn test_count_matches() {
        let ans = count_matches(40_000_000, A, B);
        assert_eq!(ans, 588);
    }

    #[test]
    fn test_count_matches_with_clause() {
        let ans = count_matches_with_clause(5_000_000, A, B);
        assert_eq!(ans, 309);
    }
}