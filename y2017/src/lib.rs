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