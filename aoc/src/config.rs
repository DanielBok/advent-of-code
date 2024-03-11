#[derive(Debug)]
pub struct Config {
    pub year: usize,
    pub day: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments"));
        }

        let year = match args[1].parse::<usize>() {
            Ok(x) => {
                let valid_years = [2017, 2019, 2020, 2021];
                if valid_years.contains(&x) {
                    Ok(x)
                } else {
                    Err(String::from("year is not valid"))
                }
            }
            Err(e) => Err(format!("Could not parse year: {}", e))
        }?;

        let day = match args[2].parse::<usize>() {
            Ok(x) => {
                if x <= 25 {
                    Ok(x)
                } else {
                    Err("day must be between 0 and 25 [inclusive]. 0 runs everything.".to_string())
                }
            }
            Err(e) => Err(format!("Could not parse day: {}", e))
        }?;

        Ok(Config { year, day })
    }
}
