use regex::Regex;

const PUZZLE_INPUT: &str = "deal into new stack
cut 9037
deal with increment 49
cut -9932
deal with increment 5
cut 6434
deal with increment 73
cut 1023
deal into new stack
cut 4227
deal with increment 57
cut -6416
deal with increment 48
cut 5020
deal with increment 15
deal into new stack
deal with increment 7
cut -7421
deal with increment 63
cut 6786
deal into new stack
deal with increment 37
cut -6222
deal into new stack
deal with increment 3
cut -4755
deal with increment 31
cut 2694
deal with increment 67
deal into new stack
deal with increment 42
cut 2634
deal into new stack
cut 2358
deal with increment 35
cut 9700
deal with increment 49
cut 264
deal with increment 55
cut 2769
deal with increment 27
cut 593
deal with increment 60
cut -6145
deal into new stack
deal with increment 75
deal into new stack
cut -7065
deal into new stack
cut -2059
deal with increment 30
cut -8773
deal into new stack
deal with increment 60
deal into new stack
deal with increment 22
deal into new stack
cut -2124
deal into new stack
deal with increment 66
cut -6962
deal with increment 31
deal into new stack
deal with increment 48
deal into new stack
deal with increment 62
cut 8716
deal with increment 27
deal into new stack
cut -679
deal into new stack
cut 1069
deal with increment 25
cut 7118
deal into new stack
cut -5787
deal into new stack
cut 9539
deal with increment 11
deal into new stack
deal with increment 49
cut 7631
deal with increment 73
cut -3476
deal into new stack
cut 1401
deal with increment 9
deal into new stack
cut -9773
deal with increment 60
cut 5149
deal with increment 13
cut 5892
deal into new stack
cut 2704
deal with increment 33
cut -3776
deal into new stack
cut -893
deal with increment 11";

#[derive(Debug, PartialEq, Eq)]
enum Technique {
    DealNewStack,
    Cut(i32),
    DealIncrement(usize),
}

fn form_shuffle(input: &str) -> Vec<Technique> {
    let cut_re = Regex::new(r"cut (?<value>-?\d+)").unwrap();
    let inc_re = Regex::new(r"deal with increment (?<value>\d+)").unwrap();

    input.trim().split('\n')
        .map(|line| {
            if line == "deal into new stack" {
                Technique::DealNewStack
            } else if let Some(capture) = cut_re.captures(line) {
                let v = capture.name("value").map(|m| m.as_str().parse::<i32>().unwrap()).unwrap();
                Technique::Cut(v)
            } else if let Some(capture) = inc_re.captures(line) {
                let v = capture.name("value").map(|m| m.as_str().parse::<usize>().unwrap()).unwrap();
                Technique::DealIncrement(v)
            } else {
                panic!("Could not parse line to shuffle technique: {}", line)
            }
        })
        .collect::<Vec<_>>()
}

fn shuffle_deck(shuffle: &Vec<Technique>, num_cards: usize) -> Vec<usize> {
    let mut cards = (0..num_cards).collect::<Vec<_>>();

    for technique in shuffle {
        match technique {
            Technique::DealNewStack => {
                cards.reverse();
            }
            Technique::Cut(n) => {
                if *n == 0 {
                    continue;
                }

                let (front, back) = if n.is_negative() {
                    cards.split_at(cards.len() - (n.abs() as usize))
                } else {
                    cards.split_at(*n as usize)
                };
                cards = [back, front].concat();
            }
            Technique::DealIncrement(v) => {
                let mut next = cards.clone();
                let mut p = 0_usize;

                for i in 0..num_cards {
                    next[p] = cards[i];
                    p = (p + *v) % num_cards
                }

                cards = next;
            }
        }
    }

    cards
}

pub fn solve_a() {
    let shuffle = form_shuffle(PUZZLE_INPUT);

    let cards = shuffle_deck(&shuffle, 10007);
    let ans = cards.iter().enumerate().find(|&(_, v)| *v == 2019).unwrap().0;

    println!("Solution A: {}", ans)
}

