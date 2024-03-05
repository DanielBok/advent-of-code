use std::collections::{HashMap, VecDeque};

use crate::inputs::read_contents;

#[derive(Debug, Clone)]
enum Rule {
    Literal(char),
    Standard(Vec<usize>),
}

fn parse_rule_part(text: &str) -> Rule {
    Rule::Standard(
        text.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>(),
    )
}

fn parse_rules(text: &str) -> Vec<Rule> {
    let mut result = Vec::new();
    if text.contains('"') {
        result.push(Rule::Literal(text.chars().skip(1).next().unwrap()));
    } else {
        for part in text.split(" | ") {
            result.push(parse_rule_part(part));
        }
    }
    result
}

fn get_rules_map(input: &str) -> HashMap<usize, Vec<Rule>> {
    input.lines()
         .map(|line| {
             let (id, rules_text) = line.split_once(": ").unwrap();
             let id = id.trim().parse().unwrap();

             let rules = parse_rules(rules_text);

             (id, rules)
         })
         .collect()
}

fn get_rules_and_messages(input: &str) -> (HashMap<usize, Vec<Rule>>, Vec<String>) {
    let input = input.replace("\r", "");
    let (rules_input, messages_input) = input.split_once("\n\n").unwrap();
    let messages = messages_input.lines().map(|x| x.to_string()).collect();
    let rules = get_rules_map(rules_input);

    (rules, messages)
}

fn matches(rules: &HashMap<usize, Vec<Rule>>, phrase: &[char], mut with: VecDeque<usize>) -> bool {
    match (phrase.len(), with.len()) {
        (0, 0) => return true,  // matches is true only if phrase and with is empty
        (_, 0) => return false, // it can't match if phrase is empty and with is not, likewise for the reverse
        (0, _) => return false,
        _ => {}
    }

    let rule_to_expand = with.pop_front().unwrap();
    let possibilities = &rules[&rule_to_expand];
    for rule in possibilities {
        let result = match rule {
            Rule::Literal(c) => {
                if c == &phrase[0] {
                    // character matched with next rule literal, call matches with one less character and rest of with.
                    let next_with = with.clone();
                    matches(&rules, &phrase[1..], next_with)
                } else {
                    // character didn't match with the expanded rule
                    false
                }
            }
            Rule::Standard(expanded) => {
                // we werent able to match with a character, expand the popped rule and try to match [expanded, rest]
                let next_with: VecDeque<usize> = expanded.iter().chain(with.iter()).copied().collect();
                if expanded.len() > phrase.len() {
                    // if the total expanded rule size is bigger than phrase size, it can't possibly match.
                    // there are no empty rules, each rule will match at least one character
                    false
                } else {
                    matches(&rules, &phrase, next_with)
                }
            }
        };
        if result {
            return true;
        }
    }
    false
}

pub fn solve_a() {
    let ans = solve_without_loops(&read_contents(19));
    println!("Solution A: {}", ans);
}

fn solve_without_loops(input: &str) -> usize {
    let (rules, messages) = get_rules_and_messages(input);
    let strings: Vec<Vec<char>> = messages.iter()
                                          .map(|s| s.chars().collect())
                                          .collect();

    let mut start = VecDeque::new();
    start.push_back(0);

    strings
        .into_iter()
        .map(|s| matches(&rules, &s, start.clone()))
        .filter(|&b| b)
        .count()
}

fn modify_part2(rules: &mut HashMap<usize, Vec<Rule>>) {
    let rule8 = vec![Rule::Standard(vec![42]), Rule::Standard(vec![42, 8])];
    let rule11 = vec![
        Rule::Standard(vec![42, 31]),
        Rule::Standard(vec![42, 11, 31]),
    ];
    if let Some(rule) = rules.get_mut(&8) {
        *rule = rule8;
    }
    if let Some(rule) = rules.get_mut(&11) {
        *rule = rule11;
    }
}

pub fn solve_b() {
    let ans = solve_with_loops(&read_contents(19));

    println!("Solution B: {}", ans);
}

fn solve_with_loops(input: &str) -> usize {
    let (mut rules, messages) = get_rules_and_messages(input);
    modify_part2(&mut rules);

    let strings: Vec<Vec<char>> = messages.iter()
                                          .map(|s| s.chars().collect())
                                          .collect();

    let mut start = VecDeque::new();
    start.push_back(0);

    strings
        .into_iter()
        .map(|s| matches(&rules, &s, start.clone()))
        .filter(|&b| b)
        .count()
}


#[cfg(test)]
mod tests {
    use super::{solve_with_loops, solve_without_loops};

    #[test]
    fn test_parse_input() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

        let ans = solve_without_loops(input);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_parse_input_loops() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        let ans = solve_with_loops(input);
        assert_eq!(ans, 12);
    }
}