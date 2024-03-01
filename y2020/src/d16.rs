use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::{Match, Regex};

use crate::inputs::read_contents;

fn parse_input(input: &str) -> (HashMap<String, HashSet<usize>>, Vec<usize>, Vec<Vec<usize>>) {
    fn match_to_usize(cap: Match) -> usize {
        cap.as_str().parse::<usize>().unwrap()
    }

    let constraint_re = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut state = "constraint";

    let mut constraints = HashMap::new();
    let mut ticket = Vec::new();
    let mut nearby_ticket: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        if line == "your ticket:" {
            state = "your ticket";
            continue;
        } else if line == "nearby tickets:" {
            state = "nearby tickets";
            continue;
        } else if line.is_empty() {
            continue;
        }

        match state {
            "constraint" => {
                let caps = constraint_re.captures(line).expect(&format!("Could not parse constraint: {}", line));
                let class = caps.get(1).unwrap().as_str().to_string();
                let ranges = [
                    match_to_usize(caps.get(2).unwrap())..=match_to_usize(caps.get(3).unwrap()),
                    match_to_usize(caps.get(4).unwrap())..=match_to_usize(caps.get(5).unwrap()),
                ].iter()
                 .flat_map(|range| range.clone().collect::<HashSet<_>>())
                 .collect();

                constraints.insert(class, ranges);
            }
            "your ticket" => {
                ticket = line.split(",").map(|v| v.parse::<usize>().unwrap()).collect();
            }
            "nearby tickets" => {
                nearby_ticket.push(line.split(",").map(|v| v.parse::<usize>().unwrap()).collect());
            }
            _ => panic!("Invalid state: {}", state)
        }
    }


    (constraints, ticket, nearby_ticket)
}

fn get_valid_numbers(constraints: &HashMap<String, HashSet<usize>>) -> HashSet<usize> {
    constraints.values()
               .into_iter()
               .fold(HashSet::new(), |mut acc, other| {
                   for v in other {
                       acc.insert(*v);
                   }
                   acc
               })
}

pub fn solve_a() {
    let (constraints, _, nearby) = parse_input(&read_contents(16));

    let valid_numbers = get_valid_numbers(&constraints);

    let ans: usize = nearby.iter()
                           .map(|ticket| ticket.iter().filter(|v| !valid_numbers.contains(v)).sum::<usize>())
                           .sum();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let (constraints, my_ticket, nearby) = parse_input(&read_contents(16));
    let valid_numbers = get_valid_numbers(&constraints);

    let valid_nearby = nearby.into_iter()
                             .filter(|ticket| ticket.iter().all(|v| valid_numbers.contains(v)))
                             .collect_vec();

    let mut fixed: HashMap<String, usize> = HashMap::new();
    let mut unfixed: HashMap<usize, HashSet<String>> = HashMap::new();

    for i in 0..my_ticket.len() {
        let nums = valid_nearby.iter().map(|ticket| ticket[i]).collect_vec();

        let mut possible_fields = Vec::new();
        for (field, valid) in constraints.iter() {
            if nums.iter().all(|v| valid.contains(v)) {
                possible_fields.push(field.clone());
            }
        }

        if possible_fields.len() == 1 {
            fixed.insert(possible_fields.pop().unwrap(), i);
        } else {
            unfixed.insert(i, possible_fields.into_iter().collect::<HashSet<_>>());
        }
    }

    // to prevent infinite looping
    let mut count = 0;
    while fixed.len() != my_ticket.len() && count < 100 {
        let confirmed = fixed.keys().cloned().collect::<HashSet<_>>();

        for i in 0..my_ticket.len() {
            if let Some(mut set) = unfixed.remove(&i) {
                for x in confirmed.iter() {
                    set.remove(x);
                }

                if set.len() == 1 {
                    let v = set.iter().collect_vec().pop().unwrap().clone();
                    fixed.insert(v, i);
                } else {
                    unfixed.insert(i, set);
                }
            }
        }

        count += 1;
    }


    assert_eq!(fixed.len(), my_ticket.len());

    let mut ans = 1;
    for (key, index) in fixed.iter() {
        if key.starts_with("departure") {
            ans *= my_ticket.get(*index).unwrap();
        }
    }

    println!("Solution B: {}", ans);
}