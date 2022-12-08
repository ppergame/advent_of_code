use itertools::Itertools;
use regex::Regex;

fn part1(inp: &str) -> i32 {
    let sp: Vec<_> = inp.split('-').collect();
    let start: i32 = sp[0].parse().unwrap();
    let end: i32 = sp[1].parse().unwrap();
    let dd_re = Regex::new(&(0..10).map(|x| x.to_string().repeat(2)).join("|")).unwrap();
    let mut count = 0;
    for i in start..end + 1 {
        let s = i.to_string();
        if !dd_re.is_match(&s) {
            continue;
        }
        if s != s.chars().sorted().collect::<String>() {
            continue;
        }
        count += 1;
    }
    count
}

fn part2(inp: &str) -> i32 {
    let sp: Vec<_> = inp.split('-').collect();
    let start: i32 = sp[0].parse().unwrap();
    let end: i32 = sp[1].parse().unwrap();
    let dd_re = Regex::new(
        &(0..10)
            .map(|x| format!("([^{d}]{d}{d}[^{d}])", d = x.to_string()))
            .join(r"|"),
    )
    .unwrap();
    let mut count = 0;
    for i in start..end + 1 {
        let s = i.to_string();
        if !dd_re.is_match(&("_".to_owned() + &s + "_")) {
            continue;
        }
        if s != s.chars().sorted().collect::<String>() {
            continue;
        }
        count += 1;
    }
    count
}

xaoc::xaoc!();
