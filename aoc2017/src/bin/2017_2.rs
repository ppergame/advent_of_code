use itertools::{Itertools, MinMaxResult};

fn part1(inp: &str) -> i64 {
    let mut ret = 0;
    for line in inp.lines() {
        let MinMaxResult::<i64>::MinMax(min, max) = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .minmax() else { unreachable!() };
        ret += max - min;
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let mut ret = 0;
    for line in inp.lines() {
        ret += line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .permutations(2)
            .find_map(|v| {
                let a = v[0];
                let b = v[1];
                if a % b == 0 {
                    Some::<i64>(a / b)
                } else {
                    None
                }
            })
            .unwrap();
    }
    ret
}

xaoc::xaoc!(no_sample = true);
