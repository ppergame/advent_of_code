use std::collections::{HashSet, VecDeque};

struct Map {
    map: HashSet<(i64, i64)>,
    width: i64,
    height: i64,
    start: (i64, i64),
}

impl Map {
    fn parse(inp: &str) -> Map {
        let mut map = HashSet::new();
        let mut start = None;
        let mut height = 0;
        let mut width = 0;
        for (row, line) in inp.lines().enumerate() {
            let row = row as i64;
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                if c == 'S' {
                    start = Some((row, col));
                }
                match c {
                    '#' => {
                        map.insert((row, col));
                        height = height.max(row);
                        width = width.max(col);
                    }
                    '.' | 'S' => {}
                    _ => unreachable!(),
                }
            }
        }
        Map {
            map,
            width,
            height,
            start: start.unwrap(),
        }
    }

    fn reach(&self, steps: usize) -> usize {
        let mut seen = HashSet::new();
        let mut todo = VecDeque::new();
        todo.push_back((self.start, 0));
        while let Some((pos, depth)) = todo.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);
            if depth < steps {
                todo.extend(adj(pos).into_iter().filter_map(|p| {
                    if self.is_wall(p) {
                        None
                    } else {
                        Some((p, depth + 1))
                    }
                }));
            }
        }
        seen.into_iter()
            .filter(|p| (p.0 + p.1).rem_euclid(2) == steps as i64 % 2)
            .count()
    }

    #[allow(dead_code)]
    fn print(&self, found: &HashSet<(i64, i64)>) {
        for row in 0..self.height + 1 {
            for col in 0..self.width + 1 {
                if found.contains(&(row, col)) {
                    eprint!("O");
                } else if self.map.contains(&(row, col)) {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
    }

    fn is_wall(&self, (row, col): (i64, i64)) -> bool {
        self.map.contains(&(
            row.rem_euclid(self.height + 2),
            col.rem_euclid(self.width + 2),
        ))
    }
}

fn adj((row, col): (i64, i64)) -> Vec<(i64, i64)> {
    vec![
        (row - 1, col),
        (row, col - 1),
        (row + 1, col),
        (row, col + 1),
    ]
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    let steps = if map.map.len() < 100 { 6 } else { 64 };
    map.reach(steps)
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    if map.map.len() < 100 {
        return map.reach(1000);
    }
    const GOAL: usize = 26501365;
    let start = GOAL % 262;
    let mut prev = 0;
    let mut pprev = 0;
    let mut ppprev = 0;
    for steps in (start..GOAL).step_by(262) {
        let count = map.reach(steps);
        let mut inc = count - prev;
        let inc2 = count - prev - pprev;
        let inc3 = count - prev - pprev - ppprev;
        ppprev = count - prev - pprev;
        pprev = count - prev;
        prev = count;
        if inc3 == 0 {
            let mut sub_steps = steps;
            let mut count = count;
            while sub_steps < GOAL {
                sub_steps += 262;
                inc += inc2;
                count += inc;
            }
            return count;
        }
    }
    unreachable!();
}

xaoc::xaoc!(
    sample = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
);
