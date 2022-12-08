use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Id(usize, usize);

lazy_static! {
    static ref NODE_RE: Regex = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)").unwrap();
    static ref NUM_RE: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

impl Node {
    fn id(&self) -> Id {
        Id(self.x, self.y)
    }

    fn avail(&self) -> usize {
        self.size - self.used
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    goal: Id,
    empty: Id,
}

struct Grid {
    width: usize,
    height: usize,
    walls: HashSet<Id>,
}

impl Grid {
    fn new(inp: &str) -> (Self, State) {
        let all_nodes = parse(inp);
        let width = all_nodes.iter().map(|node| node.x).max().unwrap() + 1;
        let height = all_nodes.iter().map(|node| node.y).max().unwrap() + 1;
        let mut empty = None;
        let mut capacity = None;
        for node in &all_nodes {
            if node.used == 0 {
                assert!(empty.is_none());
                empty = Some(node.id());
                capacity = Some(node.size);
            }
        }
        let empty = empty.unwrap();
        let capacity = capacity.unwrap();
        let mut walls = HashSet::new();
        for node in all_nodes {
            if node.used > capacity {
                walls.insert(node.id());
            }
        }
        (
            Self {
                width,
                height,
                walls,
            },
            State {
                goal: Id(width - 1, 0),
                empty,
            },
        )
    }

    fn adj(&self, id: Id) -> Vec<Id> {
        let mut ret = vec![];
        if id.0 > 0 {
            ret.push(Id(id.0 - 1, id.1));
        }
        if id.1 > 0 {
            ret.push(Id(id.0, id.1 - 1));
        }
        if id.0 < self.width - 1 {
            ret.push(Id(id.0 + 1, id.1));
        }
        if id.1 < self.height - 1 {
            ret.push(Id(id.0, id.1 + 1));
        }
        ret
    }

    fn succ(&self, state: &State) -> Vec<(State, usize)> {
        self.adj(state.empty)
            .iter()
            .filter_map(|&empty| {
                if self.walls.contains(&empty) {
                    None
                } else {
                    Some((
                        if empty == state.goal {
                            State {
                                goal: state.empty,
                                empty,
                            }
                        } else {
                            State {
                                goal: state.goal,
                                empty,
                            }
                        },
                        1,
                    ))
                }
            })
            .collect()
    }
}

fn parse(inp: &str) -> Vec<Node> {
    inp.lines()
        .skip(2)
        .map(|line| {
            let mut sp = line.split_whitespace();
            let caps = NODE_RE.captures(sp.next().unwrap()).unwrap();
            let x = caps.get(1).unwrap().as_str().parse().unwrap();
            let y = caps.get(2).unwrap().as_str().parse().unwrap();
            let size = NUM_RE
                .captures(sp.next().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let used = NUM_RE
                .captures(sp.next().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            Node { x, y, size, used }
        })
        .collect()
}

fn part1(inp: &str) -> usize {
    let nodes = parse(inp);
    let mut viable = 0;
    for node_a in &nodes {
        for node_b in &nodes {
            if node_a.used > 0 && node_a.id() != node_b.id() && node_a.used <= node_b.avail() {
                viable += 1;
            }
        }
    }
    viable
}

fn part2(inp: &str) -> usize {
    let (grid, state) = Grid::new(inp);
    let (_, cost) = pathfinding::directed::dijkstra::dijkstra(
        &state,
        |state| grid.succ(state),
        |state| state.goal == Id(0, 0),
    )
    .unwrap();
    cost
}

xaoc::xaoc!(no_sample = true);
