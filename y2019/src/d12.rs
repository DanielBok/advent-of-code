use std::ops;

use regex::Regex;

const PUZZLE_INPUT: &str = "<x=-7, y=-1, z=6>
<x=6, y=-9, z=-9>
<x=-12, y=2, z=-7>
<x=4, y=-17, z=-12>";


#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Vector {
    fn from_line(line: &str) -> Vector {
        let re = Regex::new(r"<x=(?<x>[-\s]?\d+), y=(?<y>[-\s]?\d+), z=(?<z>[-\s]?\d+)>").expect("Could not compile regex");
        let Some(caps) = re.captures(line) else {
            panic!("Invalid vector line input: {}", line);
        };

        Vector {
            x: caps["x"].trim().parse::<i64>().expect(format!("Could not parse line to x integer: {line}").as_str()),
            y: caps["y"].trim().parse::<i64>().expect(format!("Could not parse line to y integer: {line}").as_str()),
            z: caps["z"].trim().parse::<i64>().expect(format!("Could not parse line to z integer: {line}").as_str()),
        }
    }

    #[allow(dead_code)]
    fn from_arr(inp: [i64; 3]) -> Vector {
        Vector { x: inp[0], y: inp[1], z: inp[2] }
    }

    fn gravity(&self, rhs: &Self) -> Vector {
        fn compare(p1: i64, p2: i64) -> i64 {
            if p1 == p2 { 0 } else if p1 < p2 { 1 } else { -1 }
        }

        Vector {
            x: compare(self.x, rhs.x),
            y: compare(self.y, rhs.y),
            z: compare(self.z, rhs.z),
        }
    }
}


#[derive(Debug, PartialEq)]
struct Moon {
    pos: Vector,
    vel: Vector,
}

impl Moon {
    fn new(line: &str) -> Moon {
        Moon { pos: Vector::from_line(line), vel: Vector { x: 0, y: 0, z: 0 } }
    }
}

fn simulate(moons: &mut Vec<Moon>, steps: usize) {
    for _ in 0..steps {
        simulate_one_step(moons);
    }
}

fn simulate_one_step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            let g = moons[i].pos.gravity(&moons[j].pos);

            moons[i].vel += g;
            moons[j].vel -= g;
        }
    }

    for moon in moons {
        moon.pos += moon.vel;
    }
}

fn make_moons(input: &str) -> Vec<Moon> {
    input.split('\n').map(|line| Moon::new(line)).collect::<Vec<_>>()
}


pub fn solve_a() {
    let mut moons = make_moons(PUZZLE_INPUT);
    simulate(&mut moons, 1000);

    let total_energy = moons.iter().fold(0, |acc, moon| {
        let pe = moon.pos.x.abs() + moon.pos.y.abs() + moon.pos.z.abs();
        let ke = moon.vel.x.abs() + moon.vel.y.abs() + moon.vel.z.abs();

        acc + pe * ke
    });

    assert_eq!(total_energy, 11384);
    println!("Solution A: {}", total_energy);
}

fn moons_get_axis_values(moons: &Vec<Moon>, axis: char) -> Vec<(i64, i64)> {
    match axis {
        'x' => moons.iter().map(|m| (m.pos.x, m.vel.x)).collect::<Vec<_>>(),
        'y' => moons.iter().map(|m| (m.pos.y, m.vel.y)).collect::<Vec<_>>(),
        'z' => moons.iter().map(|m| (m.pos.z, m.vel.z)).collect::<Vec<_>>(),
        _ => panic!("Invalid axis: {axis}")
    }
}

