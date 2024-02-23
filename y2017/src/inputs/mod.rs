use std::fs;
use std::path::Path;

pub fn read_content(day: usize) -> String {
    let mut directory = Path::new(file!()).parent().unwrap();

    let curr_dir = std::env::current_dir().unwrap();
    if curr_dir.ends_with("y2017") {
        let mut components = directory.components();
        components.next();
        directory = components.as_path();
    }

    let path = directory.join(format!("d{}.txt", day));

    fs::read_to_string(&path)
        .expect(format!("Could not find file '{}'", path.to_str().unwrap()).as_str())
}