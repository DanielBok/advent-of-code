use std::collections::{HashMap, VecDeque};

use regex::Regex;

const PUZZLE_INPUT: &str = "2 LFPRM, 4 GPNQ => 2 VGZVD
1 KXFHM, 14 SJLP => 8 MGRTM
2 HBXVT, 3 HNHC, 5 BDLV => 1 DKTW
2 MGRTM, 8 RVTB => 4 DFMW
2 SJLP => 9 PXTS
1 NXBG => 6 FXBXZ
32 LPSQ => 9 GSDXD
13 LZGTR => 4 ZRMJ
1 FTPQ, 16 CPCS => 5 HNHC
2 THQH, 2 NDJG, 5 MSKT => 4 LRZV
2 BDLV, 9 HBXVT, 21 NXBG => 7 PLRK
16 LNSKQ, 41 KXFHM, 1 DKTW, 1 NCPSZ, 3 ZCSB, 11 MGRTM, 19 WNJWP, 11 KRBG => 1 FUEL
5 FTPQ, 1 HBXVT => 4 BDLV
15 LSDX, 1 GFJW, 1 QDHJT => 4 NKHQV
9 CZHTP, 1 FRPTK => 6 SNBS
17 LFLVS, 2 WCFT => 8 KGJQ
6 CMHLP => 1 SJLP
144 ORE => 3 KQKXZ
3 GFJW, 1 RVTB, 1 GPNQ => 2 NXBG
4 BDLV => 5 CMHLP
2 LSDX => 1 LZGTR
156 ORE => 3 NDJG
136 ORE => 8 MSKT
4 BDLV, 1 NKHQV, 1 RVTB => 7 LNSKQ
1 LRZV, 3 WCFT => 2 HBXVT
5 KGJQ, 1 SWBSN => 7 QHFX
2 DQHBG => 4 LPSQ
6 GSDXD => 3 LSDX
11 RWLD, 3 BNKVZ, 4 PXTS, 3 XTRQC, 5 LSDX, 5 LMHL, 36 MGRTM => 4 ZCSB
8 CPCS => 2 FRPTK
5 NDJG => 3 WCFT
1 GDQG, 1 QHFX => 4 KXFHM
160 ORE => 3 THQH
20 GFJW, 2 DQHBG => 6 RVTB
2 FXBXZ, 1 WNJWP, 1 VGZVD => 5 RWLD
3 DQHBG => 7 SWBSN
7 QHFX => 8 CPCS
14 HBXVT => 3 VCDW
5 FRPTK => 7 NGDX
1 HWFQ => 4 LFLVS
2 CPCS => 6 ZTKSW
9 KGJQ, 8 ZTKSW, 13 BDLV => 6 GDQG
13 LMHL, 1 LZGTR, 18 BNKVZ, 11 VCDW, 9 DFMW, 11 FTPQ, 3 RWLD => 4 KRBG
1 XRCH => 7 GPNQ
3 WCFT => 9 DQHBG
1 FTPQ => 8 CZHTP
1 PBMR, 2 ZTKSW => 2 BNKVZ
2 PLRK, 3 CPCS => 8 ZSGBG
3 NGDX, 3 XRCH => 6 XTRQC
6 ZTKSW, 11 HNHC, 22 SNBS => 9 WNJWP
5 KQKXZ => 8 HWFQ
23 WCFT => 7 PBMR
1 LRZV, 1 QDHJT => 2 GFJW
1 ZSGBG, 5 CGTHV, 9 ZRMJ => 3 LMHL
1 DQHBG => 9 XRCH
1 GDQG, 17 RWLD, 2 KGJQ, 8 VCDW, 2 BNKVZ, 2 WNJWP, 1 VGZVD => 3 NCPSZ
19 SJLP, 3 ZTKSW, 1 CZHTP => 4 LFPRM
14 SNBS => 8 CGTHV
3 DQHBG, 4 WCFT => 1 FTPQ
3 MSKT, 3 NDJG => 5 QDHJT";

#[derive(Debug)]
struct Formula {
    inputs: HashMap<String, i64>,
    output: String,
    quantity: i64,
}


