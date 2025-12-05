use rangemap::RangeInclusiveSet;

struct Inp {
    fresh: RangeInclusiveSet<i64>,
    ids: Vec<i64>,
}

impl Inp {
    fn parse(inp: &str) -> Self {
        let mut lines = inp.lines();
        let mut fresh = RangeInclusiveSet::new();
        for l in &mut lines {
            if l.is_empty() {
                break;
            }
            let (start, end) = sscanf::scanf!(l, "{i64}-{i64}").unwrap();
            fresh.insert(start..=end);
        }
        let ids = lines.map(|l| l.parse().unwrap()).collect();
        Self { fresh, ids }
    }
}

fn part1(inp: &str) -> usize {
    let inp = Inp::parse(inp);
    inp.ids.iter().filter(|id| inp.fresh.contains(id)).count()
}

fn part2(inp: &str) -> i64 {
    let inp = Inp::parse(inp);
    inp.fresh.iter().map(|r| r.end() - r.start() + 1).sum()
}

xaoc::xaoc!();
