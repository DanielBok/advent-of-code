use crate::inputs::read_contents;

#[derive(PartialEq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => unreachable!()
        }
    }
}

trait Score {
    fn played(&self) -> &Move;
    fn opponent(&self) -> &Move;

    fn score(&self) -> usize {
        let move_score = match &self.played() {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let match_score = match (&self.played(), &self.opponent()) {
            (Move::Rock, Move::Scissors) |
            (Move::Scissors, Move::Paper) |
            (Move::Paper, Move::Rock) => 6,
            (_, _) if self.played() == self.opponent() => 3,
            (_, _) => 0
        };

        match_score + move_score
    }
}

struct RoundA {
    opponent: Move,
    played: Move,
}

impl From<&str> for RoundA {
    fn from(value: &str) -> Self {
        let mut chars = value.trim().chars();
        Self {
            opponent: Move::from(chars.next().unwrap()),
            played: Move::from(chars.next_back().unwrap()),
        }
    }
}

impl Score for RoundA {
    fn played(&self) -> &Move { &self.played }
    fn opponent(&self) -> &Move { &self.opponent }
}


fn create_rounds<'a, T: From<&'a str>>(input: &'a str) -> Vec<T> {
    input.lines()
         .map(T::from)
         .collect()
}

pub fn solve_a() {
    let ans: usize = create_rounds::<RoundA>(&read_contents(2))
        .iter()
        .map(|r| r.score())
        .sum();

    println!("Solution A: {}", ans);
}

struct RoundB {
    opponent: Move,
    played: Move,
}

impl From<&str> for RoundB {
    fn from(value: &str) -> Self {
        let mut chars = value.trim().chars();
        let opponent = Move::from(chars.next().unwrap());

        let played = match chars.next_back().unwrap() {
            'X' => match &opponent {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            'Y' => opponent.clone(),
            'Z' => match &opponent {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            _ => unreachable!()
        };

        Self { opponent, played }
    }
}

impl Score for RoundB {
    fn played(&self) -> &Move { &self.played }
    fn opponent(&self) -> &Move { &self.opponent }
}

pub fn solve_b() {
    let ans: usize = create_rounds::<RoundB>(&read_contents(2))
        .iter()
        .map(|r| r.score())
        .sum();

    println!("Solution A: {}", ans);
}