use itertools::Itertools as _;
use rayon::prelude::*;
use std::collections::HashMap;

struct Seq(i64);

impl Iterator for Seq {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = self.0;
        let ret = n;
        n ^= n * 64;
        n %= 16777216;
        n ^= n / 32;
        n %= 16777216;
        n ^= n * 2048;
        n %= 16777216;
        self.0 = n;
        Some(ret)
    }
}

fn part1(inp: &str) -> i64 {
    inp.lines()
        .map(|l| {
            let n = l.parse().unwrap();
            Seq(n).nth(2000).unwrap()
        })
        .sum()
}

fn seqmap(n: i64) -> HashMap<[i64; 4], i64> {
    Seq(n)
        .tuple_windows()
        .map(|(a, b)| (b % 10, (b % 10) - (a % 10)))
        .take(2000)
        .tuple_windows()
        .map(|((_, a), (_, b), (_, c), (price, d))| ([a, b, c, d], price))
        .into_grouping_map()
        .reduce(|price, _, _| price)
}

fn part2(inp: &str) -> i64 {
    *inp.par_lines()
        .map(|l| {
            let n = l.parse().unwrap();
            seqmap(n)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .into_grouping_map()
        .sum()
        .values()
        .max()
        .unwrap()
}

xaoc::xaoc!(
    sample = "1
10
100
2024",
    sample2 = "1
2
3
2024"
);
