use std::env;
use std::process;

use config::Config;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Could not run solver because:\n\t{}", err);
        process::exit(1);
    });

    match conf.year {
        2017 => y2017::solve(conf.day),
        2019 => y2019::solve(conf.day),
        2020 => y2020::solve(conf.day),
        2021 => y2021::solve(conf.day),
        2022 => y2022::solve(conf.day),
        _ => {
            panic!("Invalid AOC year {}", conf.year);
        }
    }
}
