fn part1(inp: &str) -> i64 {
    let inp = inp.parse::<usize>().unwrap();
    let mut buf = vec![0];
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + inp) % buf.len();
        buf.insert(pos + 1, i);
        pos += 1;
    }
    buf[pos + 1]
}

fn part2(inp: &str) -> usize {
    let inp = inp.parse::<usize>().unwrap();
    // current position, in steps after 0 position
    let mut pos = 0;
    let mut ret = 0;
    for len in 1..=50000000 {
        pos = (pos + inp + 1) % len;
        match pos {
            0 => pos = len,
            1 => ret = len,
            _ => (),
        }
    }
    ret
}

xaoc::xaoc!(sample = "3");
