use itertools::Itertools;

fn parse(inp: &str) -> Vec<Vec<i64>> {
    inp.lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn val(r: &[i64]) -> bool {
    let inc = r[0] < r[1];
    for (a, b) in r.iter().tuple_windows() {
        let d = b - a;
        match (inc, d) {
            (true, 1) => (),
            (true, 2) => (),
            (true, 3) => (),
            (false, -1) => (),
            (false, -2) => (),
            (false, -3) => (),
            _ => return false,
        }
    }
    true
}

fn val2(r: &[i64]) -> bool {
    for (i, (a, b, c)) in r.iter().tuple_windows().enumerate() {
        let d1 = a - b;
        let d2 = b - c;
        if d1.signum() != d2.signum()
            || !(1..=3).contains(&d1.abs())
            || !(1..=3).contains(&d2.abs())
        {
            // second chance mode
            for i in i..=i + 2 {
                let (s1, s2) = r.split_at(i);
                let mut v = s1.to_vec();
                v.extend_from_slice(&s2[1..]);
                if val(&v) {
                    return true;
                }
            }
            return false;
        }
    }
    true
}

fn part1(inp: &str) -> usize {
    let reports = parse(inp);
    reports.into_iter().filter(|r| val(r)).count()
}

fn part2(inp: &str) -> usize {
    let reports = parse(inp);
    reports.into_iter().filter(|r| val2(r)).count()
}

xaoc::xaoc!();
