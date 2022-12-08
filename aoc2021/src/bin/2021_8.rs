use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Display {
    left: HashSet<Vec<char>>,
    right: Vec<Vec<char>>,
}

fn parse(inp: &str) -> Vec<Display> {
    inp.lines()
        .map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();
            Display {
                left: left
                    .split_whitespace()
                    .map(|w| w.chars().sorted().collect())
                    .collect(),
                right: right
                    .split_whitespace()
                    .map(|w| w.chars().sorted().collect())
                    .collect(),
            }
        })
        .collect()
}

fn part1(inp: &str) -> i64 {
    let disp = parse(inp);
    let mut count = 0;
    for d in disp {
        for w in &d.right {
            let l = w.len();
            if l == 2 || l == 4 || l == 3 || l == 7 {
                count += 1;
            }
        }
    }
    count
}

fn part2(inp: &str) -> usize {
    let disp = parse(inp);
    disp.into_par_iter()
        .map(|dis| {
            let map = "abcdefg"
                .chars()
                .permutations(7)
                .par_bridge()
                .find_map_any(|m| {
                    if let [a, b, c, d, e, f, g] = &m[..] {
                        let map = [
                            vec![a, b, c, e, f, g],
                            vec![c, f],
                            vec![a, c, d, e, g],
                            vec![a, c, d, f, g],
                            vec![b, c, d, f],
                            vec![a, b, d, f, g],
                            vec![a, b, d, e, f, g],
                            vec![a, c, f],
                            vec![a, b, c, d, e, f, g],
                            vec![a, b, c, d, f, g],
                        ]
                        .into_iter()
                        .map(|x| x.into_iter().copied().sorted().collect())
                        .collect::<Vec<Vec<_>>>();
                        if map.iter().all(|w| dis.left.contains(w)) {
                            Some(map)
                        } else {
                            None
                        }
                    } else {
                        unreachable!()
                    }
                })
                .unwrap();
            dis.right
                .iter()
                .map(|w| map.iter().position(|x| x == w).unwrap())
                .fold(0, |s, d| s * 10 + d)
        })
        .sum()
}

xaoc::xaoc!();
