use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::inputs::read_content;

#[derive(Eq, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}


impl Vector {
    fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    fn from_captures(cap: &Captures, prefix: char) -> Self {
        let get_value = |c: char| -> i64{
            let name = [prefix, c].iter().collect::<String>();
            cap.name(&name)
                .unwrap()
                .as_str()
                .parse()
                .expect(format!("Could not parse: {}", name).as_str())
        };

        Vector { x: get_value('x'), y: get_value('y'), z: get_value('z') }
    }

    fn key(&self) -> [i64; 3] {
        [self.x, self.y, self.z]
    }
}

struct Particle {
    id: usize,
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Particle {
    fn new(id: usize, position: Vector, velocity: Vector, acceleration: Vector) -> Self {
        Particle { id, p: position, v: velocity, a: acceleration }
    }

    fn update(&mut self) {
        self.v.add(&self.a);
        self.p.add(&self.v);
    }

    fn distance_from_origin(&self) -> i64 {
        self.p.x.abs() + self.p.y.abs() + self.p.z.abs()
    }
}

fn get_particles() -> Vec<Particle> {
    let re = Regex::new(r"p=<(?<px>-?\d+),(?<py>-?\d+),(?<pz>-?\d+)>, v=<(?<vx>-?\d+),(?<vy>-?\d+),(?<vz>-?\d+)>, a=<(?<ax>-?\d+),(?<ay>-?\d+),(?<az>-?\d+)>")
        .expect("Could not form regex particle line regex");

    read_content(20).lines()
        .enumerate()
        .map(|(id, line)| {
            let cap = re.captures(line).expect(format!("Could not parse line: {}", line).as_str());

            Particle::new(
                id,
                Vector::from_captures(&cap, 'p'),
                Vector::from_captures(&cap, 'v'),
                Vector::from_captures(&cap, 'a'),
            )
        })
        .collect()
}

pub fn solve_a() {
    let mut particles = get_particles();
    let ans = find_closest_particle(&mut particles);

    println!("Solution A: {}", ans);
}

fn find_closest_particle(particles: &mut Vec<Particle>) -> usize {
    for _ in 0..1000 {
        for p in particles.iter_mut() {
            p.update();
        }
    }

    let mut min_id = 0;
    let mut min_distance = particles[0].distance_from_origin();
    for p in &particles[1..] {
        let distance = p.distance_from_origin();
        if distance < min_distance {
            min_distance = distance;
            min_id = p.id;
        }
    }

    min_id
}

pub fn solve_b() {
    let particles = get_particles();
    let ans = count_remaining_particles_after_collision(particles);

    println!("Solution B: {}", ans);
}

fn count_remaining_particles_after_collision(particles: Vec<Particle>) -> usize {
    let mut particles: HashMap<usize, Particle> = particles.into_iter().map(|p| (p.id, p)).collect();

    remove_collided_particles(&mut particles);

    for _ in 0..1000 {
        for (_, p) in particles.iter_mut() {
            p.update();
        }

        remove_collided_particles(&mut particles);
    }


    particles.len()
}

fn remove_collided_particles(particles: &mut HashMap<usize, Particle>) {
    let mut collisions: HashMap<[i64; 3], Vec<usize>> = HashMap::new();

    for (id, p) in particles.iter() {
        collisions.entry(p.p.key())
            .and_modify(|v| v.push(*id))
            .or_insert(vec![*id]);
    }

    for c in collisions.values() {
        if c.len() > 1 {
            for id in c {
                particles.remove(id);
            }
        }
    }
}