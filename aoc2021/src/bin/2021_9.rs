use std::collections::{HashMap, HashSet};

type Point = (i64, i64);

pub struct Map {
    map: HashMap<Point, i64>,
    width: i64,
    height: i64,
}

impl Map {
    fn neigh(&self, p: Point) -> Vec<Point> {
        let (x, y) = p;
        let mut ret = vec![];
        if x > 0 {
            ret.push((x - 1, y));
        }
        if y > 0 {
            ret.push((x, y - 1));
        }
        if x < self.width - 1 {
            ret.push((x + 1, y));
        }
        if y < self.height - 1 {
            ret.push((x, y + 1));
        }
        ret
    }
}

fn parse(inp: &str) -> Map {
    let mut map = HashMap::new();
    for (y, line) in inp.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i64, y as i64), c.to_digit(10).unwrap() as i64);
        }
    }
    let width = *map.keys().map(|(x, _)| x).max().unwrap() + 1;
    let height = *map.keys().map(|(_, y)| y).max().unwrap() + 1;
    Map { map, width, height }
}

fn part1(inp: &str) -> i64 {
    let map = parse(inp);
    let mut risk = 0;
    for (p, d) in &map.map {
        if map.neigh(*p).into_iter().all(|np| map.map[&np] > *d) {
            risk += d + 1;
        }
    }
    risk
}

fn part2(inp: &str) -> i64 {
    let map = parse(inp);
    let mut seen = HashSet::new();
    let mut basins = vec![];
    for (p, d) in &map.map {
        let (p, d) = (*p, *d);
        if d == 9 {
            seen.insert(p);
            continue;
        }
        let mut basin = 0;
        let mut stack = vec![p];
        while let Some(p) = stack.pop() {
            if seen.contains(&p) {
                continue;
            }
            seen.insert(p);
            let d = map.map[&p];
            if d == 9 {
                continue;
            }
            basin += 1;
            stack.extend(map.neigh(p));
        }
        if basin != 0 {
            basins.push(basin);
        }
    }
    basins.sort_by_key(|x| -x);
    basins[0] * basins[1] * basins[2]
}

xaoc::xaoc!();
