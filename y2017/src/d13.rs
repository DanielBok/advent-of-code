use std::collections::HashMap;

use crate::inputs::read_content;

#[derive(Debug)]
enum ScannerDirection {
    Up,
    Down,
}

struct Firewall {
    layers: HashMap<usize, usize>,
}

impl Firewall {
    fn new(input: &str) -> Firewall {
        let layers = Firewall::parse_layer(input);

        Firewall { layers }
    }

    fn parse_layer(input: &str) -> HashMap<usize, usize> {
        input.lines()
            .map(|line| {
                let (layer, depth) = line.split_once(':').unwrap();
                let layer: usize = layer.trim().parse().unwrap();
                let depth: usize = depth.trim().parse().unwrap();

                (layer, depth)
            })
            .collect()
    }

    fn run_packet(&self) -> usize {
        let mut score = 0;
        let last_layer = *self.layers.keys().max().unwrap();

        let mut scanners: HashMap<usize, (usize, ScannerDirection)> = self.layers.keys()
            .map(|k| (*k, (0, ScannerDirection::Down)))
            .collect();

        for layer in 0..=last_layer {
            if let Some((scanner_pos, _)) = scanners.get(&layer) {
                if *scanner_pos == 0 {
                    score += self.layers[&layer] * layer
                }
            }

            for next_layer in (layer + 1)..=last_layer {
                if let Some(depth) = self.layers.get(&next_layer) {
                    scanners.entry(next_layer)
                        .and_modify(|(pos, dir)| {
                            match dir {
                                ScannerDirection::Up => { *pos -= 1; }
                                ScannerDirection::Down => { *pos += 1; }
                            }

                            if *pos == 0 {
                                *dir = ScannerDirection::Down;
                            } else if *pos + 1 == *depth {
                                *dir = ScannerDirection::Up;
                            }
                        })
                    ;
                }
            }
        }

        score
    }

    /// Finds the earliest required delay so that the packet can move through
    /// the network without being caught
    fn find_delay(&self) -> usize {
        'delay_loop: for delay in 0..usize::MAX {
            for (pos, height) in self.layers.iter() {
                // the current position of the scanner at that layer is given by
                // the following formula
                let scanner_position = (pos + delay) % (2 * (height - 1));

                // if scanner position == 0, packet can't go through given that delay
                if scanner_position == 0 {
                    continue 'delay_loop;
                }
            }
            return delay;
        }

        panic!("Solution not found")
    }
}


pub fn solve_a() {
    let firewall = Firewall::new(&read_content(13));
    let ans = firewall.run_packet();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let firewall = Firewall::new(&read_content(13));
    let ans = firewall.find_delay();

    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use crate::d13::Firewall;

    #[test]
    fn test_firewall_run_packet() {
        let firewall = Firewall::new("0: 3
1: 2
4: 4
6: 4");
        let ans = firewall.run_packet();
        assert_eq!(ans, 24);
    }
}