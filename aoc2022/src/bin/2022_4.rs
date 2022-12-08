use sscanf::scanf;

fn part1(inp: &str) -> i64 {
    let mut count = 0;
    for line in inp.lines() {
        let (s1, e1, s2, e2) = scanf!(line, "{}-{},{}-{}", i64, i64, i64, i64).unwrap();
        if (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1) {
            count += 1;
        }
    }
    count
}

fn part2(inp: &str) -> i64 {
    let mut count = 0;
    for line in inp.lines() {
        let (s1, e1, s2, e2) = scanf!(line, "{}-{},{}-{}", i64, i64, i64, i64).unwrap();
        if !(s1 > e2 || s2 > e1) {
            count += 1;
        }
    }
    count
}

xaoc::xaoc!();
