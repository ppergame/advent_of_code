#![allow(clippy::needless_range_loop)]

use lru_cache::LruCache;
use sscanf::scanf;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

// TODO:
// model with good_lp (cbc, highs)

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    id: i32,
    cost: [[i32; 3]; 4],
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        let (id, ore_bot_ore, clay_bot_ore, obs_bot_ore, obs_bot_clay, geo_bot_ore, geo_bot_obs) = scanf!(line,
            "Blueprint {i32}: Each ore robot costs {i32} ore. Each clay robot costs {i32} ore. Each obsidian robot costs {i32} ore and {i32} clay. Each geode robot costs {i32} ore and {i32} obsidian.").unwrap();
        Blueprint {
            id,
            cost: [
                [ore_bot_ore, 0, 0],
                [clay_bot_ore, 0, 0],
                [obs_bot_ore, obs_bot_clay, 0],
                [geo_bot_ore, 0, geo_bot_obs],
            ],
        }
    }

    fn max_cost(&self, i: usize) -> i32 {
        self.cost.iter().map(|cost| cost[i]).max().unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    left: i32,
    resources: [i32; 3],
    bots: [i32; 3],
    score: i32,
    next_purchase: u8,
}

impl State {
    fn initial(left: i32) -> Vec<Self> {
        (0..3)
            .map(|next_purchase| State {
                left,
                resources: [0, 0, 0],
                bots: [1, 0, 0],
                score: 0,
                next_purchase,
            })
            .collect()
    }

    fn can_afford(&self, bp: &Blueprint, i: usize) -> bool {
        bp.cost[i]
            .iter()
            .enumerate()
            .all(|(i, &cost)| cost <= self.resources[i])
    }

    fn succ(&self, bp: &Blueprint, best: i32) -> Vec<Self> {
        if self.left == 0 {
            return vec![];
        }
        let mut potential_score = self.score;
        for left in 0..self.left {
            potential_score += left;
        }
        if potential_score < best {
            return vec![];
        }

        let next_purchase = self.next_purchase as usize;
        let mut next = *self;
        let mut done = false;
        while !done {
            if next.left == 0 {
                return vec![];
            }
            next.left -= 1;
            done = next.can_afford(bp, next_purchase);
            for i in 0..3 {
                next.resources[i] += next.bots[i];
            }
        }

        for i in 0..3 {
            next.resources[i] -= bp.cost[next_purchase][i];
        }
        if self.next_purchase == 3 {
            next.score += next.left;
        } else {
            next.bots[next_purchase] += 1;
        }

        for i in 0..3 {
            next.resources[i] = next.resources[i].min(next.left * bp.max_cost(i));
        }

        let mut ret = vec![];
        for i in 0..4 {
            if i < 3 && next.left * bp.max_cost(i) <= (next.resources[i] + next.bots[i] * next.left)
            {
                continue;
            }
            let mut next = next;
            next.next_purchase = i as u8;
            ret.push(next);
        }
        ret
    }
}

fn run(bp: &Blueprint, left: i32) -> i32 {
    let mut best = 0;
    // let mut count = 0;
    // let mut hits = 0;
    let mut seen = LruCache::new(10000000);
    let mut queue = VecDeque::new();
    queue.extend(State::initial(left));
    while let Some(state) = queue.pop_front() {
        if seen.contains_key(&state) {
            // hits += 1;
            continue;
        }
        seen.insert(state, ());
        // count += 1;
        best = best.max(state.score);
        for next in state.succ(bp, best) {
            queue.push_back(next);
        }
    }
    // eprintln!("bp {} best {best} count {count} hits {hits}", bp.id);
    best
}

