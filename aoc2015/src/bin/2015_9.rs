use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse(inp: &str) -> (HashMap<(&str, &str), usize>, Vec<&str>) {
    let mut dests = HashSet::new();
    let mut map = HashMap::new();
    for line in inp.lines() {
        let sp = line.split_whitespace().collect::<Vec<_>>();
        map.insert((sp[0], sp[2]), sp[4].parse().unwrap());
        map.insert((sp[2], sp[0]), sp[4].parse().unwrap());
        dests.insert(sp[0]);
        dests.insert(sp[2]);
    }
    (map, dests.into_iter().sorted().collect())
}

fn part1(inp: &str) -> usize {
    let (map, dests) = parse(inp);
    let len = dests.len();
    dests
        .into_iter()
        .permutations(len)
        .map(|perm| {
            perm.iter()
                .fold((0, None), |(cost, prev), cur| {
                    if let Some(prev) = prev {
                        (cost + map[&(prev, *cur)], Some(cur))
                    } else {
                        (cost, Some(cur))
                    }
                })
                .0
        })
        .min()
        .unwrap()
}

fn part2(inp: &str) -> usize {
    let (map, dests) = parse(inp);
    let len = dests.len();
    dests
        .into_iter()
        .permutations(len)
        .map(|perm| {
            perm.iter()
                .fold((0, None), |(cost, prev), cur| {
                    if let Some(prev) = prev {
                        (cost + map[&(prev, *cur)], Some(cur))
                    } else {
                        (cost, Some(cur))
                    }
                })
                .0
        })
        .max()
        .unwrap()
}

xaoc::xaoc!();
