use std::collections::HashMap;

use indicatif::ProgressIterator;
use itertools::{Itertools, izip, MinMaxResult};

use crate::inputs::read_contents;

type RuleMap = HashMap<(char, char), char>;
type Counter = HashMap<(char, char), usize>;

fn parse_input(input: &str) -> (String, RuleMap) {
    let input = input.replace("\r", "");

    let (top, bottom) = input.split_once("\n\n").unwrap();

    let rules = bottom.lines()
                      .map(|line| {
                          let (left, right) = line.split_once(" -> ").unwrap();
                          assert_eq!(left.len(), 2, "Rule key must be 2 characters long. Got: '{}'", left);
                          assert_eq!(right.len(), 1, "Rule value must be 1 character long. Got '{}'", right);
                          let key: (char, char) = left.chars().collect_tuple().unwrap();
                          let value = right.chars().next().unwrap();
                          (key, value)
                      })
                      .collect();

    (top.to_string(), rules)
}

pub fn solve_a() {
    let (template, rules) = parse_input(&read_contents(14));
    let ans = most_vs_least_difference(&template, &rules, 10);

    println!("Solution A: {}", ans);
}

fn most_vs_least_difference(template: &str, rules: &RuleMap, steps: usize) -> usize {
    let counter = run_insertions(template, rules, steps);

    let last_char = template.chars().last().unwrap();
    let counts: HashMap<char, usize> = counter.iter()
                                              .fold(HashMap::from([(last_char, 1)]), |mut acc, ((k, _), v)| {
                                                  acc.entry(*k)
                                                     .and_modify(|c| { *c += v; })
                                                     .or_insert(*v);
                                                  acc
                                              });

    match counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => { max - min }
        _ => panic!("Could not find MinMax. {:?}", counter)
    }
}

fn run_insertions(template: &str, rules: &RuleMap, steps: usize) -> Counter {
    let mut counter: Counter = HashMap::new();
    for (c1, c2) in izip!(
        template.chars(),
        template.chars().skip(1)
    ) {
        counter.entry((c1, c2))
               .and_modify(|v| { *v += 1; })
               .or_insert(1);
    }

    for _ in (0..steps).progress() {
        counter = run_insertion_once(&counter, rules);
    }

    counter
}

fn run_insertion_once(counter: &Counter, rules: &RuleMap) -> Counter {
    let mut next = HashMap::new();

    counter.iter()
           .for_each(|(k, count)| {
               let c = *rules.get(k).expect(&format!("Could not find rule for {:?}", k));

               next.entry((k.0, c))
                   .and_modify(|v| { *v += count; })
                   .or_insert(*count);

               next.entry((c, k.1))
                   .and_modify(|v| { *v += count; })
                   .or_insert(*count);
           });

    next
}

pub fn solve_b() {
    let (template, rules) = parse_input(&read_contents(14));
    let ans = most_vs_least_difference(&template, &rules, 40);

    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::{most_vs_least_difference, parse_input};

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_most_vs_least_difference() {
        for (steps, exp) in [(10, 1588), (40, 2188189693529)] {
            let (template, rules) = parse_input(TEST_INPUT);
            let diff = most_vs_least_difference(&template, &rules, steps);
            assert_eq!(diff, exp);
        }
    }
}