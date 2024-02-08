mod d1;

pub fn solve(day: usize) {
    match day {
        1 => { (d1::solve_a(), d1::solve_b()); }
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