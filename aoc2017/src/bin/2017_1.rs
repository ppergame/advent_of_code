use itertools::Itertools;

fn part1(inp: &str) -> u32 {
    let mut ret = 0;
    let inp = inp.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    for (a, b) in inp.iter().tuple_windows() {
        if a == b {
            ret += a;
        }
    }
    if inp[0] == *inp.last().unwrap() {
        ret += inp[0];
    }
    ret
}

fn part2(inp: &str) -> u32 {
    let mut ret = 0;
    let inp = inp.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    for (i, d) in inp.iter().enumerate() {
        if *d == inp[(i + inp.len() / 2) % inp.len()] {
            ret += d;
        }
    }
    ret
}

xaoc::xaoc!();
