use super::inputs::read_content;

fn get_jumps() -> Vec<i32> {
    read_content(5)
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn solve_a() {
    let mut jumps = get_jumps();

    let mut p = 0;
    let mut num = 0;
    while p < jumps.len() {
        num += 1;
        let j = jumps[p];

        let mut pp = p as i32;
        pp += j;
        if pp < 0 { break; }

        jumps[p] += 1;
        p = pp as usize;
    }

    println!("Solution A: {}", num)
}

pub fn solve_b() {
    let mut jumps = get_jumps();

    let mut p = 0;
    let mut num = 0;
    while p < jumps.len() {
        num += 1;
        let j = jumps[p];

        let mut pp = p as i32;
        pp += j;
        if pp < 0 { break; }

        jumps[p] += if j >= 3 { -1 } else { 1 };
        p = pp as usize;
    }

    println!("Solution B: {}", num)
}
