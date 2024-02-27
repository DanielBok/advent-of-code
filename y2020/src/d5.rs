use std::collections::HashSet;

use crate::inputs::read_contents;

#[derive(Debug)]
struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    fn new(line: &str) -> Self {
        let mut row_range = 0..128;
        let mut col_range = 0..8;

        for c in line.chars() {
            match c {
                'F' => row_range.end = (row_range.end + row_range.start) / 2,
                'B' => row_range.start = (row_range.start + row_range.end) / 2,
                'R' => col_range.start = (col_range.start + col_range.end) / 2,
                'L' => col_range.end = (col_range.start + col_range.end) / 2,
                _ => panic!("Invalid character: '{}'", c)
            }
        }

        BoardingPass { row: row_range.start, column: col_range.start }
    }

    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

pub fn solve_a() {
    let ans = read_contents(5).lines()
        .map(|line| BoardingPass::new(line).id())
        .max()
        .unwrap();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let ids = read_contents(5).lines()
        .map(|line| BoardingPass::new(line).id())
        .collect::<HashSet<_>>();

    let all_possible_ids = ((1 * 8)..=(127 * 8)).collect::<HashSet<usize>>();
    let ans = all_possible_ids
        .difference(&ids)
        .find(|&&v| ids.contains(&(v + 1)) && ids.contains(&(v - 1)))
        .unwrap();

    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::BoardingPass;

    #[test]
    fn test_boarding_pass() {
        for (input, row, col, id) in [
            ("BFFFBBFRRR", 70, 7, 567),
            ("FFFBBBFRRR", 14, 7, 119),
            ("BBFFBBFRLL", 102, 4, 820),
        ] {
            let pass = BoardingPass::new(input);

            assert_eq!(pass.row, row, "Rows do not match. Input='{}'. Pass={:?}", input, pass);
            assert_eq!(pass.column, col, "Columns do not match. Input='{}'. Pass={:?}", input, pass);
            assert_eq!(pass.id(), id, "ID do not match. Input='{}'. Pass={:?}", input, pass);
        }
    }
}