use itertools::Itertools;

struct Boxx(u64, u64, u64);

fn parse(inp: &str) -> Vec<Boxx> {
    inp.lines()
        .map(|line| {
            let mut b = line.split('x').map(|d| d.parse().unwrap()).sorted();
            Boxx(b.next().unwrap(), b.next().unwrap(), b.next().unwrap())
        })
        .collect()
}

fn part1(inp: &str) -> u64 {
    let bb = parse(inp);
    bb.iter()
        .map(|b| 2 * b.0 * b.1 + 2 * b.1 * b.2 + 2 * b.0 * b.2 + b.0 * b.1)
        .sum()
}

fn part2(inp: &str) -> u64 {
    let bb = parse(inp);
    bb.iter().map(|b| 2 * b.0 + 2 * b.1 + b.0 * b.1 * b.2).sum()
}

xaoc::xaoc!();
