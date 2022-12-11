use sscanf::scanf;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn part1(inp: &str) -> String {
    let mut roots = HashSet::new();
    let mut edges = vec![];
    for line in inp.lines() {
        let (from, to) = scanf!(
            line,
            "Step {} must be finished before step {} can begin.",
            char,
            char
        )
        .unwrap();
        edges.push((from, to));
        roots.insert(from);
        roots.insert(to);
    }
    for (_, to) in edges.iter() {
        roots.remove(to);
    }
    let mut ret = vec![];
    let mut roots = BinaryHeap::from_iter(roots.into_iter().map(Reverse));
    while let Some(Reverse(n)) = roots.pop() {
        ret.push(n);
        let mut nodes = vec![];
        edges.retain(|&(from, m)| {
            if from == n {
                nodes.push(m);
                false
            } else {
                true
            }
        });
        for m in nodes {
            if !edges.iter().any(|&(_, to)| to == m) {
                roots.push(Reverse(m));
            }
        }
    }
    assert!(edges.is_empty());
    ret.into_iter().collect()
}

fn part2(inp: &str) -> usize {
    let mut roots = HashSet::new();
    let mut edges = vec![];
    for line in inp.lines() {
        let (from, to) = scanf!(
            line,
            "Step {} must be finished before step {} can begin.",
            char,
            char
        )
        .unwrap();
        edges.push((from, to));
        roots.insert(from);
        roots.insert(to);
    }
    let sample = edges.len() < 20;
    for (_, to) in edges.iter() {
        roots.remove(to);
    }
    let mut ret = vec![];
    let mut roots = BinaryHeap::from_iter(roots.into_iter().map(Reverse));
    let mut workers = if sample { 2 } else { 5 };
    let mut schedule = BinaryHeap::<Reverse<(usize, char)>>::new();
    let mut time = 0;
    loop {
        while workers > 0 {
            match roots.pop() {
                Some(Reverse(n)) => {
                    // if sample {
                    //     eprintln!("scheduling {n} at {time}");
                    // }
                    workers -= 1;
                    let completion =
                        time + (n as u8 - 65) as usize + 1 + if sample { 0 } else { 60 };
                    schedule.push(Reverse((completion, n)));
                }
                None => break,
            }
        }
        let Some(Reverse((now, n))) = schedule.pop() else { break };
        time = now;
        workers += 1;
        ret.push(n);
        let mut nodes = vec![];
        edges.retain(|&(from, m)| {
            if from == n {
                nodes.push(m);
                false
            } else {
                true
            }
        });
        for m in nodes {
            if !edges.iter().any(|&(_, to)| to == m) {
                roots.push(Reverse(m));
            }
        }
    }
    assert!(edges.is_empty());
    time
}

xaoc::xaoc!();
