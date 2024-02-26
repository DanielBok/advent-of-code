use once_cell::sync::OnceCell;
use regex::Regex;

use crate::inputs::read_contents;

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

static RE: OnceCell<Regex> = OnceCell::new();
static HGT_RE: OnceCell<Regex> = OnceCell::new();
static HCL_RE: OnceCell<Regex> = OnceCell::new();

impl Passport {
    fn from_lines(lines: &str) -> Passport {
        let re = RE.get_or_init(|| Regex::new(r"\s+").unwrap());

        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        for item in re.split(lines.trim()).collect::<Vec<_>>() {
            let (key, value) = item.split_once(':').expect(&format!("Could not parse item '{}' for input='{}'", item, lines));

            match key {
                "byr" => { passport.byr = Some(value.parse().expect(&format!("Could not parse value '{}' to digit", value))); }
                "iyr" => { passport.iyr = Some(value.parse().expect(&format!("Could not parse value '{}' to digit", value))); }
                "eyr" => { passport.eyr = Some(value.parse().expect(&format!("Could not parse value '{}' to digit", value))); }
                "hgt" => { passport.hgt = Some(value.to_string()); }
                "hcl" => { passport.hcl = Some(value.to_string()); }
                "ecl" => { passport.ecl = Some(value.to_string()); }
                "pid" => { passport.pid = Some(value.to_string()); }
                "cid" => { passport.cid = Some(value.parse().expect(&format!("Could not parse value '{}' to digit", value))); }
                _ => panic!("Invalid field: {}", key)
            }
        }

        passport
    }

    fn required_fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some_and(|v| v >= 1920 && v <= 2002)
            && self.iyr.is_some_and(|v| v >= 2010 && v <= 2020)
            && self.eyr.is_some_and(|v| v >= 2020 && v <= 2030)
            && self.hgt.as_deref().is_some_and(|v| {
            let caps = match HGT_RE.get_or_init(|| Regex::new(r"(?<value>\d+)(?<unit>[a-zA-Z]+)").unwrap())
                .captures(&v) {
                None => { return false; }
                Some(v) => v
            };

            let unit = caps.name("unit").expect(&format!("Could not get height unit: {}", v)).as_str();
            let value = caps.name("value").expect(&format!("Could not get height value: {}", v)).as_str().parse::<usize>().unwrap();

            match unit {
                "cm" => {
                    value >= 150 && value <= 193
                }
                "in" => {
                    value >= 59 && value <= 76
                }
                _ => false
            }
        })
            && self.hcl.as_deref().is_some_and(|v| HCL_RE.get_or_init(|| Regex::new(r"^#[0-9a-f]{6}$").unwrap()).is_match(v))
            && self.ecl.as_deref().is_some_and(|v| match v {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        })
            && self.pid.as_ref().is_some_and(|v| v.len() == 9 && v.chars().all(|c| c.is_digit(10)))
    }
}

fn form_passports(input: &str) -> Vec<Passport> {
    // replace to fix Linux and Windows line issue
    input.replace("\r\n", "\n")
        .split("\n\n")
        .map(|inp| Passport::from_lines(inp))
        .collect()
}

pub fn solve_a() {
    let input = read_contents(4);
    let passports = form_passports(&input);

    let ans = passports
        .iter()
        .filter(|p| p.required_fields_present())
        .count();

    println!("Solution A: {}", ans);
}

pub fn solve_b() {
    let input = read_contents(4);
    let passports = form_passports(&input);

    let ans = passports.iter()
        .filter(|p| p.is_valid())
        .count();

    println!("Solution B: {}", ans);
}


#[cfg(test)]
mod tests {
    use crate::d4::form_passports;

    #[test]
    fn test_passports_required_fields_present() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let passports = form_passports(input);
        assert_eq!(passports.len(), 4);

        let num_valid = passports.iter()
            .filter(|p| p.required_fields_present())
            .count();

        assert_eq!(num_valid, 2);
    }

    #[test]
    fn test_passports_is_valid() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = form_passports(input);
        assert_eq!(passports.len(), 8);

        let num_valid = passports.iter()
            .filter(|p| p.is_valid())
            .count();

        assert_eq!(num_valid, 4);
    }
}