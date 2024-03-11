use std::collections::HashMap;

/**
Begin in state A.
Perform a diagnostic checksum after 12368930 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state C.

In state B:
  If the current value is 0:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state D.

In state C:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state D.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.

In state D:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state D.

In state E:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state F.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state B.

In state F:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state E.
 **/

enum State { A, B, C, D, E, F }

pub fn solve_a() {
    let mut register: HashMap<i64, usize> = HashMap::new();
    let mut state = State::A;
    let mut index = 0;

    for _ in 0..12_368_930 {
        let v = register.entry(index).or_insert(0);

        match state {
            State::A => {
                if *v == 0 {
                    *v = 1;
                    state = State::B;
                } else {
                    *v = 0;
                    state = State::C;
                }
                index += 1;
            }
            State::B => {
                if *v == 0 {
                    index -= 1;
                    state = State::A;
                } else {
                    index += 1;
                    state = State::D;
                }
                *v = 0;
            }
            State::C => {
                state = if *v == 0 { State::D } else { State::A };
                index += 1;
                *v = 1;
            }
            State::D => {
                if *v == 0 {
                    *v = 1;
                    state = State::E;
                } else {
                    *v = 0;
                    state = State::D;
                }
                index -= 1;
            }
            State::E => {
                if *v == 0 {
                    index += 1;
                    state = State::F;
                } else {
                    index -= 1;
                    state = State::B;
                }
                *v = 1;
            }
            State::F => {
                state = if *v == 0 { State::A } else { State::E };
                index += 1;
                *v = 1;
            }
        }
    }

    let ans = register.values().filter(|&&v| v == 1).count();
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    println!("Completed AOC 2017");
}