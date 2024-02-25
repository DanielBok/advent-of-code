use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use once_cell::sync::OnceCell;

use crate::inputs::read_content;

#[derive(Clone, Eq)]
struct Component {
    pin1: usize,
    pin2: usize,
}

impl Component {
    fn new(input: &str) -> Self {
        let (a, b) = input.split_once('/').unwrap();

        let mut a = a.parse().unwrap();
        let mut b = b.parse().unwrap();

        // flip the pins so the smallest always in front
        if a > b {
            (a, b) = (b, a)
        };
        Component { pin1: a, pin2: b }
    }

    fn other_pin(&self, from_pin: usize) -> usize {
        assert!(self.pin1 == from_pin || self.pin2 == from_pin,
                "Component missing pin {}. Pins are {}/{}", from_pin, self.pin1, self.pin2);

        if self.pin1 == from_pin {
            self.pin2
        } else {
            self.pin1
        }
    }

    fn strength(&self) -> usize {
        self.pin1 + self.pin2
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.pin1 == other.pin1 && self.pin2 == other.pin2
    }
}

impl Hash for Component {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.pin1, self.pin2).hash(state);
    }
}

struct Catalogue(HashMap<usize, Vec<Component>>);

impl Catalogue {
    fn get(&self, value: usize) -> &Vec<Component> {
        self.0.get(&value).unwrap()
    }
}


fn get_component_catalogue(input: &str) -> Catalogue {
    let mut catalogue = HashMap::new();

    input.lines().for_each(|line| {
        let c = Component::new(line);

        catalogue.entry(c.pin1)
            .and_modify(|v: &mut Vec<_>| v.push(c.clone()))
            .or_insert(vec![c.clone()]);

        if c.pin2 != c.pin1 {
            catalogue.entry(c.pin2)
                .and_modify(|v| v.push(c.clone()))
                .or_insert(vec![c.clone()]);
        }
    });

    Catalogue(catalogue)
}

fn find_strongest_bridge(input: &str) -> (usize, usize) {
    let mut queue = VecDeque::new();
    let catalogue = get_component_catalogue(input);

    for c in catalogue.get(0) {
        let seen = HashSet::from([c.clone()]);
        queue.push_back((c.pin2, c.pin2, seen));
    }

    let mut highest_strength = 0;
    let mut longest_strength = (0, 0);

    while let Some((from_pin, cum_strength, seen)) = queue.pop_front() {
        for nb in catalogue.get(from_pin) {
            if !seen.contains(nb) {
                let mut seen = seen.clone();
                seen.insert(nb.clone());
                let strength = cum_strength + nb.strength();

                if seen.len() > longest_strength.0 || (seen.len() == longest_strength.0 && strength > longest_strength.1) {
                    longest_strength = (seen.len(), strength);
                }

                if strength > highest_strength {
                    highest_strength = strength;
                }

                queue.push_back((nb.other_pin(from_pin), strength, seen));
            }
        }
    }

    (highest_strength, longest_strength.1)
}


// cache results
static SOLUTION: OnceCell<(usize, usize)> = OnceCell::new();

pub fn solve_a() {
    let (ans, _) = *SOLUTION.get_or_init(|| find_strongest_bridge(&read_content(24)));

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let (_, ans) = *SOLUTION.get_or_init(|| find_strongest_bridge(&read_content(24)));
    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use crate::d24::find_strongest_bridge;

    #[test]
    fn test_strength_of_longest_bridge() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        let (a1, a2) = find_strongest_bridge(input);

        assert_eq!(a1, 31);
        assert_eq!(a2, 19);
    }
}