use std::collections::VecDeque;

use itertools::Itertools;

use crate::d15::IntCodeProgram;

const PUZZLE_INPUT: &str = "109,424,203,1,21101,11,0,0,1105,1,282,21102,18,1,0,1106,0,259,1201,1,0,221,203,1,21102,1,31,0,1105,1,282,21101,38,0,0,1106,0,259,20102,1,23,2,21201,1,0,3,21101,1,0,1,21102,57,1,0,1105,1,303,1201,1,0,222,21001,221,0,3,20101,0,221,2,21102,1,259,1,21101,0,80,0,1105,1,225,21101,76,0,2,21102,1,91,0,1106,0,303,2102,1,1,223,21002,222,1,4,21102,1,259,3,21101,0,225,2,21102,225,1,1,21102,1,118,0,1105,1,225,21001,222,0,3,21102,1,54,2,21102,1,133,0,1106,0,303,21202,1,-1,1,22001,223,1,1,21101,148,0,0,1106,0,259,1202,1,1,223,21001,221,0,4,20101,0,222,3,21101,14,0,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21101,0,195,0,106,0,108,20207,1,223,2,20101,0,23,1,21101,0,-1,3,21102,1,214,0,1105,1,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1202,-4,1,249,22102,1,-3,1,21201,-2,0,2,21202,-1,1,3,21101,0,250,0,1106,0,225,22101,0,1,-4,109,-5,2105,1,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2105,1,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,21201,-2,0,-2,109,-3,2105,1,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,22101,0,-2,3,21102,1,343,0,1106,0,303,1106,0,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,22102,1,-4,1,21101,0,384,0,1105,1,303,1106,0,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,21202,1,1,-4,109,-5,2106,0,0";

pub fn solve_a() {
    let ans: i64 = (0..50).cartesian_product(0..50)
        .map(|(x, y)| {
            let mut program = IntCodeProgram::from_str(PUZZLE_INPUT);
            let input = vec![x, y];
            program.append_inputs(&input);
            program.run();

            *program.get_outputs().last().unwrap()
        })
        .sum();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let mut y: usize = 500;
    let mut x = 0;
    while y < 10000 {
        if let Some(first_x) = get_start_x_position(PUZZLE_INPUT, x, y) {
            x = first_x;

            // move 100 cells up and check if starting x is
            let yy = (y - 99) as i64;
            let xx = x as i64;

            if get_output(PUZZLE_INPUT, xx, yy) == 1 &&
                get_output(PUZZLE_INPUT, xx + 99, yy) == 1 {
                break;
            } else {
                y += 1;
            }
        } else {
            y += 100;
        }
    };

    // used to verify answer
    // println!("{} {} {}", x, y, get_output(PUZZLE_INPUT, (x) as i64, (y) as i64));
    // println!("{} {} {}", x + 99, y, get_output(PUZZLE_INPUT, (x + 99) as i64, (y) as i64));
    // println!("{} {} {}", x, y - 99, get_output(PUZZLE_INPUT, (x) as i64, (y - 99) as i64));
    // println!("{} {} {}", x + 99, y - 99, get_output(PUZZLE_INPUT, (x + 99) as i64, (y - 99) as i64));

    let ans = x * 10000 + (y - 99);
    println!("Solution B: {}", ans);
}

// gets the starting x position of a beam of length of at least 100
fn get_start_x_position(input: &str, first_x: usize, y: usize) -> Option<usize> {
    let mut x = first_x as i64;
    let y = y as i64;

    while get_output(input, x, y) != 1 {
        x += 1;
    }

    // check that the 100th position is a beam too
    if get_output(input, x + 99, y) == 1 {
        Some(x as usize)
    } else {
        None
    }
}


fn get_output(input: &str, x: i64, y: i64) -> i64 {
    let mut program = IntCodeProgram::from_str(input);
    program.append_inputs(&vec![x, y]);
    program.run();

    program.get_outputs().pop().unwrap()
}
