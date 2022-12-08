use itertools::Itertools;
use std::collections::HashMap;

fn part1(inp: &str) -> String {
    let mut freqs = vec![];
    for line in inp.lines() {
        if freqs.is_empty() {
            for _ in line.chars() {
                freqs.push(HashMap::<char, i64>::new());
            }
        }
        for (i, c) in line.chars().enumerate() {
            *freqs[i].entry(c).or_default() += 1;
        }
    }
    freqs
        .into_iter()
        .map(|p| p.into_iter().sorted_by_key(|(_, f)| -f).next().unwrap().0)
        .join("")
}

fn part2(inp: &str) -> String {
    let mut freqs = vec![];
    for line in inp.lines() {
        if freqs.is_empty() {
            for _ in line.chars() {
                freqs.push(HashMap::<char, i64>::new());
            }
        }
        for (i, c) in line.chars().enumerate() {
            *freqs[i].entry(c).or_default() += 1;
        }
    }
    freqs
        .into_iter()
        .map(|p| p.into_iter().sorted_by_key(|(_, f)| *f).next().unwrap().0)
        .join("")
}

xaoc::xaoc!();
