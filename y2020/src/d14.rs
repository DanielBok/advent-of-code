use std::collections::HashMap;

use itertools::izip;
use regex::Regex;

use crate::inputs::read_contents;

// zero, one, none (X)
enum MaskValue { Z, O, X }

struct Mask(Vec<MaskValue>);

struct Mem {
    id: usize,
    mem: Vec<usize>,
}

enum Command {
    Mask(Mask),
    Mem(Mem),
}

fn parse_commands(input: &str) -> Vec<Command> {
    let mask_re = Regex::new(r"mask = (?<mask>\w+)").unwrap();
    let mem_re = Regex::new(r"mem\[(?<id>\d+)] = (?<value>\d+)").unwrap();

    let mut commands = Vec::new();

    for line in input.lines() {
        let cmd = if let Some(caps) = mask_re.captures(line) {
            let mask = caps.name("mask")
                           .unwrap()
                           .as_str()
                           .chars()
                           .map(|c| {
                               match c {
                                   'X' => MaskValue::X,
                                   '1' => MaskValue::O,
                                   '0' => MaskValue::Z,
                                   _ => { panic!("Invalid char '{}' in line '{}'", c, line) }
                               }
                           })
                           .collect::<Vec<_>>();

            assert_eq!(mask.len(), 36);

            Command::Mask(Mask(mask))
        } else if let Some(caps) = mem_re.captures(line) {
            let id = caps.name("id").unwrap().as_str().parse().unwrap();
            let mem = to_binary(caps.name("value").unwrap().as_str().parse::<usize>().unwrap());
            assert_eq!(mem.len(), 36);

            Command::Mem(Mem { id, mem })
        } else {
            panic!("Invalid line: {}", line);
        };

        commands.push(cmd);
    }

    commands
}

fn to_binary(value: usize) -> Vec<usize> {
    (0..36).rev().map(|i| (value >> i) & 1).collect()
}

pub fn solve_a() {
    let commands = parse_commands(&read_contents(14));
    let ans: usize = memory_store_sum(&commands, apply_mask).values().sum();

    println!("Solution A: {}", ans);
}

fn memory_store_sum<F>(commands: &Vec<Command>, apply_fn: F) -> HashMap<usize, usize>
    where F: Fn(&mut HashMap<usize, usize>, &Mask, &Mem)
{
    let mut mask = match &commands[0] {
        Command::Mask(mask) => mask,
        Command::Mem(_) => panic!("First instruction must be a Mask")
    };

    let mut memory_store = HashMap::new();
    for ins in commands.iter().skip(1) {
        match ins {
            Command::Mask(next_mask) => {
                mask = next_mask;
            }
            Command::Mem(mem) => {
                apply_fn(&mut memory_store, &mask, &mem);
            }
        }
    }

    memory_store
}

fn apply_mask(memory_store: &mut HashMap<usize, usize>, mask: &Mask, mem: &Mem) {
    let vec = izip!(mask.0.iter(), mem.mem.iter())
        .map(|(m, v)| {
            match m {
                MaskValue::Z => 0,
                MaskValue::O => 1,
                MaskValue::X => *v,
            }
        })
        .collect::<Vec<_>>();

    memory_store.insert(mem.id, to_number(&vec));
}

fn to_number(mem: &Vec<usize>) -> usize {
    let mut sum = 0;
    for (n, &flag) in mem.iter().rev().enumerate() {
        if flag == 0 {
            continue;
        } else if flag == 1 {
            sum += 2_usize.pow(n as u32);
        } else {
            panic!("Invalid memory value: {flag}. {mem:?}");
        }
    }
    sum
}

pub fn solve_b() {
    let commands = parse_commands(&read_contents(14));
    let ans: usize = memory_store_sum(&commands, apply_mask_2).values().sum();

    println!("Solution B: {}", ans);
}

fn apply_mask_2(memory_store: &mut HashMap<usize, usize>, mask: &Mask, mem: &Mem) {
    let value = to_number(&mem.mem);

    let ids: Vec<usize> = izip!(mask.0.iter(), to_binary(mem.id))
        .fold(vec![vec![]], |mut acc, (m, v)| {
            match m {
                MaskValue::Z => acc.iter_mut().for_each(|vec| vec.push(v)),
                MaskValue::O => acc.iter_mut().for_each(|vec| vec.push(1)),
                MaskValue::X => {
                    let mut extension = vec![];
                    for vec in acc.iter_mut() {
                        let mut alt = vec.clone();
                        vec.push(0);
                        alt.push(1);
                        extension.push(alt);
                    }

                    acc.extend(extension);
                }
            };

            acc
        })
        .iter()
        .map(to_number)
        .collect();

    for id in ids {
        memory_store.insert(id, value);
    }
}


#[cfg(test)]
mod tests {
    use super::{apply_mask, apply_mask_2, memory_store_sum, parse_commands};

    #[test]
    fn test_memory_store_sum() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let commands = parse_commands(input);
        let ans: usize = memory_store_sum(&commands, apply_mask).values().sum();
        assert_eq!(ans, 165);
        
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let commands = parse_commands(input);
        let ans: usize = memory_store_sum(&commands, apply_mask_2).values().sum();
        assert_eq!(ans, 208);
    }
}