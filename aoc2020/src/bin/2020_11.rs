use itertools::iproduct;
use std::collections::HashMap;

#[derive(Ord, PartialOrd, Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Coord(i64, i64);

impl Coord {
    fn neigh(self) -> Vec<Coord> {
        let Coord(x, y) = self;
        iproduct!([x - 1, x, x + 1], [y - 1, y, y + 1])
            .filter(|&(nx, ny)| nx != x || ny != y)
            .map(|(nx, ny)| Coord(nx, ny))
            .collect()
    }
}

struct Map {
    map: HashMap<Coord, bool>,
}

impl Map {
    fn new(inp: &str) -> Map {
        let mut map = HashMap::new();
        for (row, line) in inp.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                map.insert(
                    Coord(col as i64, row as i64),
                    match c {
                        'L' => false,
                        '#' => true,
                        '.' => continue,
                        _ => unreachable!(),
                    },
                );
            }
        }
        Map { map }
    }

    fn step(&self) -> Map {
        let mut map = HashMap::new();
        for (&c, &occupied) in &self.map {
            let count = c
                .neigh()
                .into_iter()
                .filter(|nc| *self.map.get(nc).unwrap_or(&false))
                .count();
            if occupied {
                map.insert(c, count < 4);
            } else {
                map.insert(c, count == 0);
            }
        }
        Map { map }
    }

    fn step2(&self) -> Map {
        let mut map = HashMap::new();
        let &maxx = self.map.keys().map(|Coord(x, _)| x).max().unwrap();
        let &maxy = self.map.keys().map(|Coord(_, y)| y).max().unwrap();
        for (&c, &occupied) in &self.map {
            let mut count = 0;
            for (dx, dy) in iproduct!([-1, 0, 1], [-1, 0, 1]) {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let Coord(mut nx, mut ny) = c;
                loop {
                    nx += dx;
                    ny += dy;
                    if !(0..=maxx).contains(&nx) || !(0..=maxy).contains(&ny) {
                        break;
                    }
                    if let Some(&nocc) = self.map.get(&Coord(nx, ny)) {
                        if nocc {
                            count += 1;
                        }
                        break;
                    }
                }
                if occupied {
                    map.insert(c, count < 5);
                } else {
                    map.insert(c, count == 0);
                }
            }
        }
        Map { map }
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &maxx = self.map.keys().map(|Coord(x, _)| x).max().unwrap();
        let &maxy = self.map.keys().map(|Coord(_, y)| y).max().unwrap();
        for row in 0..=maxx {
            for col in 0..=maxy {
                f.write_str(match self.map.get(&Coord(col, row)) {
                    Some(true) => "#",
                    Some(false) => "L",
                    None => ".",
                })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn part1(inp: &str) -> usize {
    let mut map = Map::new(inp);
    for _ in 0..10000 {
        let next = map.step();
        if next.map == map.map {
            return map.map.values().filter(|x| **x).count();
        }
        map = next;
    }
    unreachable!();
}

fn part2(inp: &str) -> usize {
    let mut map = Map::new(inp);
    for _ in 0..10000 {
        let next = map.step2();
        if next.map == map.map {
            return map.map.values().filter(|x| **x).count();
        }
        map = next;
    }
    unreachable!();
}

xaoc::xaoc!();
