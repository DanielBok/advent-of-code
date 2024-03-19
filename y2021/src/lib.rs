mod inputs;
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;

pub fn solve(day: usize) {
    match day {
        1 => { (d1::solve_a(), d1::solve_b()); }
        2 => { (d2::solve_a(), d2::solve_b()); }
        3 => { (d3::solve_a(), d3::solve_b()); }
        4 => { (d4::solve_a(), d4::solve_b()); }
        5 => { (d5::solve_a(), d5::solve_b()); }
        6 => { (d6::solve_a(), d6::solve_b()); }
        7 => { (d7::solve_a(), d7::solve_b()); }
        8 => { (d8::solve_a(), d8::solve_b()); }
        9 => { (d9::solve_a(), d9::solve_b()); }
        10 => { (d10::solve_a(), d10::solve_b()); }
        11 => { (d11::solve_a(), d11::solve_b()); }
        12 => { (d12::solve_a(), d12::solve_b()); }
        13 => { (d13::solve_a(), d13::solve_b()); }
        14 => { (d14::solve_a(), d14::solve_b()); }
        15 => { (d15::solve_a(), d15::solve_b()); }
        // 16 => { (d16::solve_a(), d16::solve_b()); }
        // 17 => { (d17::solve_a(), d17::solve_b()); }
        // 18 => { (d18::solve_a(), d18::solve_b()); }
        // 19 => { (d19::solve_a(), d19::solve_b()); }
        // 20 => { (d20::solve_a(), d20::solve_b()); }
        // 21 => { (d21::solve_a(), d21::solve_b()); }
        // 22 => { (d22::solve_a(), d22::solve_b()); }
        // 23 => { (d23::solve_a(), d23::solve_b()); }
        // 24 => { (d24::solve_a(), d24::solve_b()); }
        // 25 => { (d25::solve_a(), d25::solve_b()); }
        0 => {
            for i in 1..=25 {
                println!("Day {i}");
                solve(i);
            }
        }
        _ => {
            panic!("Day {day} not implemented. ");
        }
    };
}