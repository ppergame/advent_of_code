use std::collections::HashMap;

static DIRS: &[(i64, i64)] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];
static ALL_DIRS: &[(i64, i64)] = &[
    (1, 0),
    (0, -1),
    (-1, 0),
    (0, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

struct Map {
    map: HashMap<(i64, i64), i64>,
}

impl Map {
    fn fill(&mut self, x: i64, y: i64) -> i64 {
        let mut val = 0;
        for &(dx, dy) in ALL_DIRS.iter() {
            val += self.map.get(&(x + dx, y + dy)).unwrap_or(&0);
        }
        self.map.insert((x, y), val);
        val
    }
}

impl Default for Map {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), 1);
        Self { map }
    }
}

fn part1(inp: &str) -> i64 {
    let inp = inp.parse().unwrap();
    let (mut x, mut y) = (0, 0);
    let mut dir = 0;
    let mut steps = 1;
    let mut max_steps = 1;
    let mut bump = false;
    for _ in 2..=inp {
        let (dx, dy) = DIRS[dir];
        x += dx;
        y += dy;
        steps -= 1;
        if steps == 0 {
            if !bump {
                steps = max_steps;
                bump = true;
            } else {
                max_steps += 1;
                steps = max_steps;
                bump = false;
            }
            dir = (dir + 1) % DIRS.len();
        }
    }
    x.abs() + y.abs()
}

fn part2(inp: &str) -> i64 {
    let mut map = Map::default();
    let inp = inp.parse().unwrap();
    let (mut x, mut y) = (0, 0);
    let mut dir = 0;
    let mut steps = 1;
    let mut max_steps = 1;
    let mut bump = false;
    for _ in 2..=inp * 10 {
        let (dx, dy) = DIRS[dir];
        x += dx;
        y += dy;
        let val = map.fill(x, y);
        if val > inp {
            return val;
        }
        steps -= 1;
        if steps == 0 {
            if !bump {
                steps = max_steps;
                bump = true;
            } else {
                max_steps += 1;
                steps = max_steps;
                bump = false;
            }
            dir = (dir + 1) % DIRS.len();
        }
    }
    unreachable!();
}

xaoc::xaoc!(sample = "12");
