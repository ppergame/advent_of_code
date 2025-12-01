use sscanf::scanf;

fn parse(inp: &str) -> Vec<(i64, i64)> {
    inp.lines()
        .map(|l| {
            let (c, n) = scanf!(l, "{char}{i64}").unwrap();
            let d = if c == 'L' { -1 } else { 1 };
            (d, n)
        })
        .collect()
}

fn part1(inp: &str) -> i64 {
    let mut cur = 50;
    let mut count = 0;
    for (d, n) in parse(inp) {
        cur += d * n;
        cur = cur.rem_euclid(100);
        if cur == 0 {
            count += 1
        }
    }
    count
}

fn part2(inp: &str) -> i64 {
    let mut cur = 50;
    let mut count = 0;
    for (d, mut n) in parse(inp) {
        if cur != 0 {
            if d == 1 {
                let to_zero = 100 - cur;
                if n >= to_zero {
                    n -= to_zero;
                    cur = 0;
                    count += 1;
                }
            } else if n >= cur {
                n -= cur;
                cur = 0;
                count += 1;
            }
        }
        count += n / 100;
        n %= 100;
        cur += d * n;
        cur = cur.rem_euclid(100);
    }
    count
}

xaoc::xaoc!(
    sample = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
);
