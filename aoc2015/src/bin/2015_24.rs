use std::collections::{HashMap, HashSet};

struct AddUpTo {
    pp: Vec<usize>,
    memo: HashMap<(usize, usize), HashSet<Vec<usize>>>,
}

impl AddUpTo {
    fn new(pp: Vec<usize>) -> Self {
        Self {
            pp,
            memo: HashMap::new(),
        }
    }

    fn entry(&mut self, goal: usize, offset: usize) -> &mut HashSet<Vec<usize>> {
        self.memo.entry((goal, offset)).or_default()
    }

    fn calc(&mut self, goal: usize, offset: usize) {
        if self.memo.contains_key(&(goal, offset)) {
            return;
        }
        if offset == self.pp.len() || goal == 0 {
            let entry = self.entry(goal, offset);
            if goal == 0 {
                entry.insert(vec![]);
            }
            return;
        }

        // skip
        self.calc(goal, offset + 1);
        let reckey = (goal, offset + 1);
        let recval = self.memo.remove(&reckey).unwrap();
        let entry = self.entry(goal, offset);
        for v in &recval {
            entry.insert(v.clone());
        }
        self.memo.insert(reckey, recval);

        // use
        let next = self.pp[offset];
        if goal >= next {
            self.calc(goal - next, offset + 1);
            let reckey = (goal - next, offset + 1);
            let recval = self.memo.remove(&reckey).unwrap();
            let entry = self.entry(goal, offset);
            for v in &recval {
                let mut v = v.clone();
                v.push(next);
                entry.insert(v);
            }
            self.memo.insert(reckey, recval);
        }
    }
}

fn part1(inp: &str) -> usize {
    let pp = inp
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let goal = pp.iter().sum::<usize>() / 3;
    let mut aut = AddUpTo::new(pp);
    aut.calc(goal, 0);
    let aa = &aut.memo[&(goal, 0)];
    let len = aa.iter().map(|v| v.len()).min().unwrap();
    aa.iter()
        .filter_map(|v| {
            if v.len() != len {
                None
            } else {
                Some(v.iter().product())
            }
        })
        .min()
        .unwrap()
}

fn part2(inp: &str) -> usize {
    let pp = inp
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let goal = pp.iter().sum::<usize>() / 4;
    let mut aut = AddUpTo::new(pp);
    aut.calc(goal, 0);
    let aa = &aut.memo[&(goal, 0)];
    let len = aa.iter().map(|v| v.len()).min().unwrap();
    aa.iter()
        .filter_map(|v| {
            if v.len() != len {
                None
            } else {
                Some(v.iter().product())
            }
        })
        .min()
        .unwrap()
}

xaoc::xaoc!();
