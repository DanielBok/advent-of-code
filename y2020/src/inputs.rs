use std::fs;
use std::path::Path;

pub fn read_contents(day: usize) -> String {
    let filepath = Path::new(file!()).parent().unwrap()
        .join(format!("inputs/d{}.txt", day));

    fs::read_to_string(&filepath).unwrap()
}