#[derive(Debug)]
struct FuelCalculator {
    recipes: HashMap<String, Formula>,
}

impl FuelCalculator {
    pub fn new(input: &str) -> FuelCalculator {
        FuelCalculator {
            recipes: FuelCalculator::parse_recipe(input),
        }
    }

    fn parse_recipe(input: &str) -> HashMap<String, Formula> {
        let mut recipes = HashMap::new();

        let re = Regex::new(r"(?<qty>\d+) (?<item>\w+)")
            .expect("Invalid recipe regex");

        let parse_line = |line: &str| -> (String, i64) {
            let caps = re.captures(line).expect(format!("Could not parse line: {line}").as_str());
            let (_, [qty, item]) = caps.extract();

            (item.to_string(), qty.parse::<i64>().unwrap())
        };

        for line in input.split('\n') {
            let (inputs, output) = line.split_once("=>").unwrap();

            let (output, quantity) = parse_line(output);
            let mut formula = Formula {
                output,
                quantity,
                inputs: HashMap::new(),
            };


            for inp in inputs.split(",") {
                let (inp, qty) = parse_line(inp.trim());
                formula.inputs.insert(inp, qty);
            }

            recipes.insert(formula.output.clone(), formula);
        }

        recipes
    }

    pub fn num_ores_required(&self, num_fuel: i64) -> i64 {
        let mut production = HashMap::new();
        production.insert("FUEL".to_string(), num_fuel);

        let mut queue = VecDeque::from([self.recipes.get("FUEL").unwrap()]);
        let mut ore_count = 0;

        while let Some(recipe) = queue.pop_front() {
            let required = match production.get(&recipe.output) {
                Some(&x) if x > 0 => x,
                _ => continue
            };

            let mut batch_size = required / recipe.quantity;
            if (required % recipe.quantity) > 0 {
                batch_size += 1;
            }

            for (next_material, &required_qty) in &recipe.inputs {
                let quantity = batch_size * required_qty;  // next_material required

                production.entry(next_material.clone())
                    .and_modify(|x| *x += quantity)
                    .or_insert(quantity);

                if next_material == "ORE" {
                    ore_count += quantity;
                } else {
                    let next_recipe = self.recipes.get(&next_material.clone()).unwrap();
                    queue.push_back(next_recipe);
                }
            }

            match production.get_mut(&recipe.output) {
                Some(x) => { *x -= batch_size * recipe.quantity }
                None => {}
            };
        }

        ore_count
    }

    pub fn amount_of_fuel(&self, num_ore: i64) -> i64 {
        // total number of ores divided by num_ores_required_per_1_fuel gives the
        // lower bound for the total fuel produced since it does not account for
        // extra inventory
        let mut low = num_ore / self.num_ores_required(1);
        let mut high = 2 * low;

        while low < high {
            let mid = (high + low) / 2;
            if mid == low { break; }

            if self.num_ores_required(mid) > num_ore {
                high = mid;
            } else {
                low = mid
            }
        }
        low
    }
}


pub fn solve_a() {
    let calculator = FuelCalculator::new(PUZZLE_INPUT);

    let ans = calculator.num_ores_required(1);
    assert_eq!(ans, 720484);
    println!("Solution A: {}", ans);
}


pub fn solve_b() {
    let num_ores = 1_000_000_000_000_i64;

    let calculator = FuelCalculator::new(PUZZLE_INPUT);
    let ans = calculator.amount_of_fuel(num_ores);

    assert_eq!(ans, 1993284);
    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use crate::d14::FuelCalculator;

    #[test]
    fn test_fuel_calculator() {
        for (inp, exp) in [
            ("3 A, 3 B => 1 FUEL
5 ORE => 2 A
1 A => 1 B", 15),
            ("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL", 165),
            ("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT", 13312),
            ("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF", 180697),
            ("171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX", 2210736)
        ] {
            let calc = FuelCalculator::new(inp);
            let ans = calc.num_ores_required(1);
            assert_eq!(ans, exp);
        }
    }
}