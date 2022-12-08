use std::collections::HashSet;

use itertools::Itertools;

fn has_abba(s: &str) -> bool {
    for (a, b, c, d) in s.chars().tuple_windows() {
        if a != b && a == d && b == c {
            return true;
        }
    }
    false
}

fn extract_aba(s: &str) -> HashSet<String> {
    let mut ret = HashSet::new();
    for (a, b, c) in s.chars().tuple_windows() {
        if a != b && a == c {
            ret.insert(format!("{a}{b}{c}"));
        }
    }
    ret
}

fn extract_aba_flip(s: &str) -> HashSet<String> {
    let mut ret = HashSet::new();
    for (a, b, c) in s.chars().tuple_windows() {
        if a != b && a == c {
            ret.insert(format!("{b}{a}{b}"));
        }
    }
    ret
}

fn part1(inp: &str) -> i64 {
    let mut count = 0;
    for line in inp.lines() {
        let mut found_abba = false;
        let mut found_abba_hyper = false;
        let mut acc = String::new();
        for c in line.chars().chain(std::iter::once(' ')) {
            match c {
                '[' | ' ' => {
                    if !acc.is_empty() && has_abba(&acc) {
                        found_abba = true;
                    }
                    acc.clear();
                }
                ']' => {
                    if !acc.is_empty() && has_abba(&acc) {
                        found_abba_hyper = true;
                        break;
                    }
                    acc.clear();
                }
                _ => acc.push(c),
            }
        }
        if found_abba && !found_abba_hyper {
            count += 1;
        }
    }
    count
}

fn part2(inp: &str) -> i64 {
    let mut count = 0;
    for line in inp.lines() {
        let mut aba_supernet = HashSet::new();
        let mut aba_hypernet = HashSet::new();
        let mut acc = String::new();
        for c in line.chars().chain(std::iter::once(' ')) {
            match c {
                '[' | ' ' => {
                    if !acc.is_empty() {
                        aba_supernet.extend(extract_aba(&acc));
                    }
                    acc.clear();
                }
                ']' => {
                    if !acc.is_empty() {
                        aba_hypernet.extend(extract_aba_flip(&acc));
                    }
                    acc.clear();
                }
                _ => acc.push(c),
            }
        }
        if aba_supernet.intersection(&aba_hypernet).next().is_some() {
            count += 1;
        }
    }
    count
}

xaoc::xaoc!();
