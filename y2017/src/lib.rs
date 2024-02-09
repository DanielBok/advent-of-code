mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod inputs;

pub fn solve(day: usize) {
    match day {
        1 => { (d1::solve_a(), d1::solve_b()); }
        2 => { (d2::solve_a(), d2::solve_b()); }
        3 => { (d3::solve_a(), d3::solve_b()); }
        4 => { (d4::solve_a(), d4::solve_b()); }
        5 => { (d5::solve_a(), d5::solve_b()); }
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