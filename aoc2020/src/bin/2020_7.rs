use std::collections::HashMap;

use regex::Regex;

lazy_static::lazy_static! {
    static ref CONTAIN_RE: Regex = Regex::new(r"(.+) bags contain(.+)").unwrap();
    static ref NBAGS_RE: Regex = Regex::new(r" (\d+) ([^0-9]+) bags?").unwrap();
}

fn parse_bags(inp: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut ret = HashMap::new();
    for line in inp.lines() {
        let cap = CONTAIN_RE.captures(line).unwrap();
        let bag = &cap[1];
        let mut v = Vec::new();
        if &cap[2] != "no other bags." {
            for cap in NBAGS_RE.captures_iter(&cap[2]) {
                v.push((cap[1].parse().unwrap(), cap[2].to_string()));
            }
        }
        ret.insert(bag.to_string(), v);
    }
    ret
}

fn part1(inp: &str) -> usize {
    let bags = parse_bags(inp);
    let mut contains_gold = HashMap::<String, bool>::new();
    loop {
        let mut found = false;
        for (bag, v) in &bags {
            if contains_gold.contains_key(bag) {
                continue;
            }
            found = true;
            let mut dunno = false;
            let mut contains = false;
            for (_, cbag) in v {
                if cbag == "shiny gold" {
                    contains = true;
                    break;
                }
                if let Some(true) = contains_gold.get(cbag) {
                    contains = true;
                    break;
                }
                if !contains_gold.contains_key(cbag) {
                    dunno = true;
                    break;
                }
            }
            if contains {
                contains_gold.insert(bag.to_string(), true);
                continue;
            }
            if dunno {
                continue;
            }
            contains_gold.insert(bag.to_string(), contains);
        }
        if !found {
            break;
        }
    }
    contains_gold.values().filter(|x| **x).count()
}

fn part2(inp: &str) -> usize {
    let mut contains_count = HashMap::<String, usize>::new();
    let bags = parse_bags(inp);
    loop {
        let mut found = false;
        for (bag, v) in &bags {
            if contains_count.contains_key(bag) {
                continue;
            }
            found = true;
            let mut dunno = false;
            let mut count = 0;
            for (c, cbag) in v {
                if let Some(rc) = contains_count.get(cbag) {
                    count += (rc + 1) * c;
                    continue;
                }
                dunno = true;
                break;
            }
            if dunno {
                continue;
            }
            contains_count.insert(bag.to_string(), count);
        }
        if !found {
            break;
        }
    }
    contains_count["shiny gold"]
}

xaoc::xaoc!();
