use std::cmp::max;
use std::collections::HashMap;

const P1: usize = 3;
const P2: usize = 7;

pub fn solve_a() {
    let ans = simulate_deterministic(P1, P2, 100, 1000);
    println!("Solution A: {}", ans);
}

fn simulate_deterministic(p1: usize, p2: usize, sides: usize, target: usize) -> usize {
    let get_offset = |i: usize| -> usize {
        let get_number = |n: usize| -> usize {
            if n == 0 { sides } else { n }
        };

        let base = i * 3;
        get_number((base + 1) % sides) +
            get_number((base + 2) % sides) +
            get_number((base + 3) % sides)
    };

    fn get_position(next_p: usize) -> usize {
        if next_p == 0 { 10 } else { next_p }
    }

    let mut p1 = p1;
    let mut p2 = p2;
    let mut s1 = 0;
    let mut s2 = 0;

    for i in 0..usize::MAX {
        let offset = get_offset(i);
        if i % 2 == 0 {
            p1 = get_position((p1 + offset) % 10);
            s1 += p1;
        } else {
            p2 = get_position((p2 + offset) % 10);
            s2 += p2;
        }

        if s1 >= target {
            return s2 * (i + 1) * 3;
        } else if s2 >= target {
            return s1 * (i + 1) * 3;
        }
    };

    unreachable!()
}

pub fn solve_b() {
    let mut sim = MultiDimensionSimulator::new(21);
    let (w1, w2) = sim.simulate(P1, P2, 0, 0, true);
    
    let ans = max(w1, w2);
    println!("Solution B: {}", ans);
}


struct MultiDimensionSimulator {
    winning_score: usize,
    cache: HashMap<(usize, usize, usize, usize, bool), (usize, usize)>,
}

impl MultiDimensionSimulator {
    fn new(winning_score: usize) -> Self {
        Self { cache: HashMap::new(), winning_score }
    }

    fn simulate(
        &mut self,
        p1: usize,
        p2: usize,
        s1: usize,
        s2: usize,
        play1: bool,
    ) -> (usize, usize) {
        fn get_position(next_p: usize) -> usize {
            if next_p == 0 { 10 } else { next_p }
        }

        if s1 >= self.winning_score && s2 >= self.winning_score {
            unreachable!()
        } else if s1 >= self.winning_score {
            return (1, 0);
        } else if s2 >= self.winning_score {
            return (0, 1);
        }

        let mut w1 = 0;
        let mut w2 = 0;

        let key = (p1, p2, s1, s2, play1);
        if let Some(res) = self.cache.get(&key) {
            return *res;
        }

        for (total_roll, freq) in [
            (3, 1),
            (4, 3),
            (5, 6),
            (6, 7),
            (7, 6),
            (8, 3),
            (9, 1),
        ] {
            let (a1, a2) = if play1 {
                let np = get_position((p1 + total_roll) % 10);
                self.simulate(np, p2, s1 + np, s2, !play1)
            } else {
                let np = get_position((p2 + total_roll) % 10);
                self.simulate(p1, np, s1, s2 + np, !play1)
            };

            w1 += freq * a1;
            w2 += freq * a2;
        }

        self.cache.insert(key, (w1, w2));
        
        (w1, w2)
    }
}


#[cfg(test)]
mod tests {
    use super::{simulate_deterministic, MultiDimensionSimulator};

    #[test]
    fn test_simulate_deterministic() {
        let ans = simulate_deterministic(4, 8, 100, 1000);
        assert_eq!(ans, 739785);
    }

    #[test]
    fn test_simulate_multi_dimension_game() {
        let mut sim = MultiDimensionSimulator::new(21);
        let (w1, w2) = sim.simulate(4, 8, 0, 0, true);
        
        assert_eq!(w1, 444356092776315);
        assert_eq!(w2, 341960390180808);
    }
}
