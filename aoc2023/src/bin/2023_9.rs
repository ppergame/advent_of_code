use itertools::Itertools as _;

fn solve(mut vals: Vec<i64>) -> i64 {
    let mut ret = 0;
    loop {
        ret += vals.last().unwrap();
        vals = vals
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        if vals.iter().all(|&x| x == 0) {
            break;
        }
    }
    ret
}

fn part1(inp: &str) -> i64 {
    let mut ret = 0;
    for line in inp.lines() {
        let vals = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        ret += solve(vals);
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let mut ret = 0;
    for line in inp.lines() {
        let vals = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .rev()
            .collect::<Vec<_>>();
        ret += solve(vals);
    }
    ret
}

xaoc::xaoc!();

// 13 9 -1 1 -2 10 7 12 13 -5 15 14
