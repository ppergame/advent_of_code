use bitmaps::Bitmap;
use itertools::Itertools;
use pathfinding::prelude::*;
use petgraph::algo::floyd_warshall;
use petgraph::prelude::*;
use sscanf::scanf;
use std::collections::HashMap;

#[derive(Default)]
struct ValveNames {
    names: Vec<String>,
    name_indices: HashMap<String, usize>,
}

impl ValveNames {
    #[allow(dead_code)]
    fn name(&self, index: usize) -> &str {
        &self.names[index]
    }

    fn get_mut(&mut self, name: &str) -> usize {
        if let Some(&ret) = self.name_indices.get(name) {
            return ret;
        }
        let ret = self.names.len();
        self.names.push(name.to_string());
        self.name_indices.insert(name.to_string(), ret);
        ret
    }
}

struct Map {
    vn: ValveNames,
    flows: HashMap<usize, i64>,
    dist: HashMap<(usize, usize), usize>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut vn = ValveNames::default();
        let mut flows = HashMap::new();
        let mut graph = DiGraphMap::new();
        for line in inp.lines() {
            let (cur, flow, _, lead) = scanf!(
                line,
                "Valve {} has flow rate={}; {} to valv{}",
                str,
                i64,
                str,
                str
            )
            .unwrap();
            let cur = vn.get_mut(cur);
            let (_, lead) = lead.split_once(' ').unwrap();
            if flow > 0 {
                assert!(cur < 64);
                flows.insert(cur, flow);
            }
            for name in lead.split(", ") {
                graph.add_edge(cur, vn.get_mut(name), 1);
            }
        }
        let dist = floyd_warshall(&graph, |_| 1).unwrap();
        Map {
            vn,
            flows,
            dist: dist.into_iter().collect(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    left: usize,
    at: usize,
    flow: i64,
    valves: Bitmap<64>,
}

impl State {
    fn new(at: usize, left: usize) -> Self {
        Self {
            left,
            at,
            flow: 0,
            valves: Bitmap::new(),
        }
    }

    fn succ(&self, map: &Map) -> Vec<Self> {
        let mut ret = vec![];
        // let mut next = self.clone();
        // next.advance(self.left);
        // if !self.is_elephant && part2 {
        //     next.is_elephant = true;
        //     next.left = 26;
        //     next.flow_per_minute = 0;
        //     next.at = map.vn.get("AA");
        // } else {
        //     next.remaining_valves = Bitmap::new();
        // }
        // ret.push(next);
        for &valve in map.flows.keys() {
            if self.valves.get(valve) {
                continue;
            }
            let dist = map.dist[&(self.at, valve)];
            if dist + 1 > self.left {
                continue;
            }
            let mut next = self.clone();
            next.valves.set(valve, true);
            next.left -= dist + 1;
            next.flow += next.left as i64 * map.flows[&valve];
            next.at = valve;
            ret.push(next);
        }
        ret
    }

    #[allow(dead_code)]
    fn to_string(&self, map: &Map) -> String {
        format!(
            "State {{ left: {}, at: {}, flow: {}, valves: {:?} }}",
            self.left,
            map.vn.name(self.at),
            self.flow,
            self.valves
                .into_iter()
                .map(|v| map.vn.name(v))
                .sorted()
                .join(" ")
        )
    }
}

fn part1(inp: &str) -> i64 {
    let mut map = Map::parse(inp);
    let start = State::new(map.vn.get_mut("AA"), 30);
    dfs_reach(start, |state| state.succ(&map))
        .map(|state| state.flow)
        .max()
        .unwrap()
}

fn part2(inp: &str) -> i64 {
    let mut map = Map::parse(inp);
    let start = State::new(map.vn.get_mut("AA"), 26);
    let mut best = HashMap::<Bitmap<64>, i64>::new();
    for state in dfs_reach(start, |state| state.succ(&map)) {
        let a = best.entry(state.valves).or_default();
        *a = (*a).max(state.flow);
    }
    let mut count = 0;
    let max = best
        .iter()
        .tuple_combinations()
        .filter_map(|((best_rem1, best_flow1), (best_rem2, best_flow2))| {
            if (*best_rem1 & *best_rem2).into_value() == 0 {
                count += 1;
                Some(best_flow1 + best_flow2)
            } else {
                None
            }
        })
        .max()
        .unwrap();
    max
}

// xaoc::xaoc!(sample_idx = 1);
xaoc::xaoc!(no_sample = true);
