use itertools::Itertools;
use crate::inputs::read_contents;

pub fn solve_a() {
    let ans: usize = read_contents(10)
        .lines()
        .map(|line| score_syntax_error(line).0)
        .sum();

    println!("Solution A: {}", ans);
}

fn score_syntax_error(line: &str) -> (usize, Option<String>) {
    let mut openings = Vec::new();
    let mut last: Option<char> = None;

    for c in line.chars() {
        match c {
            '{' | '(' | '[' | '<' => {
                if last.is_some() {
                    openings.push(last.unwrap());
                }
                last = Some(c);
            }
            ')' => {
                if last != Some('(') {
                    return (3, None);
                } else {
                    last = openings.pop();
                }
            }
            ']' => {
                if last != Some('[') {
                    return (57, None);
                } else {
                    last = openings.pop();
                }
            }
            '}' => {
                if last != Some('{') {
                    return (1197, None);
                } else {
                    last = openings.pop();
                }
            }
            '>' => {
                if last != Some('<') {
                    return (25137, None);
                } else {
                    last = openings.pop();
                }
            }
            _ => panic!("Invalid character: {}", c)
        }
    }

    if last.is_some() {
        openings.push(last.unwrap());
    }

    (0, Some(openings.iter().collect::<String>()))
}

pub fn solve_b() {
    let scores = read_contents(10)
        .lines()
        .filter_map(|line| score_syntax_error(line).1)
        .into_iter()
        .map(score_valid_chunk)
        .sorted()
        .collect::<Vec<_>>();

    let ans = scores[scores.len() / 2];
    println!("Solution B: {}", ans);
}

fn score_valid_chunk(chunk: String) -> usize {
    chunk.chars()
         .rev()
         .fold(0, |acc, c|
             acc * 5 + match c {
                 '(' => 1,
                 '[' => 2,
                 '{' => 3,
                 '<' => 4,
                 _ => panic!("Invalid element: {}", c)
             })
}

#[cfg(test)]
mod tests {
    use super::score_syntax_error;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_score_syntax_error() {
        let ans: usize = TEST_INPUT
            .lines()
            .map(|line| score_syntax_error(line).0)
            .sum();

        println!("Solution A: {}", ans);
    }
}