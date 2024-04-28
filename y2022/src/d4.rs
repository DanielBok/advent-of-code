use regex::Regex;

use crate::inputs::read_contents;

fn get_numbers() -> Vec<[usize; 4]> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    read_contents(4)
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let x1 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let x2 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let y2 = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();

            [x1, y1, x2, y2]
        })
        .collect()
}

pub fn solve_a() {
    let ans = get_numbers()
        .iter()
        .filter(|[x1, y1, x2, y2]|
            (x2 <= x1 && x1 <= y2 && x2 <= y1 && y1 <= y2)
                || (x1 <= x2 && x2 <= y1 && x1 <= y2 && y2 <= y1)
        )
        .count();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let ans = get_numbers()
        .iter()
        .filter(|[x1, y1, x2, y2]|
            (x2 <= x1 && x1 <= y2)
                || (x2 <= y1 && y1 <= y2)
                || (x1 <= x2 && x2 <= y1)
                || (x1 <= y2 && y2 <= y1)
        )
        .count();

    println!("Solution B: {}", ans);
}