use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use regex::Regex;

use crate::inputs::read_contents;

#[derive(Eq)]
struct BagRule {
    source: String,
    children: HashMap<String, usize>,
    keys: HashSet<String>,
}

impl PartialEq for BagRule {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Hash for BagRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state)
    }
}


fn get_bag_rules(input: &str) -> Vec<BagRule> {
    let name_re = Regex::new(r"(?<bag_name>.+) bags?").unwrap();
    let re = Regex::new(r"(?<num>\d+) (?<bag_name>.+) bags?").unwrap();

    input.lines().map(|line| {
        let (source, children_info) = line.split_once(" contain ").unwrap();

        let source = name_re.captures(source).unwrap().name("bag_name").unwrap().as_str().to_owned();

        let children = if children_info.trim() == "no other bags." {
            HashMap::new()
        } else {
            children_info.trim_end_matches(".").split(",")
                .map(|line| {
                    let caps = re.captures(line).expect(&format!("could not parse: '{}'", line));
                    let qty = caps.name("num").unwrap().as_str().parse().unwrap();
                    let name = caps.name("bag_name").unwrap().as_str().to_owned();
                    (name, qty)
                })
                .collect()
        };

        let keys = children.keys().cloned().collect();

        BagRule { source: source.to_string(), children, keys }
    })
        .collect()
}

pub fn solve_a() {
    let rules = get_bag_rules(&read_contents(7));
    let ans = find_gold_bag_source(&rules);

    println!("Solution A: {}", ans);
}

fn find_gold_bag_source(rules: &Vec<BagRule>) -> usize {
    let mut seen = HashSet::new();
    let mut holds_gold_bag = HashSet::from(["shiny gold".to_owned()]);

    loop {
        let n = seen.len();
        for rule in rules {
            if !seen.contains(&rule) && rule.keys.intersection(&holds_gold_bag).count() > 0 {
                seen.insert(rule);
                holds_gold_bag.insert(rule.source.clone());
            }
        }

        if n == seen.len() {
            break;
        }
    }

    holds_gold_bag.len() - 1
}

pub fn solve_b() {
    let rules = get_bag_rules_map(&read_contents(7));
    let ans = count_bags("shiny gold", &rules);
    println!("Solution B: {}", ans);
}


fn get_bag_rules_map(input: &str) -> HashMap<String, BagRule> {
    get_bag_rules(input)
        .into_iter()
        .map(|b| (b.source.clone(), b))
        .collect()
}


fn count_bags(source: &str, rules: &HashMap<String, BagRule>) -> usize {
    let mut total = 0;
    let mut queue = VecDeque::from([(source, 1)]);

    while let Some((bag, qty)) = queue.pop_front() {
        let rule = rules.get(bag)
            .expect(&format!("could not find bag '{}'", bag));

        if bag != source {
            // println!("Adding {qty} {bag}. {total} -> {}", total + qty);
            total += qty;
        }
        if !rule.children.is_empty() {
            for (child_bag, &child_qty) in rule.children.iter() {
                queue.push_back((child_bag.deref(), child_qty * qty))
            }
        }
    }


    total
}


#[cfg(test)]
mod tests {
    use super::{count_bags, find_gold_bag_source, get_bag_rules, get_bag_rules_map};

    #[test]
    fn test_find_gold_bag_source() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let rules = get_bag_rules(input);
        assert_eq!(find_gold_bag_source(&rules), 4);
    }

    #[test]
    fn test_count_bags() {
        for (input, exp) in [
            ("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.", 32),
            ("shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.", 126)
        ] {
            let rules = get_bag_rules_map(input);
            assert_eq!(count_bags("shiny gold", &rules), exp);
        }
    }
}