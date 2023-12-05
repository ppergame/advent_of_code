use std::collections::HashSet;

fn count_unique(s: &str) -> usize {
    s.chars()
        .filter(|x| x.is_ascii_lowercase())
        .collect::<HashSet<_>>()
        .len()
}

fn part1(inp: &str) -> usize {
    inp.split("\n\n")
        .map(count_unique)
        .reduce(|a, b| a + b)
        .unwrap()
}

fn part2(inp: &str) -> usize {
    inp.split("\n\n")
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(('a'..='z').collect::<HashSet<_>>(), |b, el| {
                    b.intersection(&el).cloned().collect()
                })
                .len()
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

xaoc::xaoc!();
