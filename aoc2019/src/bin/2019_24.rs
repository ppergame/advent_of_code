use std::collections::{HashMap, HashSet};

type GridEasy = [bool; 25];

trait GridEasyMethods {
    fn step(&self) -> Self;
    fn ncount(&self, i: usize) -> usize;
    fn show(&self);
}

impl GridEasyMethods for GridEasy {
    fn step(&self) -> Self {
        let mut new_grid: GridEasy = [false; 25];
        for i in 0..self.len() {
            let nc = self.ncount(i);
            match self[i] {
                true => new_grid[i] = nc == 1,
                false => new_grid[i] = nc == 1 || nc == 2,
            }
        }
        new_grid
    }

    fn ncount(&self, i: usize) -> usize {
        let mut ret = 0;
        // i - 1 and i + 1 are wrong here
        let mut indices = vec![i - 5, i + 5];
        if i % 5 != 0 {
            indices.push(i - 1);
        }
        if i % 5 != 4 {
            indices.push(i + 1);
        }
        for index in indices {
            if *self.get(index).unwrap_or(&false) {
                ret += 1;
            }
        }
        ret
    }

    fn show(&self) {
        for i in 0..5 {
            for j in 0..5 {
                print!("{}", if self[i * 5 + j] { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }
}

fn part1(inp: &str) -> i32 {
    let mut grid: GridEasy = [false; 25];
    let mut i = 0;
    for b in inp.chars() {
        match b {
            '\n' => continue,
            '.' => grid[i] = false,
            '#' => grid[i] = true,
            _ => panic!("unknown map char {}", b as u8),
        }
        i += 1;
    }

    let mut grids = HashSet::<GridEasy>::new();
    loop {
        //grid.show();
        if grids.contains(&grid) {
            break;
        }
        let new_grid = grid.step();
        grids.insert(grid);
        grid = new_grid;
    }
    //println!("{:?}", grid);
    let mut bio = 0;
    let mut pow = 1;
    for el in grid {
        if el {
            bio += pow;
        }
        pow *= 2;
    }
    bio
}

// Depth, x (col), y (row)
type CellId = (i32, u8, u8);

trait CellIdMethods {
    fn neigh(&self) -> Vec<CellId>;
}

impl CellIdMethods for CellId {
    fn neigh(&self) -> Vec<CellId> {
        let mut ret = Vec::<CellId>::new();
        let &(depth, x, y) = self;
        match (x, y) {
            (0, _) => ret.push((depth - 1, 1, 2)),
            (3, 2) => ret.extend((0..5).map(|i| (depth + 1, 4, i))),
            _ => ret.push((depth, x - 1, y)),
        };
        match (x, y) {
            (4, _) => ret.push((depth - 1, 3, 2)),
            (1, 2) => ret.extend((0..5).map(|i| (depth + 1, 0, i))),
            _ => ret.push((depth, x + 1, y)),
        };
        match (x, y) {
            (_, 0) => ret.push((depth - 1, 2, 1)),
            (2, 3) => ret.extend((0..5).map(|i| (depth + 1, i, 4))),
            _ => ret.push((depth, x, y - 1)),
        };
        match (x, y) {
            (_, 4) => ret.push((depth - 1, 2, 3)),
            (2, 1) => ret.extend((0..5).map(|i| (depth + 1, i, 0))),
            _ => ret.push((depth, x, y + 1)),
        };
        ret
    }
}

struct Universe {
    bugs: HashSet<CellId>,
}

impl Universe {
    fn ncount(&self, id: CellId) -> usize {
        id.neigh()
            .into_iter()
            .filter(|id| self.bugs.contains(id))
            .count()
    }

    fn step(&mut self) {
        // just look at bug cells and bug adjacent cells
        let mut to_update = HashMap::<CellId, bool>::new();
        let mut stack = Vec::<CellId>::new();
        stack.extend(&self.bugs);
        while let Some(id) = stack.pop() {
            if to_update.contains_key(&id) {
                continue;
            }
            let bug = self.bugs.contains(&id);
            if bug {
                stack.extend(id.neigh());
            }
            let ncount = self.ncount(id);
            if bug {
                to_update.insert(id, ncount == 1);
            } else {
                to_update.insert(id, ncount == 1 || ncount == 2);
            }
        }
        for (id, bug) in to_update {
            if bug {
                self.bugs.insert(id);
            } else {
                self.bugs.remove(&id);
            }
        }
    }
}

fn part2(inp: &str) -> usize {
    let mut uni = Universe {
        bugs: HashSet::new(),
    };
    for (row, line) in inp.lines().enumerate() {
        for (col, b) in line.chars().enumerate() {
            if b == '#' {
                uni.bugs.insert((0, col as u8, row as u8));
            }
        }
    }

    for _ in 1..=200 {
        uni.step();
    }
    uni.bugs.len()
}

xaoc::xaoc!();
