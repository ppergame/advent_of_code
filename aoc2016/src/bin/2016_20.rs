use rangemap::RangeInclusiveSet;
use sscanf::scanf;

fn part1(inp: &str) -> u32 {
    let mut allowed = RangeInclusiveSet::new();
    allowed.insert(0..=4294967295);
    for line in inp.lines() {
        let (start, end) = scanf!(line, "{}-{}", u32, u32).unwrap();
        allowed.remove(start..=end);
    }
    *allowed.iter().next().unwrap().start()
}

fn part2(inp: &str) -> usize {
    let mut allowed = RangeInclusiveSet::new();
    allowed.insert(0..=4294967295);
    for line in inp.lines() {
        let (start, end) = scanf!(line, "{}-{}", u32, u32).unwrap();
        allowed.remove(start..=end);
    }
    allowed
        .into_iter()
        .map(|r| (r.end() + 1 - r.start()) as usize)
        .sum()
}

xaoc::xaoc!(sample_idx = 4);
