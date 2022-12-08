use std::collections::{HashMap, HashSet};

type Point = (i64, i64);

fn parse(inp: &str) -> HashMap<Point, i64> {
    let mut map = HashMap::new();
    for (y, line) in inp.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i64, y as i64), c.to_digit(10).unwrap() as i64);
        }
    }
    map
}

fn neigh(map: &HashMap<Point, i64>, p: Point) -> Vec<Point> {
    let mut ret = vec![];
    let (x, y) = p;
    for nx in [x - 1, x, x + 1] {
        for ny in [y - 1, y, y + 1] {
            if (nx, ny) != (x, y) && map.contains_key(&(nx, ny)) {
                ret.push((nx, ny));
            }
        }
    }
    ret
}

#[allow(dead_code)]
fn pmap(map: &HashMap<Point, i64>) {
    for y in 0..5 {
        for x in 0..5 {
            print!("{}", map[&(x, y)]);
        }
        println!();
    }
    println!();
}

fn part1(inp: &str) -> usize {
    let mut map = parse(inp);
    let mut flashes = 0;
    for _ in 0..100 {
        let mut newmap = map.clone();
        for d in newmap.values_mut() {
            *d += 1;
        }
        let mut flashed = HashSet::new();
        loop {
            let mut to_inc = vec![];
            for ((x, y), d) in &newmap {
                let x = *x;
                let y = *y;
                let d = *d;
                if d > 9 && !flashed.contains(&(x, y)) {
                    to_inc.extend(neigh(&map, (x, y)));
                    flashed.insert((x, y));
                }
            }
            if to_inc.is_empty() {
                break;
            }
            for (x, y) in to_inc {
                *newmap.get_mut(&(x, y)).unwrap() += 1;
            }
        }
        for d in newmap.values_mut() {
            if *d > 9 {
                *d = 0;
            }
        }
        flashes += flashed.len();
        map = newmap;
    }
    flashes
}

fn part2(inp: &str) -> usize {
    let mut map = parse(inp);
    for step in 1..10000 {
        let mut newmap = map.clone();
        for d in newmap.values_mut() {
            *d += 1;
        }
        let mut flashed = HashSet::new();
        loop {
            let mut to_inc = vec![];
            for ((x, y), d) in &newmap {
                let x = *x;
                let y = *y;
                let d = *d;
                if d > 9 && !flashed.contains(&(x, y)) {
                    to_inc.extend(neigh(&map, (x, y)));
                    flashed.insert((x, y));
                }
            }
            if to_inc.is_empty() {
                break;
            }
            for (x, y) in to_inc {
                *newmap.get_mut(&(x, y)).unwrap() += 1;
            }
        }
        for d in newmap.values_mut() {
            if *d > 9 {
                *d = 0;
            }
        }
        map = newmap;
        if map
            .values()
            .copied()
            .collect::<HashSet<_>>()
            .iter()
            .copied()
            .collect::<Vec<_>>()
            == vec![0]
        {
            return step;
        }
    }
    unreachable!();
}

xaoc::xaoc!();
