use pathfinding::directed::dijkstra::dijkstra;
use regex::Regex;
use std::collections::btree_map::Entry::Occupied;
use std::collections::{BTreeMap, HashMap};

lazy_static::lazy_static! {
    static ref GEN_RE: Regex = Regex::new(r"(\w+) generator").unwrap();
    static ref CHIP_RE: Regex = Regex::new(r"(\w+)-compatible microchip").unwrap();
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
enum Item {
    Gen(usize),
    Chip(usize),
}

impl Item {
    fn is_gen(&self) -> bool {
        matches!(self, Item::Gen(_))
    }

    fn is_chip(&self) -> bool {
        matches!(self, Item::Chip(_))
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct Floor {
    pairs: usize,
    items: BTreeMap<Item, usize>,
}

impl Floor {
    fn insert(&mut self, item: Item) {
        *self.items.entry(item).or_default() += 1;
    }

    fn remove(&mut self, item: Item) {
        let Occupied(mut o) = self.items.entry(item) else { panic!(); };
        let val = o.get_mut();
        match *val {
            0 => unreachable!(),
            1 => {
                o.remove();
            }
            _ => *val -= 1,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    floors: Vec<Floor>,
    elevator: usize,
}

impl State {
    fn move_one(&self, dest_floor: usize) -> Vec<State> {
        let mut ret = vec![];
        for item in self.floors[self.elevator].items.keys() {
            let mut next = self.clone();
            next.floors[self.elevator].remove(*item);
            match *item {
                Item::Gen(other_floor) => {
                    next.floors[other_floor].remove(Item::Chip(self.elevator));
                    if other_floor == dest_floor {
                        next.floors[dest_floor].pairs += 1;
                    } else {
                        next.floors[other_floor].insert(Item::Chip(dest_floor));
                        next.floors[dest_floor].insert(Item::Gen(other_floor));
                    }
                }
                Item::Chip(other_floor) => {
                    next.floors[other_floor].remove(Item::Gen(self.elevator));
                    if other_floor == dest_floor {
                        next.floors[dest_floor].pairs += 1;
                    } else {
                        next.floors[other_floor].insert(Item::Gen(dest_floor));
                        next.floors[dest_floor].insert(Item::Chip(other_floor));
                    }
                }
            }
            ret.push(next);
        }
        if self.floors[self.elevator].pairs > 0 {
            let mut next = self.clone();
            next.floors[self.elevator].pairs -= 1;
            next.floors[self.elevator].insert(Item::Gen(dest_floor));
            next.floors[dest_floor].insert(Item::Chip(self.elevator));
            ret.push(next);
            let mut next = self.clone();
            next.floors[self.elevator].pairs -= 1;
            next.floors[self.elevator].insert(Item::Chip(dest_floor));
            next.floors[dest_floor].insert(Item::Gen(self.elevator));
            ret.push(next);
        }
        ret
    }

    fn succ(&self) -> Vec<(State, i64)> {
        let mut ret = vec![];
        let mut dest_floors = vec![];
        if self.elevator > 0 {
            dest_floors.push(self.elevator - 1);
        }
        if self.elevator < self.floors.len() - 1 {
            dest_floors.push(self.elevator + 1);
        }
        for dest_floor in dest_floors {
            let moved_one = self.move_one(dest_floor);
            for mut next in moved_one {
                let moved_two = next.move_one(dest_floor);
                for mut next in moved_two {
                    next.elevator = dest_floor;
                    if next.check() {
                        ret.push((next, 1));
                    }
                }
                next.elevator = dest_floor;
                if next.check() {
                    ret.push((next, 1));
                }
            }
        }
        ret
    }

    fn check(&self) -> bool {
        for floor in &self.floors {
            let has_gens = floor.pairs > 0 || floor.items.keys().any(|item| item.is_gen());
            for item in floor.items.keys() {
                if item.is_chip() && has_gens {
                    return false;
                }
            }
        }
        true
    }

    fn goal(&self, num: usize) -> bool {
        self.floors.last().unwrap().pairs == num
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            floors: vec![Default::default(); 4],
            elevator: 0,
        }
    }
}

fn part1(inp: &str) -> i64 {
    let mut gens = HashMap::new();
    let mut chips = HashMap::new();
    for (floor, line) in inp.lines().enumerate() {
        gens.extend(
            GEN_RE
                .captures_iter(line)
                .map(|cap| (cap.get(1).unwrap().as_str(), floor)),
        );
        chips.extend(
            CHIP_RE
                .captures_iter(line)
                .map(|cap| (cap.get(1).unwrap().as_str(), floor)),
        );
    }
    let pairs = gens.len();
    let mut init = State::default();
    for (gen, floor) in gens {
        let floor2 = chips.remove(gen).unwrap();
        if floor == floor2 {
            init.floors[floor].pairs += 1;
        } else {
            init.floors[floor].insert(Item::Gen(floor2));
            init.floors[floor2].insert(Item::Chip(floor));
        }
    }
    assert!(chips.is_empty());
    let (_, cost) = dijkstra(&init, |state| state.succ(), |state| state.goal(pairs)).unwrap();
    cost
}

fn part2(inp: &str) -> i64 {
    let mut gens = HashMap::new();
    let mut chips = HashMap::new();
    for (floor, line) in inp.lines().enumerate() {
        gens.extend(
            GEN_RE
                .captures_iter(line)
                .map(|cap| (cap.get(1).unwrap().as_str(), floor)),
        );
        chips.extend(
            CHIP_RE
                .captures_iter(line)
                .map(|cap| (cap.get(1).unwrap().as_str(), floor)),
        );
    }
    let mut pairs = gens.len();
    let mut init = State::default();
    for (gen, floor) in gens {
        let floor2 = chips.remove(gen).unwrap();
        if floor == floor2 {
            init.floors[floor].pairs += 1;
        } else {
            init.floors[floor].insert(Item::Gen(floor2));
            init.floors[floor2].insert(Item::Chip(floor));
        }
    }
    assert!(chips.is_empty());
    init.floors[0].pairs += 2;
    pairs += 2;
    let (_, cost) = dijkstra(&init, |state| state.succ(), |state| state.goal(pairs)).unwrap();
    cost
}

xaoc::xaoc!();
