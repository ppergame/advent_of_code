use hashbrown::HashSet;
use sscanf::scanf;

#[derive(Debug)]
struct Input {
    rules: HashSet<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

impl Input {
    fn parse(inp: &str) -> Self {
        let mut it = inp.lines();
        let mut rules = HashSet::new();
        for line in &mut it {
            if line.is_empty() {
                break;
            }
            let (from, to) = scanf!(line, "{}|{}", i64, i64).unwrap();
            rules.insert((from, to));
        }
        let updates = it
            .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();
        Self { rules, updates }
    }

    fn sorted(&self, up: &[i64]) -> Vec<i64> {
        let mut upd = up.to_vec();
        upd.sort_by(|a, b| {
            if a == b {
                panic!("dupe {a}");
            }
            if self.rules.contains(&(*a, *b)) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        upd
    }
}

fn part1(inp: &str) -> i64 {
    let inp = Input::parse(inp);
    let mut ret = 0;
    for up in &inp.updates {
        if up == &inp.sorted(up) {
            ret += up[up.len() / 2];
        }
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let inp = Input::parse(inp);
    let mut ret = 0;
    for up in &inp.updates {
        let upd = inp.sorted(up);
        if *up != upd {
            ret += upd[upd.len() / 2];
        }
    }
    ret
}

xaoc::xaoc!(
    sample = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
);
