use std::collections::HashSet;

use itertools::Itertools;

fn part1(inp: &str) -> i64 {
    let mut valid_count = 0;
    for line in inp.lines() {
        let words = line.split_ascii_whitespace().collect_vec();
        let words_hash = HashSet::<&str>::from_iter(words.iter().copied());
        if words.len() == words_hash.len() {
            valid_count += 1;
        }
    }
    valid_count
}

fn part2(inp: &str) -> i64 {
    let mut valid_count = 0;
    for line in inp.lines() {
        let words = line.split_ascii_whitespace().collect_vec();
        let words_hash = HashSet::<Vec<char>>::from_iter(
            words
                .iter()
                .copied()
                .map(|s| s.chars().sorted().collect_vec()),
        );
        if words.len() == words_hash.len() {
            valid_count += 1;
        }
    }
    valid_count
}

xaoc::xaoc!();
