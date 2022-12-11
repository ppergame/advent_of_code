use std::collections::HashSet;

fn part1(inp: &str) -> i64 {
    let mut sum = 0;
    for line in inp.lines() {
        sum += line.parse::<i64>().unwrap();
    }
    sum
}

fn part2(inp: &str) -> i64 {
    let mut hist = HashSet::new();
    let mut sum = 0;
    for val in inp.lines().map(|line| line.parse::<i64>().unwrap()).cycle() {
        sum += val;
        if hist.contains(&sum) {
            return sum;
        }
        hist.insert(sum);
    }
    unreachable!();
}

xaoc::xaoc!(no_sample = true);
