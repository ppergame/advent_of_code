use hashbrown::HashMap;
use itertools::Itertools;

fn parse(inp: &str) -> (Vec<i64>, Vec<i64>) {
    inp.lines()
        .map(|l| {
            let (a, b) = l.split_whitespace().next_tuple().unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .unzip()
}

fn part1(inp: &str) -> i64 {
    let (mut v1, mut v2) = parse(inp);
    v1.sort();
    v2.sort();
    v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn part2(inp: &str) -> i64 {
    let (v1, v2) = parse(inp);
    let mut counts = HashMap::new();
    for b in v2 {
        *counts.entry(b).or_insert(0) += 1;
    }
    v1.into_iter()
        .map(|a| a * counts.get(&a).unwrap_or(&0))
        .sum()
}

xaoc::xaoc!();
