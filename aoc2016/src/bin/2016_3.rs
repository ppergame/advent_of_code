use itertools::Itertools;

fn part1(inp: &str) -> i64 {
    let mut count = 0;
    for (t1, t2, t3) in inp
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .tuples()
    {
        if poss(t1, t2, t3) {
            count += 1
        }
    }
    count
}

fn poss(t1: i64, t2: i64, t3: i64) -> bool {
    t1 < t2 + t3 && t2 < t1 + t3 && t3 < t1 + t2
}

fn part2(inp: &str) -> i64 {
    let mut count = 0;
    for (t1, t2, t3, t4, t5, t6, t7, t8, t9) in inp
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .tuples()
    {
        if poss(t1, t4, t7) {
            count += 1
        }
        if poss(t2, t5, t8) {
            count += 1
        }
        if poss(t3, t6, t9) {
            count += 1
        }
    }
    count
}

xaoc::xaoc!();
