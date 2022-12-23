use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet};
use tinyvec::ArrayVec;

enum Dir {
    North,
    South,
    West,
    East,
}

struct Map {
    elves: HashSet<(i16, i16)>,
    dir_order: usize,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut elves = HashSet::new();
        for (row, line) in inp.lines().enumerate() {
            let row = row as i16;
            for (col, c) in line.chars().enumerate() {
                let col = col as i16;
                if c == '#' {
                    elves.insert((row, col));
                }
            }
        }
        Self {
            elves,
            dir_order: 0,
        }
    }

    fn step(&mut self) -> bool {
        let mut proposals =
            HashMap::<(i16, i16), ArrayVec<[(i16, i16); 4]>>::with_capacity(self.elves.len());
        'outer: for &(row, col) in &self.elves {
            let n_adj = [(row - 1, col - 1), (row - 1, col), (row - 1, col + 1)];
            let s_adj = [(row + 1, col - 1), (row + 1, col), (row + 1, col + 1)];
            let w_adj = [(row - 1, col - 1), (row, col - 1), (row + 1, col - 1)];
            let e_adj = [(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)];
            let mut found = false;
            'adj: for adj in [n_adj, s_adj, w_adj, e_adj].iter() {
                for &(arow, acol) in adj {
                    if self.elves.contains(&(arow, acol)) {
                        found = true;
                        break 'adj;
                    }
                }
            }
            if !found {
                proposals.entry((row, col)).or_default().push((row, col));
                continue;
            }
            let mut dirs = [Dir::North, Dir::South, Dir::West, Dir::East];
            dirs.rotate_left(self.dir_order);
            for dir in dirs {
                match dir {
                    Dir::North => {
                        if n_adj
                            .iter()
                            .all(|&(arow, acol)| !self.elves.contains(&(arow, acol)))
                        {
                            proposals
                                .entry((row - 1, col))
                                .or_default()
                                .push((row, col));
                            continue 'outer;
                        }
                    }
                    Dir::South => {
                        if s_adj
                            .iter()
                            .all(|&(arow, acol)| !self.elves.contains(&(arow, acol)))
                        {
                            proposals
                                .entry((row + 1, col))
                                .or_default()
                                .push((row, col));
                            continue 'outer;
                        }
                    }
                    Dir::West => {
                        if w_adj
                            .iter()
                            .all(|&(arow, acol)| !self.elves.contains(&(arow, acol)))
                        {
                            proposals
                                .entry((row, col - 1))
                                .or_default()
                                .push((row, col));
                            continue 'outer;
                        }
                    }
                    Dir::East => {
                        if e_adj
                            .iter()
                            .all(|&(arow, acol)| !self.elves.contains(&(arow, acol)))
                        {
                            proposals
                                .entry((row, col + 1))
                                .or_default()
                                .push((row, col));
                            continue 'outer;
                        }
                    }
                }
            }
            assert!(!proposals.contains_key(&(row, col)));
            proposals.entry((row, col)).or_default().push((row, col));
        }
        self.elves.clear();
        let mut any_moved = false;
        for (new_pos, old_positions) in proposals {
            if old_positions.len() == 1 {
                if new_pos != old_positions[0] {
                    any_moved = true;
                }
                self.elves.insert(new_pos);
                continue;
            }
            for pos in old_positions {
                self.elves.insert(pos);
            }
        }
        self.dir_order = (self.dir_order + 1) % 4;
        any_moved
    }

    fn count_box_empty(&self) -> i16 {
        let MinMaxResult::MinMax(min_row, max_row) = self.elves.iter().map(|&(r, _)| r).minmax() else { unreachable!() };
        let MinMaxResult::MinMax(min_col, max_col) = self.elves.iter().map(|&(_, c)| c).minmax() else { unreachable!() };
        (max_row - min_row + 1) * (max_col - min_col + 1) - self.elves.len() as i16
    }

    #[allow(dead_code)]
    fn print(&self) {
        let MinMaxResult::MinMax(min_row, max_row) = self.elves.iter().map(|&(r, _)| r).minmax() else { unreachable!() };
        let MinMaxResult::MinMax(min_col, max_col) = self.elves.iter().map(|&(_, c)| c).minmax() else { unreachable!() };
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let c = if self.elves.contains(&(row, col)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn part1(inp: &str) -> i16 {
    let mut map = Map::parse(inp);
    for _ in 0..10 {
        map.step();
    }
    map.count_box_empty()
}

fn part2(inp: &str) -> i16 {
    let mut map = Map::parse(inp);
    let mut idx = 1;
    while map.step() {
        idx += 1;
    }
    idx
}

xaoc::xaoc!();
