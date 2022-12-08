use aoc2016::assem::Bunny;

fn part1(inp: &str) -> i64 {
    let mut b = Bunny::parse(inp);
    b.a = 7;
    b.run();
    b.a
}

fn part2(inp: &str) -> i64 {
    let mut b = Bunny::parse(inp);
    b.a = 12;
    b.run();
    b.a
}

xaoc::xaoc!(sample_idx = 16, no_sample = true);
