use crate::inputs::read_contents;

fn parse_input(input: &str) -> Vec<usize> {
    input.lines()
         .map(|line| line.parse().unwrap())
         .collect()
}

pub fn solve_a() {
    let measurements = parse_input(&read_contents(1));
    let ans = count_depth_increases(&measurements);

    println!("Solution A: {}", ans);
}

fn count_depth_increases(measurements: &Vec<usize>) -> usize {
    measurements[1..].into_iter()
                     .fold((0, measurements[0]), |(count, prev), &depth| {
                         if depth > prev {
                             (count + 1, depth)
                         } else {
                             (count, depth)
                         }
                     })
                     .0
}

pub fn solve_b() {
    let measurements = parse_input(&read_contents(1));
    let ans = rolling_sum(&measurements, 3);
    println!("Solution B: {}", ans);
}

fn rolling_sum(measurements: &Vec<usize>, window: usize) -> usize {
    measurements.windows(window)
                .map(|v| v.iter().sum::<usize>())
                .fold((0, usize::MAX), |(count, prev), depth| {
                    if depth > prev {
                        (count + 1, depth)
                    } else {
                        (count, depth)
                    }
                })
                .0
}
