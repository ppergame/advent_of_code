fn part1(inp: &str) -> i64 {
    let mut floor = 0;
    for c in inp.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
    }
    floor
}

fn part2(inp: &str) -> i64 {
    let mut floor = 0;
    for (idx, c) in inp.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor == -1 {
            return (idx + 1) as i64;
        }
    }
    unreachable!();
}

xaoc::xaoc!();
