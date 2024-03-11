use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::inputs::read_contents;

fn parse_input(input: &str) -> [VecDeque<usize>; 2] {
    let mut v1 = VecDeque::new();
    let mut v2 = VecDeque::new();

    let mut player = 0;
    input.lines().for_each(|line| {
        let line = line.trim();
        if line.is_empty() {
            return;
        } else if line.starts_with("Player") {
            player += 1;
        } else {
            let value = line.parse().unwrap();

            if player == 1 {
                v1.push_back(value);
            } else if player == 2 {
                v2.push_back(value);
            } else {
                panic!("Too many players!")
            }
        }
    });

    [v1, v2]
}

pub fn solve_a() {
    let [mut player1, mut player2] = parse_input(&read_contents(22));

    let player = simulate(&mut player1, &mut player2);
    let score = count_score(player);
    println!("Solution A: {}", score);
}

fn simulate<'a>(player1: &'a mut VecDeque<usize>, player2: &'a mut VecDeque<usize>) -> &'a VecDeque<usize> {
    while !player1.is_empty() && !player2.is_empty() {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        assert_ne!(c1, c2, "Cards cannot have equal values: {}", c1);

        if c1 > c2 {
            player1.extend([c1, c2]);
        } else {
            player2.extend([c2, c1]);
        }
    }

    if player1.is_empty() { player2 } else { player1 }
}

fn count_score(player: &VecDeque<usize>) -> usize {
    let n = player.len();

    player.iter().enumerate().map(|(i, v)| (n - i) * v).sum()
}

pub fn solve_b() {
    let [player1, player2] = parse_input(&read_contents(22));
    let (_, winning_deck) = recursive_simulate(player1, player2);
    // let (_, winning_deck) = recursive_combat(player1, player2);

    let score = count_score(&winning_deck);
    println!("Solution B: {}", score);
}

enum Winner {
    Player1,
    Player2,
}

fn recursive_simulate(player1: VecDeque<usize>, player2: VecDeque<usize>) -> (Winner, VecDeque<usize>) {
    let mut player1 = player1;
    let mut player2 = player2;

    let mut seen = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        let hash = hash_decks(&player1, &player2);
        if !seen.insert(hash) {  // insert returns false if hash already exists
            return (Winner::Player1, player1);
        }

        if c1 <= player1.len() && c2 <= player2.len() {
            let (winner, _) = recursive_simulate(
                player1.range(..c1).copied().collect(),
                player2.range(..c2).copied().collect(),
            );
            match winner {
                Winner::Player1 => player1.extend([c1, c2]),
                Winner::Player2 => player2.extend([c2, c1]),
            }
        } else if c1 > c2 {
            player1.extend([c1, c2]);
        } else if c1 < c2 {
            player2.extend([c2, c1]);
        } else {
            panic!("This should not occur!")
        }
    }

    if player1.is_empty() { (Winner::Player2, player2) } else { (Winner::Player1, player1) }
}

fn hash_decks(deck1: &VecDeque<usize>, deck2: &VecDeque<usize>) -> String {
    [
        deck1.iter().map(|v| v.to_string()).join(","),
        deck2.iter().map(|v| v.to_string()).join(",")
    ].join(";")
}


#[cfg(test)]
mod tests {
    use crate::d22::{count_score, parse_input, recursive_simulate, simulate};

    fn test_input<'a>() -> &'a str {
        "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
    }

    #[test]
    fn test_simulate() {
        let [mut player1, mut player2] = parse_input(test_input());

        let player = simulate(&mut player1, &mut player2);
        let score = count_score(player);
        assert_eq!(score, 306);
    }

    #[test]
    fn test_recursive_simulate() {
        let [player1, player2] = parse_input(test_input());
        let (_, winning_deck) = recursive_simulate(player1, player2);

        let score = count_score(&winning_deck);
        assert_eq!(score, 291);
    }
}