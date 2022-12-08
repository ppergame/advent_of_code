use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn part1(inp: &str) -> i64 {
    let mut banks = inp
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();
    let mut seen = HashSet::new();
    let mut cycles = 0;
    loop {
        if seen.contains(&banks) {
            break;
        }
        seen.insert(banks.clone());
        let max_idx = banks
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(idx, val)| (val, -(idx as i64)))
            .unwrap()
            .0;
        let mut distro = banks[max_idx];
        banks[max_idx] = 0;
        let mut idx = (max_idx + 1) % banks.len();
        while distro > 0 {
            banks[idx] += 1;
            distro -= 1;
            idx = (idx + 1) % banks.len();
        }
        cycles += 1;
    }
    cycles
}

fn part2(inp: &str) -> i64 {
    let mut banks = inp
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();
    let mut seen = HashMap::new();
    let mut cycles: i64 = 0;
    loop {
        if let Some(last_cycles) = seen.get(&banks) {
            return cycles - last_cycles;
        }
        seen.insert(banks.clone(), cycles);
        let max_idx = banks
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(idx, val)| (val, -(idx as i64)))
            .unwrap()
            .0;
        let mut distro = banks[max_idx];
        banks[max_idx] = 0;
        let mut idx = (max_idx + 1) % banks.len();
        while distro > 0 {
            banks[idx] += 1;
            distro -= 1;
            idx = (idx + 1) % banks.len();
        }
        cycles += 1;
    }
}

xaoc::xaoc!(sample = "0 2 7 0");
