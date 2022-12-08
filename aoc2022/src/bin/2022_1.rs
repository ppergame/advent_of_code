use itertools::Itertools;

fn part1(inp: &str) -> i64 {
    inp.split("\n\n")
        .map(|chunk| chunk.lines().map(|line| line.parse::<i64>().unwrap()))
        .map(|e| e.sum())
        .max()
        .unwrap()
}

fn part2(inp: &str) -> i64 {
    inp.split("\n\n")
        .map(|chunk| chunk.lines().map(|line| line.parse::<i64>().unwrap()))
        .map(|e| e.sum::<i64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

xaoc::xaoc!();
