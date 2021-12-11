use eyre::Result;
use log::debug;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input4.txt");

pub fn solve() -> Result<()> {
    println!("day4.1: {}", solve1(INPUT));
    println!("day4.2: {}", solve2(INPUT));
    Ok(())
}

fn solve1(input: &str) -> usize {
    let passports = input.split("\n\n");
    passports
        .map(parse_passport)
        .filter(|it| it.valid1())
        .count()
}

fn solve2(input: &str) -> usize {
    let passports = input.split("\n\n");
    passports
        .map(parse_passport)
        .filter(|it| it.valid2())
        .count()
}

fn parse_passport(passport_lines: &str) -> Passport {
    let mut fields = passport_lines
        .split(char::is_whitespace)
        .filter_map(|field| {
            let split = field.trim().split(':').collect::<Vec<_>>();
            Some((split.get(0)?.to_string(), split.get(1)?.to_string()))
        })
        .collect::<HashMap<_, _>>();
    let passport = Passport {
        byr: fields.remove("byr"),
        iyr: fields.remove("iyr"),
        eyr: fields.remove("eyr"),
        hgt: fields.remove("hgt"),
        hcl: fields.remove("hcl"),
        ecl: fields.remove("ecl"),
        pid: fields.remove("pid"),
        cid: fields.remove("cid"),
    };
    debug!("Created passport {:?}", passport);
    passport
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn valid1(&self) -> bool {
        let valid = [
            &self.byr, &self.iyr, &self.eyr, &self.hgt, &self.hcl, &self.ecl, &self.pid,
        ]
        .iter()
        .all(|it| it.is_some());
        debug!("{} {:?}", if valid { "Valid" } else { "Invalid" }, self);
        valid
    }

    fn valid2(&self) -> bool {
        static HAIR_COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new("#[0-9a-f]{6}").unwrap());

        let valid = [
            number_between(self.byr.as_deref(), 1920, 2002),
            number_between(self.iyr.as_deref(), 2010, 2020),
            number_between(self.eyr.as_deref(), 2020, 2030),
            valid_cm(self.hgt.as_deref()) || valid_inch(self.hgt.as_deref()),
            regex_matches(self.hcl.as_deref(), &*HAIR_COLOR_RE),
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .any(|it| Some(*it) == self.ecl.as_deref()),
            match &self.pid {
                Some(pid) => pid.len() == 9 && pid.chars().all(|c| c.is_numeric()),
                None => false,
            },
        ]
        .iter()
        .all(|b| *b);

        debug!("{} {:?}", if valid { "Valid" } else { "Invalid" }, self);

        valid
    }
}

fn valid_cm(height: Option<&str>) -> bool {
    let height = match height {
        Some(it) => it,
        None => return false,
    };
    static CM_RE: Lazy<Regex> = Lazy::new(|| Regex::new("[0-9]{3}cm").unwrap());
    CM_RE.is_match(height) && number_between(Some(&height[..3]), 150, 193)
}

fn valid_inch(height: Option<&str>) -> bool {
    let height = match height {
        Some(it) => it,
        None => return false,
    };
    static INCH_RE: Lazy<Regex> = Lazy::new(|| Regex::new("[0-9]{2}in").unwrap());
    INCH_RE.is_match(height) && number_between(Some(&height[..2]), 59, 76)
}

fn number_between(num: Option<&str>, min: u32, max: u32) -> bool {
    let valid: Option<bool> = try {
        let num: u32 = num.as_ref()?.parse().ok()?;
        num >= min && num <= max
    };
    debug!("{:?} between {} and {}: {:?}", num, min, max, valid);
    valid.unwrap_or(false)
}

fn regex_matches(s: Option<&str>, regex: &Regex) -> bool {
    match s {
        Some(s) => regex.is_match(s),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        assert_eq!(solve1(INPUT), 256);
        assert_eq!(solve2(INPUT), 198);
    }

    #[test]
    fn pt1_ex() {
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

        assert_eq!(solve1(input), 2);
    }

    #[test]
    fn pt2_ex_invalid() {
        let invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        assert_eq!(solve2(invalid), 0);
    }

    #[test]
    fn pt2_ex_valid() {
        let valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
        assert_eq!(solve2(valid), 4);
    }
}
