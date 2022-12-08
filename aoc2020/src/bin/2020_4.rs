use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

lazy_static::lazy_static! {
    static ref PREFIXES: HashSet<&'static str> = HashSet::from_iter(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]);
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6,6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9,9}$").unwrap();
}

fn part1(inp: &str) -> i32 {
    let mut valid = 0;
    for passport in inp.trim().split("\n\n") {
        let mut found_prefixes = passport
            .split_ascii_whitespace()
            .map(|chunk| {
                let (key, _) = chunk.split_once(':').unwrap();
                key
            })
            .collect::<HashSet<_>>();
        found_prefixes.insert("cid");
        if *PREFIXES == found_prefixes {
            valid += 1;
        }
    }
    valid
}

fn part2(inp: &str) -> i32 {
    let mut valid = 0;
    for passport in inp.trim().split("\n\n") {
        let mut found_prefixes = HashSet::new();
        found_prefixes.insert("cid");
        if !passport.split_ascii_whitespace().any(|chunk| {
            let (key, val) = chunk.split_once(':').unwrap();
            found_prefixes.insert(key);
            match key {
                "byr" => {
                    let byr: u32 = val.parse().unwrap();
                    !(1920..=2002).contains(&byr)
                }
                "iyr" => {
                    let iyr: u32 = val.parse().unwrap();
                    !(2010..=2020).contains(&iyr)
                }
                "eyr" => {
                    let eyr: u32 = val.parse().unwrap();
                    !(2020..=2030).contains(&eyr)
                }
                "hgt" => {
                    if val.ends_with("cm") {
                        let hgt_cm: u32 = val.trim_end_matches("cm").parse().unwrap();
                        !(150..=193).contains(&hgt_cm)
                    } else if val.ends_with("in") {
                        let hgt_in: u32 = val.trim_end_matches("in").parse().unwrap();
                        !(59..=76).contains(&hgt_in)
                    } else {
                        true
                    }
                }
                "hcl" => !HCL_RE.is_match(val),
                "ecl" => !ECL_RE.is_match(val),
                "pid" => !PID_RE.is_match(val),
                _ => false,
            }
        }) && *PREFIXES == found_prefixes
        {
            valid += 1;
        }
    }

    valid
}

xaoc::xaoc!();
