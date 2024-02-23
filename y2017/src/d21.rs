use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use itertools::{Itertools, izip};

use crate::inputs::read_content;

#[derive(Clone)]
struct Fractal {
    v: Vec<Vec<char>>,
    s: String,
    size: usize,
}

impl PartialEq for Fractal {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Fractal {}

impl Hash for Fractal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl Debug for Fractal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fractal")
            .field("size", &self.size)
            .field("s", &self.s.clone())
            .finish()
    }
}

impl Fractal {
    fn as_str(&self) -> &str {
        self.s.as_str()
    }

    fn from_str(input: &str) -> Self {
        let v: Vec<Vec<char>> = input.split('/')
            .map(|line| line.chars().collect())
            .collect();

        let size = v.len();
        Fractal { v, s: input.to_string(), size }
    }

    fn from_vec(v: &Vec<Vec<char>>) -> Self {
        let s = v.iter().map(|sv| sv.iter().collect::<String>()).join("/");
        Fractal { v: v.clone(), s, size: v.len() }
    }

    fn permutations(&self) -> HashSet<Self> {
        let mut perms = HashSet::new();

        match self.size {
            2 | 3 => {
                let mut nv = self.v.clone();
                let mut fv = Self::flip(&self.v);

                perms.insert(Fractal::from_vec(&nv));
                perms.insert(Fractal::from_vec(&fv));

                for _ in 0..3 {
                    nv = Self::rotate(&nv);
                    fv = Self::rotate(&fv);

                    perms.insert(Fractal::from_vec(&nv));
                    perms.insert(Fractal::from_vec(&fv));
                }
            }
            _ => { panic!("Size should either be 2 or 3") }
        };

        perms
    }

    fn rotate(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut nv = v.clone();

        match v.len() {
            2 => {
                nv[0][0] = v[1][0];
                nv[0][1] = v[0][0];
                nv[1][0] = v[1][1];
                nv[1][1] = v[0][1];
            }
            3 => {
                nv[0][0] = v[2][0];
                nv[0][1] = v[1][0];
                nv[0][2] = v[0][0];
                nv[1][0] = v[2][1];
                nv[1][2] = v[0][1];
                nv[2][0] = v[2][2];
                nv[2][1] = v[1][2];
                nv[2][2] = v[0][2];
            }
            _ => { panic!("Size should either be 2 or 3") }
        }
        nv
    }

    fn flip(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut nv = v.clone();

        match v.len() {
            2 => {
                nv[0][0] = v[0][1];
                nv[0][1] = v[0][0];
                nv[1][0] = v[1][1];
                nv[1][1] = v[1][0];
            }
            3 => {
                nv[0][0] = v[0][2];
                nv[0][2] = v[0][0];
                nv[1][0] = v[1][2];
                nv[1][2] = v[1][0];
                nv[2][0] = v[2][2];
                nv[2][2] = v[2][0];
            }
            _ => { panic!("Size should either be 2 or 3") }
        }
        nv
    }
}

struct Grid {
    layout: Vec<Vec<char>>,
    size: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.layout.iter().map(|v| v.iter().collect::<String>()).join("\n"))
    }
}

impl Grid {
    fn new() -> Self {
        let seed = ".#./..#/###";
        let layout = seed.split('/').map(|x| x.chars().collect_vec()).collect_vec();
        let size = layout.len();
        Grid { layout, size }
    }

