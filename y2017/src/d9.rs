use crate::inputs::read_content;

pub fn solve_a() {
    let stream = read_content(9);

    let ans = score_stream(&stream);
    println!("Solution A: {}", ans);
}

fn score_stream(stream: &str) -> usize {
    let mut ignore_flag = false;
    let mut garbage_flag = false;
    let mut level = 0;
    let mut total_score = 0;

    for c in stream.chars() {
        if ignore_flag {
            ignore_flag = false;
            continue;
        }

        if garbage_flag {
            if c == '!' {
                ignore_flag = true;
            } else if c == '>' {
                garbage_flag = false;
            }
            continue;
        }

        match c {
            '!' => { ignore_flag = true; }
            '{' => {
                level += 1;
            }
            '}' => {
                total_score += level;
                level -= 1
            }
            '<' => {
                garbage_flag = true;
            }
            _ => {}
        }
    }

    total_score
}

pub fn solve_b() {
    let stream = read_content(9);
    let ans = count_garbage(&stream);

    println!("Solution B: {}", ans);
}

fn count_garbage(stream: &str) -> usize {
    let mut ignore_flag = false;
    let mut garbage_flag = false;
    let mut total_score = 0;

    for c in stream.chars() {
        if ignore_flag {
            ignore_flag = false;
            continue;
        }

        if c == '!' {
            ignore_flag = true;
            continue;
        }

        if garbage_flag {
            if c == '>' {
                garbage_flag = false;
            } else {
                total_score += 1;
            }
        } else if c == '<' {
            garbage_flag = true;
        }
    }

    total_score
}

#[cfg(test)]
mod tests {
    use crate::d9::{count_garbage, score_stream};

    #[test]
    fn test_score_stream() {
        for (stream, exp) in [
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ] {
            let ans = score_stream(stream);
            assert_eq!(ans, exp, "{}", stream);
        }
    }

    #[test]
    fn test_count_garbage() {
        for (stream, exp) in [
            ("<>", 0),
            ("<random characters>", 17),
            ("<<<<>", 3),
            ("<{!>}>", 2),
            ("<!!>", 0),
            ("<!!!>>,", 0),
            ("<{o\"i!a,<{i<a>", 10),
        ] {
            let ans = count_garbage(stream);
            assert_eq!(ans, exp, "{}", stream);
        }
    }
}