fn parse(inp: &str) -> Vec<usize> {
    inp.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(inp: &str) -> usize {
    let meas = parse(inp);
    meas.windows(2).filter(|m| m[1] > m[0]).count()
}

fn part2(inp: &str) -> usize {
    let meas = parse(inp);
    meas.windows(4).filter(|m| m[3] > m[0]).count()
}

xaoc::xaoc!();
