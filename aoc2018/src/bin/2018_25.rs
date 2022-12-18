use itertools::Itertools;
use pathfinding::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord(i64, i64, i64, i64);

impl Coord {
    fn parse(s: &str) -> Self {
        let sp = s
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Coord(sp[0], sp[1], sp[2], sp[3])
    }

    fn dist(&self, other: &Coord) -> i64 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

fn part1(inp: &str) -> usize {
    let points = inp.lines().map(Coord::parse).collect::<Vec<_>>();
    let mut neigh = HashMap::<Coord, Vec<Coord>>::new();
    for (a, b) in points.iter().tuple_combinations() {
        if a.dist(b) <= 3 {
            neigh.entry(*a).or_default().push(*b);
            neigh.entry(*b).or_default().push(*a);
        }
    }
    connected_components(&points, |a| neigh.get(a).cloned().unwrap_or_default()).len()
}

fn part2(_inp: &str) -> i64 {
    0
}

//xaoc::xaoc!(sample_idx = 0);
//xaoc::xaoc!(sample_idx = 13);
// xaoc::xaoc!(sample_idx=15);
xaoc::xaoc!(sample_idx = 17);
