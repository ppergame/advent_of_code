use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse(inp: &str) -> (HashMap<(&str, &str), i64>, Vec<&str>) {
    let mut util = HashMap::new();
    let mut names = HashSet::new();
    for line in inp.lines() {
        let sp = line
            .strip_suffix('.')
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        names.insert(sp[0]);
        names.insert(sp[10]);
        let mut happ: i64 = sp[3].parse().unwrap();
        if sp[2] == "lose" {
            happ = -happ;
        }
        util.insert((sp[0], sp[10]), happ);
    }
    (util, names.into_iter().sorted().collect())
}

fn part1(inp: &str) -> i64 {
    let (util, names) = parse(inp);
    let mut max = 0;
    let len = names.len();
    for perm in names.into_iter().permutations(len) {
        let mut happ = 0;
        for (i, &name) in perm.iter().enumerate() {
            let prev_i = if i == 0 { len - 1 } else { i - 1 };
            let next_i = if i == len - 1 { 0 } else { i + 1 };
            happ += util[&(name, perm[prev_i])];
            happ += util[&(name, perm[next_i])];
        }
        max = max.max(happ);
    }
    max
}

fn part2(inp: &str) -> i64 {
    let (mut util, mut names) = parse(inp);
    for name in &names {
        util.insert((name, "me"), 0);
        util.insert(("me", name), 0);
    }
    names.push("me");
    let mut max = 0;
    let len = names.len();
    for perm in names.into_iter().permutations(len) {
        let mut happ = 0;
        for (i, &name) in perm.iter().enumerate() {
            let prev_i = if i == 0 { len - 1 } else { i - 1 };
            let next_i = if i == len - 1 { 0 } else { i + 1 };
            happ += util[&(name, perm[prev_i])];
            happ += util[&(name, perm[next_i])];
        }
        max = max.max(happ);
    }
    max
}

xaoc::xaoc!();
