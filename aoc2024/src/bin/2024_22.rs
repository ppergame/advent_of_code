use itertools::Itertools as _;
use std::collections::HashMap;

fn step(mut n: i64) -> i64 {
    n ^= n * 64;
    n %= 16777216;
    n ^= n / 32;
    n %= 16777216;
    n ^= n * 2048;
    n %= 16777216;
    n
}

fn part1(inp: &str) -> i64 {
    inp.lines()
        .map(|l| {
            let mut n = l.parse::<i64>().unwrap();
            for _ in 0..2000 {
                n = step(n);
            }
            n
        })
        .sum()
}

fn seqmap(n: i64) -> HashMap<[i64; 4], i64> {
    (0..2000)
        .scan(n, |n, _| {
            let ret = *n;
            *n = step(*n);
            Some(ret)
        })
        .tuple_windows()
        .map(|(a, b)| (b % 10, (b % 10) - (a % 10)))
        .tuple_windows()
        .map(|((_, a), (_, b), (_, c), (price, d))| ([a, b, c, d], price))
        .into_grouping_map()
        .reduce(|price, _, _| price)
}

fn part2(inp: &str) -> i64 {
    let mut vals = HashMap::new();
    for l in inp.lines() {
        let n = l.parse::<i64>().unwrap();
        for (seq, price) in seqmap(n) {
            *vals.entry(seq).or_insert(0) += price;
        }
    }
    *vals.values().max().unwrap()
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
