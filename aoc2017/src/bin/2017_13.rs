use itertools::Itertools;
use sscanf::scanf;

fn is_zero_at(range: i64, at: i64) -> bool {
    let period = (range - 1) * 2;
    at % period == 0
}

fn part1(inp: &str) -> i64 {
    let layers = inp
        .lines()
        .map(|line| {
            let (left, right) = scanf!(line, "{}: {}", i64, i64).unwrap();
            (left, right)
        })
        .collect_vec();
    let mut sev = 0;
    for (depth, range) in layers {
        if is_zero_at(range, depth) {
            sev += depth * range;
        }
    }
    sev
}

fn part2(inp: &str) -> i64 {
    let layers = inp
        .lines()
        .map(|line| {
            let (left, right) = scanf!(line, "{}: {}", i64, i64).unwrap();
            (left, right)
        })
        .collect_vec();
    for delay in 0..10000000 {
        let mut found = true;
        for &(depth, range) in &layers {
            if is_zero_at(range, depth + delay) {
                found = false;
                break;
            }
        }
        if found {
            return delay;
        }
    }
    unreachable!();
}

xaoc::xaoc!(sample_idx = 4);
