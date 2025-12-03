use hashbrown::HashMap;

fn parse(inp: &str) -> Vec<Vec<i64>> {
    inp.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(inp: &str) -> i64 {
    let mut ret = 0;
    for dd in parse(inp) {
        let mut best = 0;
        for (i, first) in dd.iter().enumerate() {
            for second in &dd[i + 1..] {
                best = best.max(first * 10 + second);
            }
        }
        ret += best;
    }
    ret
}

#[derive(Default, Debug)]
struct FindBest {
    cache: HashMap<(usize, usize), i64>,
}

impl FindBest {
    fn find_best(&mut self, dd: &[i64], count: usize) -> i64 {
        assert!(dd.len() >= count);
        if let Some(&ret) = self.cache.get(&(dd.len(), count)) {
            return ret;
        }
        let lead_slice = &dd[0..dd.len() + 1 - count];
        let max = *lead_slice.iter().max().unwrap();
        if count == 1 {
            return max;
        }
        let mut best = 0;
        for (i, &digit) in lead_slice.iter().enumerate() {
            if digit != max {
                continue;
            }
            best = best
                .max(digit * 10i64.pow(count as u32 - 1) + self.find_best(&dd[i + 1..], count - 1));
        }
        self.cache.insert((dd.len(), count), best);
        best
    }
}

fn part2(inp: &str) -> i64 {
    let mut ret = 0;
    for dd in parse(inp) {
        let mut fb = FindBest::default();
        ret += fb.find_best(&dd, 12);
    }
    ret
}

xaoc::xaoc!(
    sample = "987654321111111
811111111111119
234234234234278
818181911112111"
);
