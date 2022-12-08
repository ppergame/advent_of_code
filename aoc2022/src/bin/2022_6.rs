use itertools::Itertools;
use std::collections::HashSet;

fn solve(inp: &str, count: usize) -> usize {
    let inp = inp.chars().collect_vec();
    inp.windows(count)
        .position(|w| w.len() == HashSet::<&char>::from_iter(w.iter()).len())
        .unwrap()
        + count
}

fn part1(inp: &str) -> usize {
    solve(inp, 4)
}

fn part2(inp: &str) -> usize {
    solve(inp, 14)
}

xaoc::xaoc!();
