use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

use crate::inputs::read_contents;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point(i32, i32);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0)
                 .then(self.1.cmp(&other.1)))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_map(input: &str) -> HashMap<Point, usize> {
    let mut map = HashMap::new();

    input.lines()
         .enumerate()
         .for_each(|(row_no, line)| {
             line.chars()
                 .enumerate()
                 .for_each(|(col_no, risk)| {
                     let risk = risk.to_digit(10).unwrap() as usize;
                     map.insert(Point(col_no as i32, row_no as i32), risk);
                 });
         });

    map
}

fn get_neighbours(map: &HashMap<Point, usize>, point: &Point) -> Vec<(Point, usize)> {
    [
        Point(point.0 + 1, point.1),
        Point(point.0 - 1, point.1),
        Point(point.0, point.1 + 1),
        Point(point.0, point.1 - 1),
    ].into_iter()
     .filter(|p| map.contains_key(p))
     .map(|p| {
         let risk = *map.get(&p).unwrap();
         (p, risk)
     })
     .collect()
}


fn find_risk_of_best_path(map: &HashMap<Point, usize>) -> usize {
    let mut queue = BTreeSet::from([Point(0, 0)]);
    let mut risk_scores: HashMap<Point, usize> = HashMap::from([(Point(0, 0), 0)]);

    let endpoint = map.keys()
                      .sorted_by(|p1, p2| p2.cmp(p1))
                      .next()
                      .unwrap()
                      .clone();

    let mut lowest = usize::MAX;
    while let Some(curr) = queue.pop_first() {
        let curr_risk = *risk_scores.get(&curr).unwrap();

        if curr == endpoint {
            if curr_risk < lowest {
                lowest = curr_risk;
            }
            continue;
        }

        for (next_pt, nb_risk) in get_neighbours(&map, &curr) {
            let risk = curr_risk + nb_risk;
            if !risk_scores.contains_key(&next_pt) || *risk_scores.get(&next_pt).unwrap() > risk {
                risk_scores.insert(next_pt.clone(), risk);
                queue.insert(next_pt);
            }
        }
    }

    lowest
}


pub fn solve_a() {
    let map = parse_map(&read_contents(15));
    let ans = find_risk_of_best_path(&map);

    println!("Solution A: {}", ans);
}

fn parse_map2(input: &str) -> HashMap<Point, usize> {
    let mut map = HashMap::new();
    let n = input.lines().count() as i32;

    for (row_no, line) in input.lines().enumerate() {
        for ry in 0..5 {
            let rn = row_no as i32 + ry * n;

            for (col_no, risk) in line.chars().enumerate() {
                let base_risk = risk.to_digit(10).unwrap() as i32 + ry;

                for rx in 0..5 {
                    let cn = col_no as i32 + rx * n;
                    let mut risk = base_risk + rx;
                    if risk > 9 { risk -= 9; }

                    map.insert(Point(cn, rn), risk as usize);
                }
            }
        }
    }

    map
}

pub fn solve_b() {
    let map = parse_map2(&read_contents(15));
    let ans = find_risk_of_best_path(&map);

    println!("Solution B: {}", ans);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{parse_map2, Point};

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_parse_map2() {
        let map = parse_map2(TEST_INPUT);

        let n = 5 * TEST_INPUT.lines().count() as i32;

        let full_map = (0..n).map(|y| {
            (0..n)
                .map(|x| {
                    map.get(&Point(x, y))
                       .expect(&format!("No values at ({}, {})", x, y))
                       .to_string()
                })
                .collect::<String>()
        }).join("\n");

        assert_eq!(full_map, "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479");
    }
}