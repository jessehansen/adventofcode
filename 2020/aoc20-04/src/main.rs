use std::{collections::HashMap, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    passports: Vec<Passport>,
}

struct Passport {
    fields: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Passport {
            fields: contents
                .split(&[' ', '\n'])
                .filter_map(|x| {
                    let mut parts = x.split(':');
                    if let Some(name) = parts.next() {
                        if let Some(value) = parts.next() {
                            return Some((name.to_owned(), value.to_owned()));
                        }
                    }
                    None
                })
                .collect(),
        })
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            passports: parse_line_groups(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let mut count = self.passports.len();

        for passport in &self.passports {
            for field in required_fields {
                if !passport.fields.contains_key(field) {
                    count -= 1;
                    break;
                }
            }
        }

        Ok(count)
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut count = self.passports.len();
        for passport in &self.passports {
            dbg!(&passport.fields);
            if !valid_byr(passport.fields.get("byr")) {
                count -= 1;
                continue;
            }
            if !valid_iyr(passport.fields.get("iyr")) {
                count -= 1;
                continue;
            }
            if !valid_eyr(passport.fields.get("eyr")) {
                count -= 1;
                continue;
            }
            if !valid_hgt(passport.fields.get("hgt")) {
                count -= 1;
                continue;
            }
            if !valid_hcl(passport.fields.get("hcl")) {
                count -= 1;
                continue;
            }
            if !valid_ecl(passport.fields.get("ecl")) {
                count -= 1;
                continue;
            }
            if !valid_pid(passport.fields.get("pid")) {
                count -= 1;
                continue;
            }
            dbg!("GOOD");
        }

        Ok(count)
    }
}

fn valid_byr(value: Option<&String>) -> bool {
    match value {
        Some(year_string) => match year_string.parse::<u32>() {
            std::result::Result::Ok(year) => (1920..=2002).contains(&year),
            _ => false,
        },
        _ => false,
    }
}
fn valid_iyr(value: Option<&String>) -> bool {
    match value {
        Some(year_string) => match year_string.parse::<u32>() {
            std::result::Result::Ok(year) => (2010..=2020).contains(&year),
            _ => false,
        },
        _ => false,
    }
}
fn valid_eyr(value: Option<&String>) -> bool {
    match value {
        Some(year_string) => match year_string.parse::<u32>() {
            std::result::Result::Ok(year) => (2020..=2030).contains(&year),
            _ => false,
        },
        _ => false,
    }
}
fn valid_hgt(value: Option<&String>) -> bool {
    match value {
        Some(hgt_string) => {
            if hgt_string.ends_with("cm") {
                match hgt_string.substring(0, hgt_string.len() - 2).parse::<u32>() {
                    std::result::Result::Ok(height_cm) => (150..=193).contains(&height_cm),
                    _ => false,
                }
            } else if hgt_string.ends_with("in") {
                match hgt_string.substring(0, hgt_string.len() - 2).parse::<u32>() {
                    std::result::Result::Ok(height_in) => (59..=76).contains(&height_in),
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
fn valid_hcl(value: Option<&String>) -> bool {
    match value {
        Some(hcl_string) => {
            if hcl_string.len() != 7 {
                return false;
            }
            if !hcl_string.starts_with('#') {
                return false;
            }
            for pos in 1..=6 {
                if !matches!(
                    hcl_string.chars().nth(pos),
                    Some('a'..='f') | Some('0'..='9')
                ) {
                    return false;
                }
            }

            true
        }
        _ => false,
    }
}
fn valid_ecl(value: Option<&String>) -> bool {
    match value {
        Some(ecl_string) => matches!(
            ecl_string.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ),
        _ => false,
    }
}
fn valid_pid(value: Option<&String>) -> bool {
    match value {
        Some(pid_string) => {
            if pid_string.len() != 9 {
                return false;
            }

            for pos in 0..=8 {
                if !matches!(pid_string.chars().nth(pos), Some('0'..='9')) {
                    return false;
                }
            }

            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(2, result);

        Ok(())
    }

    #[test]
    fn test_valid_byr() -> Result<()> {
        assert!(!valid_byr(None));
        assert!(!valid_byr(Some(&"abc123".to_string())));
        assert!(!valid_byr(Some(&"1919".to_string())));
        assert!(!valid_byr(Some(&"2003".to_string())));
        assert!(valid_byr(Some(&"1920".to_string())));
        assert!(valid_byr(Some(&"1983".to_string())));
        assert!(valid_byr(Some(&"2002".to_string())));

        Ok(())
    }

    #[test]
    fn test_valid_iyr() -> Result<()> {
        assert!(!valid_iyr(None));
        assert!(!valid_iyr(Some(&"abc123".to_string())));
        assert!(!valid_iyr(Some(&"1909".to_string())));
        assert!(!valid_iyr(Some(&"2009".to_string())));
        assert!(valid_iyr(Some(&"2010".to_string())));
        assert!(valid_iyr(Some(&"2018".to_string())));
        assert!(valid_iyr(Some(&"2020".to_string())));
        assert!(!valid_iyr(Some(&"2021".to_string())));

        Ok(())
    }

    #[test]
    fn test_valid_eyr() -> Result<()> {
        assert!(!valid_eyr(None));
        assert!(!valid_eyr(Some(&"abc123".to_string())));
        assert!(!valid_eyr(Some(&"1909".to_string())));
        assert!(!valid_eyr(Some(&"2019".to_string())));
        assert!(valid_eyr(Some(&"2020".to_string())));
        assert!(valid_eyr(Some(&"2025".to_string())));
        assert!(valid_eyr(Some(&"2030".to_string())));
        assert!(!valid_eyr(Some(&"2031".to_string())));

        Ok(())
    }

    #[test]
    fn test_valid_hgt() -> Result<()> {
        assert!(!valid_hgt(None));
        assert!(!valid_hgt(Some(&"123".to_string())));
        assert!(!valid_hgt(Some(&"abc".to_string())));
        assert!(!valid_hgt(Some(&"5".to_string())));
        assert!(valid_hgt(Some(&"170cm".to_string())));
        assert!(valid_hgt(Some(&"182cm".to_string())));
        assert!(valid_hgt(Some(&"60in".to_string())));
        assert!(valid_hgt(Some(&"75in".to_string())));

        Ok(())
    }

    #[test]
    fn sample_part2_invalid() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_INVALID)?;

        let result = problem.part2()?;

        assert_eq!(0, result);

        Ok(())
    }

    #[test]
    fn sample_part2_valid() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_VALID)?;

        let result = problem.part2()?;

        assert_eq!(4, result);

        Ok(())
    }

    const SAMPLE: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    const SAMPLE_INVALID: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    const SAMPLE_VALID: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
}
