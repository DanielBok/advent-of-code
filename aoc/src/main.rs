use std::env;
use std::process;

use config::Config;
use y2019;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Could not run solver because:\n\t{}", err);
        process::exit(1);
    });

    match conf.year {
        2019 => {
            y2019::solve(conf.day)
        }
        _ => {
            panic!("Invalid AOC year {}", conf.year)
        }
    }
}
