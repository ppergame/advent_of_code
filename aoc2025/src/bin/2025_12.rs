use sscanf::scanf;

struct Region {
    size: usize,
    needed: Vec<usize>,
}

impl Region {
    fn parse(l: &str) -> Self {
        let (w, h, nn) = scanf!(l, "{usize}x{usize}: {str}").unwrap();
        let needed = nn.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Self {
            size: w * h,
            needed,
        }
    }
}

struct Task {
    shapes: Vec<usize>,
    regions: Vec<Region>,
}

impl Task {
    fn parse(inp: &str) -> Self {
        let (shapes, regions) = inp.rsplit_once("\n\n").unwrap();
        Self {
            shapes: shapes.split("\n\n").map(|s| s.matches('#').count()).collect(),
            regions: regions.lines().map(Region::parse).collect(),
        }
    }
}

fn part1(inp: &str) -> usize {
    let task = Task::parse(inp);
    task.regions
        .iter()
        .filter(|r| {
            r.needed
                .iter()
                .zip(&task.shapes)
                .map(|(n, s)| n * s)
                .sum::<usize>()
                <= r.size
        })
        .count()
}

fn part2(_inp: &str) -> i64 {
    0
}

xaoc::xaoc!();
