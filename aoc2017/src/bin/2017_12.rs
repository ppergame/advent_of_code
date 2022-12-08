use itertools::Itertools;
use pathfinding::prelude::*;
use sscanf::scanf;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let mut can_talk_to = HashMap::<usize, Vec<usize>>::new();
    for line in inp.lines() {
        let (left, right) = scanf!(line, "{} <-> {}", usize, str).unwrap();
        let others = right
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        can_talk_to.insert(left, others);
    }
    strongly_connected_component(&0, |prog| can_talk_to[prog].clone()).len()
}

fn part2(inp: &str) -> usize {
    let mut can_talk_to = HashMap::<usize, Vec<usize>>::new();
    for line in inp.lines() {
        let (left, right) = scanf!(line, "{} <-> {}", usize, str).unwrap();
        let others = right
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        can_talk_to.insert(left, others);
    }
    strongly_connected_components(&can_talk_to.keys().copied().collect_vec(), |prog| {
        can_talk_to[prog].clone()
    })
    .len()
}

xaoc::xaoc!(sample_idx = 5);
