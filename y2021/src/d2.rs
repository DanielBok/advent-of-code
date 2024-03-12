use crate::inputs::read_contents;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn parse_course(input: &str) -> Vec<Command> {
    input.lines()
         .map(|line| {
             let (cmd, value) = line.trim().split_once(" ").unwrap();
             let value = value.parse::<usize>().unwrap();

             match cmd {
                 "forward" => Command::Forward(value),
                 "down" => Command::Down(value),
                 "up" => Command::Up(value),
                 _ => panic!("Invalid command: {}", cmd)
             }
         })
         .collect()
}

pub fn solve_a() {
    let course = parse_course(&read_contents(2));
    let (x, y) = run_course(&course);

    let ans = x * y;
    println!("Solution A: {}", ans);
}

fn run_course(course: &Vec<Command>) -> (usize, usize) {
    let mut depth = 0;
    let mut horizontal = 0;

    for cmd in course {
        match cmd {
            Command::Forward(x) => { horizontal += x; }
            Command::Up(x) => { depth -= x; }
            Command::Down(x) => { depth += x; }
        }
    }

    (horizontal, depth)
}

pub fn solve_b() {
    let course = parse_course(&read_contents(2));
    let (x, y) = run_course_with_aim(&course);

    let ans = x * y;
    println!("Solution B: {}", ans);
}

fn run_course_with_aim(course: &Vec<Command>) -> (usize, usize) {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for cmd in course {
        match cmd {
            Command::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Command::Up(x) => { aim -= x; }
            Command::Down(x) => { aim += x; }
        }
    }

    (horizontal, depth)
}

#[cfg(test)]
mod tests {
    use super::{parse_course, run_course};

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_run_course() {
        let course = parse_course(TEST_INPUT);
        let (x, y) = run_course(&course);

        assert_eq!(x, 15);
        assert_eq!(y, 10);
    }
}