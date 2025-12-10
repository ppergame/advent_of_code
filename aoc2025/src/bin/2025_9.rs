use hashbrown::{HashMap, HashSet};
use itertools::Itertools as _;
use sscanf::scanf;

fn part1(inp: &str) -> i64 {
    inp.lines()
        .map(|l| scanf!(l, "{i64},{i64}").unwrap())
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1))
        .max()
        .unwrap()
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    x: i64,
    y: i64,
    orig_x: i64,
    orig_y: i64,
}

fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}

fn part2(inp: &str) -> i64 {
    let mut tiles = inp
        .lines()
        .map(|l| {
            let (x, y) = scanf!(l, "{i64},{i64}").unwrap();
            Tile {
                x,
                y,
                orig_x: x,
                orig_y: y,
            }
        })
        .collect::<Vec<_>>();
    let by_x = tiles.iter_mut().sorted_by_key(|t| t.x).collect::<Vec<_>>();
    let mut max_x = 0;
    let mut prev = by_x[0].orig_x;
    for t in by_x {
        if prev != t.orig_x {
            max_x += 2;
            prev = t.orig_x;
        }
        t.x = max_x;
    }
    let by_y = tiles.iter_mut().sorted_by_key(|t| t.y).collect::<Vec<_>>();
    let mut max_y = 0;
    let mut prev = by_y[0].orig_y;
    for t in by_y {
        if prev != t.orig_y {
            max_y += 2;
            prev = t.orig_y;
        }
        t.y = max_y;
    }
    let segments = tiles
        .iter()
        .copied()
        .circular_tuple_windows()
        .collect::<Vec<(_, _)>>();
    let mut by_x = HashMap::<i64, Vec<_>>::new();
    let mut green_tiles = HashSet::new();
    for (t1, t2) in segments {
        let (x1, x2) = minmax(t1.x, t2.x);
        let (y1, y2) = minmax(t1.y, t2.y);
        for x in x1..=x2 {
            for y in y1..=y2 {
                green_tiles.insert((x, y));
            }
        }
        if x1 == x2 {
            by_x.entry(t1.x).or_default().push(y1..y2);
        }
    }
    for y in 0..=max_y {
        let mut inside = false;
        for x in 0..=max_x {
            if by_x
                .get(&x)
                .is_some_and(|segs| segs.iter().any(|r| r.contains(&y)))
            {
                inside = !inside;
            } else if inside {
                green_tiles.insert((x, y));
            }
        }
    }

    let max_y = max_y as usize + 1;
    let max_x = max_x as usize + 1;
    let mut pref = vec![vec![0; max_y]; max_x];
    for y in 0..max_y {
        for x in 0..max_x {
            pref[x][y] = green_tiles.contains(&(x as i64, y as i64)) as usize;
            if x > 0 {
                pref[x][y] += pref[x - 1][y];
            }
            if y > 0 {
                pref[x][y] += pref[x][y - 1];
            }
            if x > 0 && y > 0 {
                pref[x][y] -= pref[x - 1][y - 1];
            }
        }
    }

    tiles
        .iter()
        .tuple_combinations()
        .filter_map(|(t1, t2)| {
            let (x1, x2) = minmax(t1.x as usize, t2.x as usize);
            let (y1, y2) = minmax(t1.y as usize, t2.y as usize);
            let mut contains = pref[x2][y2];
            if x1 > 0 && y1 > 0 {
                contains += pref[x1 - 1][y1 - 1];
            }
            if x1 > 0 {
                contains -= pref[x1 - 1][y2];
            }
            if y1 > 0 {
                contains -= pref[x2][y1 - 1];
            }
            (contains == (x2 - x1 + 1) * (y2 - y1 + 1))
                .then(|| ((t1.orig_x - t2.orig_x).abs() + 1) * ((t1.orig_y - t2.orig_y).abs() + 1))
        })
        .max()
        .unwrap()
}

xaoc::xaoc!();
