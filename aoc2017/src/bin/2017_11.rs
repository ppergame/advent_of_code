fn part1(inp: &str) -> i64 {
    let (mut q, mut r, mut s): (i64, i64, i64) = (0, 0, 0);
    for mv in inp.split(',') {
        let (dq, dr, ds) = match mv {
            "n" => (0, -1, 1),
            "ne" => (1, -1, 0),
            "se" => (1, 0, -1),
            "s" => (0, 1, -1),
            "sw" => (-1, 1, 0),
            "nw" => (-1, 0, 1),
            _ => unreachable!(),
        };
        q += dq;
        r += dr;
        s += ds;
    }
    (q.abs() + r.abs() + s.abs()) / 2
}

fn part2(inp: &str) -> i64 {
    let mut max = 0;
    let (mut q, mut r, mut s): (i64, i64, i64) = (0, 0, 0);
    for mv in inp.split(',') {
        let (dq, dr, ds) = match mv {
            "n" => (0, -1, 1),
            "ne" => (1, -1, 0),
            "se" => (1, 0, -1),
            "s" => (0, 1, -1),
            "sw" => (-1, 1, 0),
            "nw" => (-1, 0, 1),
            _ => unreachable!(),
        };
        q += dq;
        r += dr;
        s += ds;
        max = max.max((q.abs() + r.abs() + s.abs()) / 2);
    }
    max
}

xaoc::xaoc!(sample = "se,sw,se,sw,sw");