#[allow(dead_code)]
fn run_lp(bp: &Blueprint, left: i32) -> i32 {
    use good_lp::*;
    // variables
    //   buy ore/obs/clay/geo bot at time 0..max
    //     between 0 and 1
    // constraints
    //   sum of variables for each time is between 0 and 1
    //   ore/obs/clay at beginning of each time is greater or equal to spend
    // expressions
    //   score
    //     for each geo, sum of geo * time left
    //   ore/obs/clay spend during each time
    //     bots purchased
    //   ore/obs/clay income at the end of each time
    //     sum of bot vars from each previous time
    //       plus 1 for starting ore bot
    //   ore/obs/clay at beginning of each time
    //     previous time's value plus income from previous time minus previous round's spend
    let mut vars = ProblemVariables::new();
    let bots = (1..=left)
        .map(|t| {
            (
                t,
                ["ore_bot", "obs_bot", "clay_bot", "geo_bot"]
                    .iter()
                    .map(|name| {
                        vars.add(
                            VariableDefinition::new()
                                .binary()
                                .name(format!("{}_{}", name, t)),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut score = Expression::from(0);
    for (t, bots_built) in &bots {
        score += bots_built[3] * (*t - 1) as f64;
    }

    let mut spend = HashMap::new();
    for t in 1..=left {
        spend.insert(
            t,
            (0..3)
                .map(|resource_type| {
                    bp.cost
                        .iter()
                        .enumerate()
                        .map(|(bot_type, cost)| bots[&t][bot_type] * cost[resource_type])
                        .sum::<Expression>()
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut income = HashMap::new();
    for t in 1..=left {
        income.insert(
            t,
            (0..3)
                .map(|resource_type| {
                    let mut v = (t + 1..=left)
                        .map(|st| bots[&st][resource_type])
                        .sum::<Expression>();
                    if resource_type == 0 {
                        v += Expression::from(1);
                    }
                    v
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut resources = HashMap::<i32, Vec<Expression>>::new();
    resources.insert(left, std::iter::repeat(0.into()).take(4).collect());
    for t in (1..left).rev() {
        resources.insert(
            t,
            (0..3)
                .map(|resource_type| {
                    resources[&(t + 1)][resource_type].clone()
                        + income[&(t + 1)][resource_type].clone()
                        - spend[&(t + 1)][resource_type].clone()
                })
                .collect(),
        );
    }

    let mut prob = vars.maximise(&score).using(default_solver);
    // prob.set_parameter("log", "0");
    for bots_built in bots.values() {
        prob = prob.with(bots_built.iter().sum::<Expression>().leq(1));
    }
    for t in 1..=left {
        for i in 0..3 {
            prob = prob.with(resources[&t][i].clone().geq(spend[&t][i].clone()));
        }
    }

    let sol = prob.solve().unwrap();

    // let mut bots_built = [1, 0, 0, 0];
    // let mut resources = [0; 4];
    // let mut left = left;
    // while left > 0 {
    //     eprintln!("at left {left}");
    //     eprintln!("  resources {resources:?}");
    //     eprintln!("       bots {bots_built:?}");
    //     let old_bots = bots_built;
    //     for (idx, built) in bots[&left].iter().enumerate() {
    //         if sol.eval(built) > 0.9 {
    //             eprintln!("  build bot {idx}");
    //             bots_built[idx] += 1;
    //             for i in 0..3 {
    //                 resources[i] -= bp.cost[idx][i];
    //                 assert!(resources[i] >= 0);
    //             }
    //         }
    //     }
    //     eprintln!(" spent, now {resources:?}");
    //     for i in 0..4 {
    //         resources[i] += old_bots[i];
    //     }
    //     eprintln!("    accrued {resources:?}");
    //     eprintln!("       bots {bots_built:?}");
    //     left -= 1;
    // }

    // for t in bots.keys().sorted() {
    //     eprintln!("t {t} geo built {}", sol.eval(bots[t][3]));
    // }
    sol.eval(&score).round() as i32
}

fn part1(inp: &str) -> i32 {
    let bps = inp.lines().map(Blueprint::parse).collect::<Vec<_>>();
    let mut ret = 0;
    for bp in bps {
        let best = run(&bp, 24);
        // let best2 = run_lp(&bp, 24);
        // assert_eq!(best, best2, "bp {}", bp.id);
        ret += bp.id * best;
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let bps = inp.lines().map(Blueprint::parse).collect::<Vec<_>>();
    // if bps.len() < 5 {
    //     return 0;
    // }
    let mut ret = 1i64;
    for bp in bps.iter().take(3) {
        let best = run(bp, 32);
        // let best2 = run_lp(bp, 32);
        // assert_eq!(best, best2, "bp {}", bp.id);
        ret *= best as i64;
    }
    ret
}

xaoc::xaoc!(
    sample = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#
);

// xaoc::xaoc!(no_sample = true);
