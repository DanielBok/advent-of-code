use std::collections::HashMap;

use itertools::izip;

use crate::inputs::read_contents;

fn form_inputs(input: &str) -> (usize, HashMap<usize, usize>) {
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse().unwrap();

    // the key is the bus ID and the value is it's offset
    let buses = lines.next().unwrap()
                     .split(",")
                     .enumerate()
                     .filter_map(|(pos, id)| {
                         match id.parse::<usize>() {
                             Ok(v) => { Some((v, pos)) }
                             Err(_) => { None }
                         }
                     })
                     .collect();
    (start, buses)
}

pub fn solve_a() {
    let (start_time, buses) = form_inputs(&read_contents(13));
    let (bus_id, wait_time) = first_bus_id_and_waiting_time(start_time, &buses);

    println!("Solution A: {}", bus_id * wait_time);
}

fn first_bus_id_and_waiting_time(start_time: usize, buses: &HashMap<usize, usize>) -> (usize, usize) {
    let mut best_bus = 0;
    let mut shortest_waiting_time = usize::MAX;

    for &bus_id in buses.keys() {
        if start_time % bus_id == 0 {
            return (bus_id, 0);
        }

        let bus_time = (start_time / bus_id + 1) * bus_id;
        let wait_time = bus_time - start_time;

        if wait_time < shortest_waiting_time {
            shortest_waiting_time = wait_time;
            best_bus = bus_id;
        }
    }

    (best_bus, shortest_waiting_time)
}

pub fn solve_b() {
    let (_, buses) = form_inputs(&read_contents(13));
    let ans = chinese_remainder(&buses);

    assert_eq!(ans, 294354277694107);
    println!("Solution B: {}", ans);
}


/**
Chinese remainder theorem implementation from
https://homepages.math.uic.edu/~leon/mcs425-s08/handouts/chinese_remainder.pdf

Also look at https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/modular-inverses
for how to calculate modulo inverses
 **/
fn chinese_remainder(buses: &HashMap<usize, usize>) -> usize {
    let (modulo, a) = buses.into_iter()
                           .fold((Vec::new(), Vec::new()),
                                 |(mut modulo, mut remainder), (bus_id, offset)| {
                                     modulo.push(*bus_id);
                                     remainder.push(if *offset == 0 { 0 } else { bus_id - (offset % bus_id) });
                                     (modulo, remainder)
                                 });
    let m = modulo.iter().product::<usize>();
    let z = modulo.iter().map(|v| m / v).collect::<Vec<_>>();
    let y = z.iter()
             .zip(modulo.iter())
             .map(|(zi, mi)| {
                 // find modulo inverse
                 (1..*mi).find(|vi| (vi * zi) % mi == 1).unwrap()
             })
             .collect::<Vec<_>>();

    let x = izip!(y.iter(), z.iter(), a.iter())
        .map(|(yi, zi, ai)| yi * zi * ai)
        .sum::<usize>() % m;

    x
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{chinese_remainder, first_bus_id_and_waiting_time, form_inputs};

    #[test]
    fn test_first_bus_id_and_waiting_time() {
        let (start_time, buses) = form_inputs("939
7,13,x,x,59,x,31,19");

        let (bus_id, wait_time) = first_bus_id_and_waiting_time(start_time, &buses);

        assert_eq!(bus_id, 59);
        assert_eq!(wait_time, 5);
    }

    fn create_bus_id_offset_map(input: &str) -> HashMap<usize, usize> {
        input.split(",")
             .enumerate()
             .filter_map(|(pos, id)| {
                 match id.parse::<usize>() {
                     Ok(v) => { Some((v, pos)) }
                     Err(_) => { None }
                 }
             })
             .collect()
    }

    #[test]
    fn test_chinese_remainder() {
        for (input, exp) in [
            ("7,13,x,x,59,x,31,19", 1068781),
            ("17,x,13,19", 3417),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ] {
            let buses = create_bus_id_offset_map(input);
            let ans = chinese_remainder(&buses);
            assert_eq!(ans, exp);
        }
    }
}