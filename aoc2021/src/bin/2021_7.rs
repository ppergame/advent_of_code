fn parse(inp: &str) -> Vec<i64> {
    inp.split(',').map(|c| c.parse().unwrap()).collect()
}

fn part1(inp: &str) -> i64 {
    let inp = parse(inp);
    let min = *inp.iter().min().unwrap();
    let max = *inp.iter().max().unwrap();
    (min..=max)
        .map(|i| inp.iter().map(|p| (i - p).abs()).sum())
        .min()
        .unwrap()
}

fn cost(p1: i64, p2: i64) -> i64 {
    let c = (p1 - p2).abs();
    c * (c + 1) / 2
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let min = *inp.iter().min().unwrap();
    let max = *inp.iter().max().unwrap();
    (min..=max)
        .map(|i| inp.iter().map(|&p| cost(p, i)).sum())
        .min()
        .unwrap()
}

xaoc::xaoc!();
