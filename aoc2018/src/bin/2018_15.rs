use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

// For every node in goal, returns the second (after start) element in the shortest path as well as path distance
// Explores children in order returned by successors.
pub fn bfs_set_second<N, FN, IN1, IN2>(
    start: &N,
    mut successors: FN,
    goals: IN2,
) -> HashMap<N, (N, usize)>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN1,
    IN1: IntoIterator<Item = N>,
    IN2: IntoIterator<Item = N>,
{
    let mut seen = HashSet::new();
    seen.insert(start.clone());
    let goals = HashSet::<N>::from_iter(goals);
    let mut ret = HashMap::new();
    // queue of (node, second_node, depth)
    let mut queue = VecDeque::new();
    for succ in successors(start) {
        queue.push_back((succ.clone(), succ, 1));
    }
    while let Some((node, second, depth)) = queue.pop_front() {
        if seen.contains(&node) {
            continue;
        }
        seen.insert(node.clone());
        if goals.contains(&node) {
            ret.insert(node.clone(), (second.clone(), depth));
            if ret.len() == goals.len() {
                return ret;
            }
        }
        for succ in successors(&node) {
            queue.push_back((succ, second.clone(), depth + 1));
        }
    }
    ret
}

#[derive(Copy, Clone)]
struct Unit {
    side: char,
    hp: usize,
}

fn adj(row: usize, col: usize) -> Vec<(usize, usize)> {
    vec![
        (row - 1, col),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col),
    ]
}

#[derive(Clone)]
struct Map {
    map: HashSet<(usize, usize)>,
    units: HashMap<(usize, usize), Unit>,
    elf_power: usize,
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        let MinMaxResult::MinMax(minr, maxr) =
            self.map.iter().map(|(row, _)| row).copied().minmax()
        else {
            unreachable!()
        };
        let MinMaxResult::MinMax(minc, maxc) =
            self.map.iter().map(|(_, col)| col).copied().minmax()
        else {
            unreachable!()
        };
        for row in minr..=maxr {
            let mut uline = vec![];
            for col in minc..=maxc {
                if let Some(unit) = self.units.get(&(row, col)) {
                    print!("{}", unit.side);
                    uline.push(format!("{}({})", unit.side, unit.hp));
                } else if self.map.contains(&(row, col)) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            print!("   {}", uline.into_iter().join(", "));
            println!();
        }
    }

    fn parse(inp: &str) -> Self {
        let mut map = HashSet::new();
        let mut units = HashMap::new();
        for (row, line) in inp.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => (),
                    '.' => {
                        map.insert((row, col));
                    }
                    'G' | 'E' => {
                        map.insert((row, col));
                        units.insert((row, col), Unit { side: c, hp: 200 });
                    }
                    _ => unreachable!("bad char {c:?}"),
                }
            }
        }
        Map {
            map,
            units,
            elf_power: 3,
        }
    }

    fn round(&mut self) -> bool {
        let move_order = self.units.keys().copied().sorted().collect_vec();
        for (mut row, mut col) in move_order {
            // eprintln!("moving {row} {col}");
            let Some(my_unit) = self.units.get(&(row, col)).cloned() else {
                continue;
            };
            if !self.units.contains_key(&(row, col)) {
                continue;
            }
            // move phase
            if self.target(row, col, &my_unit).is_none() {
                let Some(target_adj) = self.all_target_adj(&my_unit) else {
                    return false;
                };
                if let Some((_, (next, _))) =
                    bfs_set_second(&(row, col), |&(row, col)| self.succ(row, col), target_adj)
                        .into_iter()
                        .sorted_by_key(|&(target, (next, cost))| (cost, target, next))
                        .next()
                {
                    self.units.remove(&(row, col));
                    (row, col) = next;
                    self.units.insert((row, col), my_unit);
                }
            }
            if let Some((_, (trow, tcol))) = self.target(row, col, &my_unit) {
                // attack phase
                let std::collections::hash_map::Entry::Occupied(mut o) =
                    self.units.entry((trow, tcol))
                else {
                    unreachable!()
                };
                let unit = o.get_mut();
                let power = if my_unit.side == 'E' {
                    self.elf_power
                } else {
                    3
                };
                unit.hp = unit.hp.saturating_sub(power);
                if unit.hp == 0 {
                    o.remove();
                }
            }
        }
        true
    }

    fn target(&self, row: usize, col: usize, my_unit: &Unit) -> Option<(usize, (usize, usize))> {
        adj(row, col)
            .into_iter()
            .filter_map(|(row, col)| {
                if let Some(unit) = self.units.get(&(row, col)) {
                    if unit.side == my_unit.side {
                        None
                    } else {
                        Some((unit.hp, (row, col)))
                    }
                } else {
                    None
                }
            })
            .sorted()
            .next()
    }

    fn all_target_adj(&self, my_unit: &Unit) -> Option<HashSet<(usize, usize)>> {
        let mut found_enemies = false;
        let mut ret = HashSet::new();
        for (&(trow, tcol), unit) in &self.units {
            if unit.side == my_unit.side {
                continue;
            }
            found_enemies = true;
            ret.extend(self.succ(trow, tcol));
        }
        if found_enemies {
            Some(ret)
        } else {
            None
        }
    }

    fn succ(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        adj(row, col).into_iter().filter(|&(arow, acol)| {
            !self.units.contains_key(&(arow, acol)) && self.map.contains(&(arow, acol))
        })
    }

    fn elf_count(&self) -> usize {
        self.units.values().filter(|unit| unit.side == 'E').count()
    }
}

fn part1(inp: &str) -> usize {
    let mut map = Map::parse(inp);
    let mut round = 0;
    while map.round() {
        round += 1;
        // println!("After {round} round:");
        // map.print();
        // println!();
    }
    round * map.units.into_values().map(|unit| unit.hp).sum::<usize>()
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    let start_elf_count = map.elf_count();
    for power in 4..100 {
        let mut map = map.clone();
        map.elf_power = power;
        let mut round = 0;
        while map.round() {
            round += 1;
            // println!("After {round} round:");
            // map.print();
            // println!();
        }
        if map.elf_count() == start_elf_count {
            return round * map.units.into_values().map(|unit| unit.hp).sum::<usize>();
        }
    }
    unreachable!();
}

xaoc::xaoc!(
    sample = r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#
);
