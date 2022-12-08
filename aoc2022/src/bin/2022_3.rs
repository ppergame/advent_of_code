use itertools::Itertools;
use std::collections::HashSet;

fn prio(c: char) -> u8 {
    if c.is_ascii_uppercase() {
        return (c as u8) + 26 - 64;
    }
    (c as u8) - 96
}

fn part1(inp: &str) -> i64 {
    let mut ret = 0;
    for line in inp.lines() {
        let v = line.chars().collect_vec();
        let (v1, v2) = v.split_at(v.len() / 2);
        let v1 = HashSet::<&char>::from_iter(v1);
        let v2 = HashSet::from_iter(v2);
        let mut int = v1.intersection(&v2);
        let c = int.next().unwrap();
        assert!(int.next().is_none());
        ret += prio(**c) as i64;
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let mut ret = 0;
    for (l1, l2, l3) in inp.lines().tuples() {
        let h1 = HashSet::<char>::from_iter(l1.chars());
        let h2 = HashSet::from_iter(l2.chars());
        let h3 = HashSet::<char>::from_iter(l3.chars());
        let i = HashSet::from_iter(h1.intersection(&h2).copied());
        let i = HashSet::<&char>::from_iter(i.intersection(&h3));
        assert_eq!(i.len(), 1);
        ret += prio(*i.into_iter().next().unwrap()) as i64;
    }
    ret
}

xaoc::xaoc!(sample_idx = 2);
