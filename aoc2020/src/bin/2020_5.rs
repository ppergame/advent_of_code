fn bsp_to_row(bsp: &str) -> usize {
    let mut start = 0;
    let mut end = 128;
    for c in bsp.chars() {
        match c {
            'F' => end -= (end - start) / 2,
            'B' => start += (end - start) / 2,
            _ => panic!(),
        }
    }
    start
}

fn bsp_to_col(bsp: &str) -> usize {
    let mut start = 0;
    let mut end = 8;
    for c in bsp.chars() {
        match c {
            'L' => end -= (end - start) / 2,
            'R' => start += (end - start) / 2,
            _ => panic!(),
        }
    }
    start
}

fn make_passes(inp: &str) -> impl Iterator<Item = usize> + '_ {
    inp.lines().map(|line| {
        let (r, c) = line.split_at(7);
        let row = bsp_to_row(r);
        let col = bsp_to_col(c);
        row * 8 + col
    })
}

fn part1(inp: &str) -> usize {
    make_passes(inp).max().unwrap()
}

fn part2(inp: &str) -> usize {
    let mut passes = make_passes(inp).collect::<Vec<_>>();
    passes.sort_unstable();
    let mut seatid = passes[0];
    for pass in passes {
        if pass != seatid {
            return seatid;
        }
        seatid += 1;
    }
    unreachable!();
}

xaoc::xaoc!();