pub fn solve_b() {
    // copied solution from https://www.reddit.com/r/adventofcode/comments/ee0rqi/comment/fbnifwk/
    let shuffle = form_shuffle(PUZZLE_INPUT);
    let num_cards: i128 = 119315717514047;
    let repeats: i128 = 101741582076661;

    let x = 2020;
    let y = apply_reversed_shuffle(x, &shuffle, num_cards);
    let z = apply_reversed_shuffle(y, &shuffle, num_cards);

    let a = ((y - z) * modinv(x - y, num_cards)) % num_cards;
    let b = (y - a * x) % num_cards;

    let one = mod_power(a, repeats, num_cards) * x;
    let two = mod_power(a, repeats, num_cards) - 1;
    let three = modinv(a - 1, num_cards);

    let ans = (one + mul_mod(mul_mod(two, three, num_cards), b, num_cards)) % num_cards;

    println!("Solution B: {}", ans);
}

fn apply_reversed_shuffle(x: i128, shuffle: &Vec<Technique>, num_cards: i128) -> i128 {
    let mut x = x;
    for technique in shuffle.iter().rev() {
        match technique {
            Technique::DealNewStack => {
                x = rev_card_deal(x, num_cards);
            }
            Technique::Cut(v) => {
                let v = *v as i128;
                x = rev_card_cut(x, num_cards, v);
            }
            Technique::DealIncrement(v) => {
                let v = *v as i128;
                x = rev_card_increment(x, num_cards, v);
            }
        }
    };
    x
}


fn rev_card_deal(x: i128, num_cards: i128) -> i128 {
    num_cards - x - 1
}

fn rev_card_cut(x: i128, num_cards: i128, cut: i128) -> i128 {
    (x + cut + num_cards) % num_cards
}

fn rev_card_increment(x: i128, num_cards: i128, incr: i128) -> i128 {
    (modinv(incr, num_cards) * x) % num_cards
}

pub fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn modinv(a: i128, m: i128) -> i128 {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        panic!("modinv does not exist")
    } else {
        (x % m + m) % m
    }
}


fn mod_power(mut a: i128, mut b: i128, p: i128) -> i128 {
    let mut res = 1;

    a = a % p;
    if a == 0 {
        return 0;
    }
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % p
        }
        b = b >> 1;
        a = (a * a) % p
    }
    res
}

fn mul_mod(mut a: i128, mut b: i128, m: i128) -> i128 {
    if a >= m {
        a %= m;
    }
    if b >= m {
        b %= m;
    }
    let x = a;
    let c = x * b / m;
    let r = (a * b - c * m) % m;
    if r < 0 {
        r + m
    } else {
        r
    }
}


#[cfg(test)]
mod tests {
    use super::form_shuffle;

    #[test]
    fn test_form_shuffle() {
        use super::Technique;

        let inp = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        let expected = Vec::from([
            Technique::DealNewStack,
            Technique::Cut(-2),
            Technique::DealIncrement(7),
            Technique::Cut(8),
            Technique::Cut(-4),
            Technique::DealIncrement(7),
            Technique::Cut(3),
            Technique::DealIncrement(9),
            Technique::DealIncrement(3),
            Technique::Cut(-1),
        ]);

        let shuffle = form_shuffle(inp);

        assert_eq!(shuffle, expected);
    }

    #[test]
    fn test_shuffle() {
        use super::shuffle_deck;

        for (inp, exp) in [
            ("deal with increment 7
deal into new stack
deal into new stack", "0 3 6 9 2 5 8 1 4 7"),
            ("cut 6
deal with increment 7
deal into new stack", "3 0 7 4 1 8 5 2 9 6"),
            ("deal with increment 7
deal with increment 9
cut -2", "6 3 0 7 4 1 8 5 2 9"),
            ("deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1", "9 2 5 8 1 4 7 0 3 6")
        ] {
            let exp = exp.split(' ').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let shuffle = form_shuffle(inp);
            let cards = shuffle_deck(&shuffle, 10);

            assert_eq!(cards, exp);
        }
    }
}