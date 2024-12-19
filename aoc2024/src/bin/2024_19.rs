use hashbrown::HashMap;
use std::cell::RefCell;

struct M {
    stripes: Vec<Vec<char>>,
    goals: Vec<Vec<char>>,
    cache: RefCell<HashMap<Vec<char>, usize>>,
}

impl M {
    fn parse(inp: &str) -> Self {
        let mut it = inp.lines();
        let stripes = it
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().chars().collect())
            .collect();
        assert_eq!(it.next(), Some(""));
        let goals = it.map(|s| s.chars().collect()).collect();
        Self {
            stripes,
            goals,
            cache: Default::default(),
        }
    }

    fn solutions(&self, goal: &[char]) -> usize {
        if goal.is_empty() {
            return 1;
        }
        if let Some(&ret) = self.cache.borrow().get(goal) {
            return ret;
        }
        let r = self
            .stripes
            .iter()
            .filter_map(|stripe| {
                if goal.starts_with(stripe) {
                    Some(self.solutions(&goal[stripe.len()..]))
                } else {
                    None
                }
            })
            .sum();
        self.cache.borrow_mut().insert(goal.to_vec(), r);
        r
    }
}

fn part1(inp: &str) -> usize {
    let m = M::parse(inp);
    m.goals.iter().filter(|s| m.solutions(s) > 0).count()
}

fn part2(inp: &str) -> usize {
    let m = M::parse(inp);
    m.goals.iter().map(|s| m.solutions(s)).sum()
}

xaoc::xaoc!(
    sample = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
);
