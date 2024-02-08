pub fn solve(day: usize) {
    match day {
        0 => {
            for i in 1..=25 {
                println!("Day {i}");
                solve(i);
            }
            ((), ())
        }
        _ => {
            panic!("Day {day} not implemented. ");
        }
    };
}