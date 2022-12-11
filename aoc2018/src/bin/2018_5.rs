use std::collections::HashSet;

use itertools::Itertools;

fn collapse(mut inp: Vec<char>) -> Vec<char> {
    let mut skip = false;
    loop {
        let mut new_inp = vec![];
        for (&a, &b) in inp.iter().tuple_windows() {
            if skip {
                skip = false;
                continue;
            }
            if (a as u8).abs_diff(b as u8) == 32 {
                skip = true;
            } else {
                new_inp.push(a);
            }
        }
        if !skip {
            new_inp.push(*inp.last().unwrap());
        }
        if new_inp.len() == inp.len() {
            break;
        }
        inp = new_inp;
    }
    inp
}

fn part1(inp: &str) -> usize {
    let inp = collapse(inp.chars().collect_vec());
    inp.len()
}

fn part2(inp: &str) -> usize {
    let uniques = HashSet::<char>::from_iter(inp.chars().map(|c| c.to_ascii_lowercase()));
    let mut min = usize::MAX;
    for u in uniques {
        let inp = inp
            .chars()
            .filter(|&c| c != u && c != u.to_ascii_uppercase())
            .collect_vec();
        min = min.min(collapse(inp).len());
    }
    min
}

xaoc::xaoc!(no_sample = true);
