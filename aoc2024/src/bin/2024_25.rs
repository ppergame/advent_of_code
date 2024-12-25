use hashbrown::HashSet;

#[derive(Debug)]
struct Map {
    keys: Vec<Vec<i64>>,
    locks: Vec<Vec<i64>>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut keys = vec![];
        let mut locks = vec![];
        let mut it = inp.lines();
        loop {
            let mut map = HashSet::new();
            let mut more = false;
            for (row, l) in (&mut it).enumerate() {
                if l.is_empty() {
                    more = true;
                    break;
                }
                for (col, c) in l.chars().enumerate() {
                    match c {
                        '#' => {
                            map.insert((row as i64, col as i64));
                        }
                        '.' => {}
                        _ => unreachable!("unexpected char: {c}"),
                    }
                }
            }
            if !map.contains(&(0, 0)) {
                let mut key = vec![];
                for col in 0..5 {
                    key.push(5 - ((0..7).find(|&row| map.contains(&(row, col))).unwrap() - 1));
                }
                keys.push(key);
            } else {
                let mut lock = vec![];
                for col in 0..5 {
                    lock.push((0..7).rev().find(|&row| map.contains(&(row, col))).unwrap());
                }
                locks.push(lock);
            }
            if !more {
                break;
            }
        }
        Self { keys, locks }
    }
}

fn part1(inp: &str) -> i64 {
    let map = Map::parse(inp);
    let mut ret = 0;
    for key in &map.keys {
        for lock in &map.locks {
            if key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5) {
                ret += 1;
            }
        }
    }
    ret
}

fn part2(_inp: &str) -> i64 {
    0
}

xaoc::xaoc!();
