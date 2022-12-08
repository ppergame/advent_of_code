use aoc2016::assem::Bunny;

fn part1(inp: &str) -> i64 {
    let mut b = Bunny::parse(inp);
    b.run();
    b.a
}

fn part2(inp: &str) -> i64 {
    let mut b = Bunny::parse(inp);
    b.c = 1;
    b.run();
    b.a
}

xaoc::xaoc!();
