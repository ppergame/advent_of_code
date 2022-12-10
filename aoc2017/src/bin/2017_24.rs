use itertools::Itertools;
use sscanf::scanf;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Copy, Clone)]
struct Info {
    strongest: usize,
    longest: usize,
    longest_strength: usize,
}

impl Info {
    fn merge(&mut self, mut other: Info, start: usize, end: usize) {
        other.strongest += start + end;
        other.longest += 1;
        other.longest_strength += start + end;
        self.strongest = self.strongest.max(other.strongest);
        match self.longest.cmp(&other.longest) {
            std::cmp::Ordering::Less => {
                self.longest = other.longest;
                self.longest_strength = other.longest_strength;
            }
            std::cmp::Ordering::Equal => {
                self.longest_strength = self.longest_strength.max(other.longest_strength)
            }
            std::cmp::Ordering::Greater => (),
        }
    }
}

#[derive(Default)]
struct State {
    memo: HashMap<(usize, Vec<(usize, usize)>), Info>,
}

impl State {
    fn best(&mut self, plug: usize, remaining: Vec<(usize, usize)>) -> Info {
        if let Some(&res) = self.memo.get(&(plug, remaining.clone())) {
            return res;
        }

        let mut best = Info::default();
        for (idx, &item) in remaining.iter().enumerate() {
            let (start, end) = item;
            if start == plug || end == plug {
                let mut rem = remaining.clone();
                rem.remove(idx);
                let next_plug = if start == plug { end } else { start };
                best.merge(self.best(next_plug, rem), start, end);
            }
        }
        self.memo.insert((plug, remaining), best);
        best
    }
}

fn part1(inp: &str) -> usize {
    let ports = inp
        .lines()
        .map(|line| scanf!(line, "{}/{}", usize, usize).unwrap())
        .sorted()
        .collect_vec();
    {
        let port_set = HashSet::<(usize, usize)>::from_iter(ports.iter().copied());
        assert_eq!(ports.len(), port_set.len());
    }
    let mut state = State::default();
    state.best(0, ports).strongest
}

fn part2(inp: &str) -> usize {
    let ports = inp
        .lines()
        .map(|line| scanf!(line, "{}/{}", usize, usize).unwrap())
        .sorted()
        .collect_vec();
    {
        let port_set = HashSet::<(usize, usize)>::from_iter(ports.iter().copied());
        assert_eq!(ports.len(), port_set.len());
    }
    let mut state = State::default();
    state.best(0, ports).longest_strength
}

xaoc::xaoc!(sample_idx = 8);
