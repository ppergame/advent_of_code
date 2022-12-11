use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let mut twos = 0;
    let mut threes = 0;
    for line in inp.lines() {
        let c = line.chars().collect::<counter::Counter<_>>();
        if c.values().any(|&n| n == 2) {
            twos += 1;
        }
        if c.values().any(|&n| n == 3) {
            threes += 1;
        }
    }
    twos * threes
}

fn part2(inp: &str) -> String {
    inp.lines()
        .tuple_combinations()
        .find_map(|(a, b)| {
            let mut acc = String::new();
            for (c1, c2) in a.chars().zip(b.chars()) {
                if c1 == c2 {
                    acc.push(c1);
                }
            }
            if acc.len() == a.len() - 1 {
                Some(acc)
            } else {
                None
            }
        })
        .unwrap()
}

xaoc::xaoc!(no_sample = true);
