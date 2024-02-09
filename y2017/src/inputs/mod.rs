use std::fs;
use std::path::Path;

pub fn read_content(day: usize) -> String {
    let directory = Path::new(file!()).parent().unwrap();
    let path = directory.join(format!("d{}.txt", day));

    fs::read_to_string(&path)
        .expect(format!("Could not find file '{}'", path.to_str().unwrap()).as_str())
}