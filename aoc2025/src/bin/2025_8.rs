use hashbrown::{HashMap, HashSet};
use itertools::Itertools as _;
use pathfinding::prelude::*;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Box(i64, i64, i64);

impl Box {
    fn parse(l: &str) -> Self {
        let (a, b, c) = sscanf::scanf!(l, "{i64},{i64},{i64}").unwrap();
        Self(a, b, c)
    }

    fn dist(&self, other: &Self) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

fn part1(inp: &str) -> usize {
    let boxes = inp.lines().map(Box::parse).collect::<Vec<_>>();
    let mut conns = HashMap::<Box, HashSet<Box>>::new();

    let count = if boxes.len() < 30 { 10 } else { 1000 };
    for (bx1, bx2) in boxes
        .iter()
        .cloned()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| a.dist(b))
        .take(count)
    {
        conns.entry(bx1).or_default().insert(bx2);
        conns.entry(bx2).or_default().insert(bx1);
    }
    let comps = connected_components(&boxes, |n| conns.get(n).into_iter().flatten().cloned());
    comps.into_iter().map(|x| x.len()).k_largest(3).product()
}

struct Dsu {
    parent: Vec<usize>,
    comps: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            comps: n,
        }
    }

    fn find(&mut self, mut i: usize) -> usize {
        while i != self.parent[i] {
            self.parent[i] = self.parent[self.parent[i]];
            i = self.parent[i];
        }
        i
    }

    fn union(&mut self, i: usize, j: usize) {
        let (i, j) = (self.find(i), self.find(j));
        if i == j {
            return;
        }
        self.parent[i] = j;
        self.comps -= 1;
    }
}

fn part2(inp: &str) -> i64 {
    let boxes = inp.lines().map(Box::parse).collect::<Vec<_>>();
    let mut dsu = Dsu::new(boxes.len());

    for (i, j) in (0..boxes.len())
        .tuple_combinations()
        .sorted_by_key(|&(i, j)| boxes[i].dist(&boxes[j]))
    {
        dsu.union(i, j);
        if dsu.comps == 1 {
            return boxes[i].0 * boxes[j].0;
        }
    }
    unreachable!();
}

xaoc::xaoc!();