    fn fractals(&self) -> Vec<Vec<Fractal>> {
        let mut fractals = vec![];
        if self.size % 2 == 0 {
            for (r1, r2) in izip!(
                self.layout.iter().step_by(2),
                self.layout.iter().skip(1).step_by(2),
            ) {
                let mut row_frac = vec![];

                for i in (0..r1.len()).step_by(2) {
                    let v = vec![
                        vec![r1[i], r1[i + 1]],
                        vec![r2[i], r2[i + 1]],
                    ];
                    row_frac.push(Fractal::from_vec(&v));
                }

                fractals.push(row_frac);
            }
        } else if self.size % 3 == 0 {
            for (r1, r2, r3) in izip!(
                self.layout.iter().step_by(3),
                self.layout.iter().skip(1).step_by(3),
                self.layout.iter().skip(2).step_by(3)
            ) {
                let mut row_frac = vec![];

                for i in (0..r1.len()).step_by(3) {
                    let v = vec![
                        vec![r1[i], r1[i + 1], r1[i + 2]],
                        vec![r2[i], r2[i + 1], r2[i + 2]],
                        vec![r3[i], r3[i + 1], r3[i + 2]],
                    ];
                    row_frac.push(Fractal::from_vec(&v));
                }

                fractals.push(row_frac);
            }
        } else {
            panic!("Size must be divisible by 2 or 3. Got {}", self.size)
        }
        fractals
    }

    fn evolve(&self, rulebook: &Rulebook) -> Grid {
        let next_size = if self.size % 2 == 0 {
            self.size / 2 * 3
        } else if self.size % 3 == 0 {
            self.size / 3 * 4
        } else {
            panic!("Size must be divisible by 2 or 3. Got {}", self.size)
        };

        let mut layout: Vec<Vec<char>> = (0..next_size).map(|_| Vec::with_capacity(next_size)).collect();

        for (rn, row) in self.fractals().iter().enumerate() {
            for f in row {
                let next_f = rulebook.next_fractal(f);

                for (srn, char_row) in next_f.v.iter().enumerate() {
                    layout[rn * next_f.size + srn].extend(char_row);
                }
            }
        }

        Grid { layout, size: next_size }
    }

    fn num_pixels(&self) -> usize {
        self.layout.iter().map(|r| r.iter().filter(|c| **c == '#').count()).sum()
    }
}

struct Rulebook {
    m: HashMap<Fractal, Fractal>,
}

impl Rulebook {
    fn new(input: &str) -> Self {
        let mut rulebook = HashMap::new();

        for line in input.lines() {
            let (base, next) = line.split_once(" => ").unwrap();
            let next = Fractal::from_str(next);

            let base_fractal = Fractal::from_str(base);

            for f in base_fractal.permutations() {
                rulebook.insert(f, next.clone());
            }
        }

        Rulebook { m: rulebook }
    }

    fn next_fractal(&self, f: &Fractal) -> Fractal {
        self.m.get(f).unwrap().clone()
    }
}


pub fn solve_a() {
    let rulebook = Rulebook::new(&read_content(21));

    let mut grid = Grid::new();
    
    for _ in 0..5 {
        grid = grid.evolve(&rulebook);
    }
    
    let ans = grid.num_pixels();
    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let rulebook = Rulebook::new(&read_content(21));

    let mut grid = Grid::new();

    for _ in 0..18 {
        grid = grid.evolve(&rulebook);
        println!("{}", grid.num_pixels());
    }

    let ans = grid.num_pixels();
    println!("Solution A: {}", ans);
}

#[cfg(test)]
mod tests {
    use crate::d21::{Fractal, Grid, Rulebook};

    #[test]
    fn test_permutations() {
        for (inp, exp) in [
            ("../..", 1),
            ("#./..", 4),
            ("###/#.#/...", 4),
            ("##./#../.#.", 8),
            ("##./.#./...", 8),
            ("##./.#./.#.", 8),
        ] {
            let fractal = Fractal::from_str(inp);
            let perms = fractal.permutations();
            assert_eq!(perms.len(), exp);
        }
    }

    #[test]
    fn test_grid() {
        let rulebook = Rulebook::new("../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#");

        let mut grid = Grid::new();
        assert_eq!(grid.size, 3);
        println!("{}\n\n", grid);

        for exp in [4, 12] {
            grid = grid.evolve(&rulebook);
            println!("{}\n\n", grid);
            assert_eq!(grid.num_pixels(), exp);
        }
    }
}