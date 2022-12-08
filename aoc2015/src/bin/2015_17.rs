use itertools::Itertools;
use std::collections::HashMap;

fn parse(inp: &str) -> Vec<usize> {
    inp.lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect()
}

fn part1(inp: &str) -> usize {
    let cc = parse(inp);
    let goal = 150;
    let mut memo = HashMap::new();
    let mut work = vec![(goal, 0)];
    while let Some((target, offset)) = work.pop() {
        if target == 0 {
            memo.insert((target, offset), 1);
            continue;
        }
        if offset == cc.len() {
            memo.insert((target, offset), 0);
            continue;
        }
        let c = cc[offset];
        let key = (target, offset + 1);
        let mut queue = vec![(key, memo.get(&key).cloned())];
        if c <= target {
            let key = (target - c, offset + 1);
            queue.push((key, memo.get(&key).cloned()));
        }
        let missing = queue
            .iter()
            .filter_map(|(k, v)| v.map_or(Some(k), |_| None))
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            work.push((target, offset));
            work.extend(missing);
        } else {
            memo.insert(
                (target, offset),
                queue.iter().map(|(_, v)| v.unwrap()).sum(),
            );
        }
    }
    memo[&(goal, 0)]
}

fn part2(inp: &str) -> i64 {
    let cc = parse(inp);
    for num_containers in 1..cc.len() {
        let goal = 150;
        let mut memo = HashMap::<(usize, usize, usize), i64>::new();
        let mut work = vec![(goal, num_containers, 0)];
        while let Some((target, budget, offset)) = work.pop() {
            if target == 0 {
                memo.insert((target, budget, offset), (budget == 0) as i64);
                continue;
            }
            if offset == cc.len() || budget == 0 {
                memo.insert((target, budget, offset), 0);
                continue;
            }
            let c = cc[offset];
            let key = (target, budget, offset + 1);
            let mut queue = vec![(key, memo.get(&key).cloned())];
            if c <= target {
                let key = (target - c, budget - 1, offset + 1);
                queue.push((key, memo.get(&key).cloned()));
            }
            let missing = queue
                .iter()
                .filter_map(|(k, v)| v.map_or(Some(k), |_| None))
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                work.push((target, budget, offset));
                work.extend(missing);
            } else {
                memo.insert(
                    (target, budget, offset),
                    queue.iter().map(|(_, v)| v.unwrap()).sum(),
                );
            }
        }
        let res = memo[&(goal, num_containers, 0)];
        if res > 0 {
            return res;
        }
    }
    unreachable!();
}

xaoc::xaoc!();