pub fn solve_b() {
    let mut moons = make_moons(PUZZLE_INPUT);
    let mut steps: Vec<Option<u64>> = vec![None, None, None];
    let mut counter = 0_u64;

    let start_x = moons_get_axis_values(&moons, 'x');
    let start_y = moons_get_axis_values(&moons, 'y');
    let start_z = moons_get_axis_values(&moons, 'z');

    while steps.iter().any(|&x| x.is_none()) {
        counter += 1;
        simulate_one_step(&mut moons);

        if let None = steps[0] {
            if start_x == moons_get_axis_values(&moons, 'x') {
                steps[0] = Some(counter);
            }
        }

        if let None = steps[1] {
            if start_y == moons_get_axis_values(&moons, 'y') {
                steps[1] = Some(counter);
            }
        }

        if let None = steps[2] {
            if start_z == moons_get_axis_values(&moons, 'z') {
                steps[2] = Some(counter);
            }
        }
    }

    let steps = steps.iter().map(|x| x.unwrap()).collect::<Vec<_>>();

    let mut ans = steps[0];
    for x in &steps[1..] {
        ans *= *x / gcd(ans, *x);
    }

    assert_eq!(ans, 452582583272768);
    println!("Solution B: {}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b > 0 {
        (a, b) = (b, a % b)
    }

    a
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::d12::{gcd, make_moons, Moon, simulate_one_step, Vector};

    fn make_test_input() -> Vec<Moon> {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        make_moons(input)
    }

    fn make_expected_moons_vector(input: &str) -> Vec<Moon> {
        let re = Regex::new("pos=(<x=.*, y=.*, z=.*>), vel=(<x=.*, y=.*, z=.*>)").unwrap();
        input.split('\n').map(|line| {
            let caps = re.captures(line).expect(format!("Could not parse line: {}", line).as_str());
            let pos = caps.get(1).expect("Could not get 'pos'").as_str();
            let vel = caps.get(2).expect("Could not get 'vel'").as_str();

            Moon { pos: Vector::from_line(pos), vel: Vector::from_line(vel) }
        }).collect::<Vec<_>>()
    }

    #[test]
    fn test_simulate_one_step() {
        let mut moons = make_test_input();

        for (&ref moon, (pos, vel)) in moons.iter().zip(vec![
            ([-1, 0, 2], [0, 0, 0]),
            ([2, -10, -7], [0, 0, 0]),
            ([4, -8, 8], [0, 0, 0]),
            ([3, 5, -1], [0, 0, 0]),
        ]) {
            let exp = Moon { pos: Vector::from_arr(pos), vel: Vector::from_arr(vel) };
            assert_eq!(*moon, exp);
        }

        for (i, &exp_output) in [
            "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>",
            "pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>
pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>
pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>
pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>",
            "pos=<x= 5, y=-6, z=-1>, vel=<x= 0, y=-3, z= 0>
pos=<x= 0, y= 0, z= 6>, vel=<x=-1, y= 2, z= 4>
pos=<x= 2, y= 1, z=-5>, vel=<x= 1, y= 5, z=-4>
pos=<x= 1, y=-8, z= 2>, vel=<x= 0, y=-4, z= 0>",
            "pos=<x= 2, y=-8, z= 0>, vel=<x=-3, y=-2, z= 1>
pos=<x= 2, y= 1, z= 7>, vel=<x= 2, y= 1, z= 1>
pos=<x= 2, y= 3, z=-6>, vel=<x= 0, y= 2, z=-1>
pos=<x= 2, y=-9, z= 1>, vel=<x= 1, y=-1, z=-1>",
            "pos=<x=-1, y=-9, z= 2>, vel=<x=-3, y=-1, z= 2>
pos=<x= 4, y= 1, z= 5>, vel=<x= 2, y= 0, z=-2>
pos=<x= 2, y= 2, z=-4>, vel=<x= 0, y=-1, z= 2>
pos=<x= 3, y=-7, z=-1>, vel=<x= 1, y= 2, z=-2>",
            "pos=<x=-1, y=-7, z= 3>, vel=<x= 0, y= 2, z= 1>
pos=<x= 3, y= 0, z= 0>, vel=<x=-1, y=-1, z=-5>
pos=<x= 3, y=-2, z= 1>, vel=<x= 1, y=-4, z= 5>
pos=<x= 3, y=-4, z=-2>, vel=<x= 0, y= 3, z=-1>",
            "pos=<x= 2, y=-2, z= 1>, vel=<x= 3, y= 5, z=-2>
pos=<x= 1, y=-4, z=-4>, vel=<x=-2, y=-4, z=-4>
pos=<x= 3, y=-7, z= 5>, vel=<x= 0, y=-5, z= 4>
pos=<x= 2, y= 0, z= 0>, vel=<x=-1, y= 4, z= 2>",
            "pos=<x= 5, y= 2, z=-2>, vel=<x= 3, y= 4, z=-3>
pos=<x= 2, y=-7, z=-5>, vel=<x= 1, y=-3, z=-1>
pos=<x= 0, y=-9, z= 6>, vel=<x=-3, y=-2, z= 1>
pos=<x= 1, y= 1, z= 3>, vel=<x=-1, y= 1, z= 3>",
            "pos=<x= 5, y= 3, z=-4>, vel=<x= 0, y= 1, z=-2>
pos=<x= 2, y=-9, z=-3>, vel=<x= 0, y=-2, z= 2>
pos=<x= 0, y=-8, z= 4>, vel=<x= 0, y= 1, z=-2>
pos=<x= 1, y= 1, z= 5>, vel=<x= 0, y= 0, z= 2>",
            "pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>"
        ].iter().enumerate() {
            simulate_one_step(&mut moons);

            for (&ref moon, exp) in moons.iter().zip(make_expected_moons_vector(exp_output)) {
                assert_eq!(*moon, exp, "Failed at step {}", i + 1);
            }
        }
    }

    #[test]
    fn test_gcd() {
        for (a, b, exp) in [
            (78, 15, 3),
            (99, 53, 1),
            (48, 56, 8),
        ] {
            assert_eq!(gcd(a, b), exp);
        }
    }
}