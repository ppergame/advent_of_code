use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;

fn parse_map(s: &str) -> HashSet<(i32, i32)> {
    let mut ret = HashSet::new();
    for (row, line) in s.split_whitespace().enumerate() {
        for (col, b) in line.as_bytes().iter().enumerate() {
            if *b == b'#' {
                ret.insert((col.try_into().unwrap(), row.try_into().unwrap()));
            }
        }
    }
    ret
}

fn make_equivs(
    map: &HashSet<(i32, i32)>,
    sx: i32,
    sy: i32,
) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut ret = HashMap::new();
    for (ax, ay) in map {
        if (sx, sy) == (*ax, *ay) {
            continue;
        }
        let (dx, dy) = (*ax - sx, *ay - sy);
        let g = num::integer::gcd(dx, dy);
        ret.entry((dx / g, dy / g))
            .or_insert_with(Vec::new)
            .push((*ax, *ay));
    }
    ret
}

fn find_best(map: &HashSet<(i32, i32)>) -> ((i32, i32), i32) {
    let mut mx = -1;
    let mut my = -1;
    let mut max = 0;
    for (sx, sy) in map {
        let equivs = make_equivs(map, *sx, *sy);
        if max < equivs.len() {
            max = equivs.len();
            mx = *sx;
            my = *sy;
        }
    }
    ((mx, my), max.try_into().unwrap())
}

fn part1(inp: &str) -> i32 {
    let map = parse_map(inp);
    let ((_, _), max) = find_best(&map);
    max
}

struct Equiv {
    dx: i32,
    dy: i32,
    aa: Vec<(i32, i32)>,
}

fn part2(inp: &str) -> i32 {
    let map = parse_map(inp);
    let ((mx, my), _) = find_best(&map);
    let equivs = make_equivs(&map, mx, my);
    let mut sequivs: Vec<Equiv> = equivs
        .iter()
        .map(|((dx, dy), v)| Equiv {
            dx: *dx,
            dy: *dy,
            aa: v.to_vec(),
        })
        .collect();
    sequivs.sort_by_key(|s| ordered_float::NotNan::new(-(s.dx as f32).atan2(s.dy as f32)).unwrap());
    for s in sequivs.iter_mut() {
        s.aa.sort_by_key(|(ax, ay)| (mx - ax).abs() + (my - ay).abs());
    }
    let mut count = 0;
    loop {
        let mut found = false;
        for s in sequivs.iter_mut() {
            if s.aa.is_empty() {
                continue;
            }
            let (ax, ay) = s.aa.remove(0);
            count += 1;
            if count == 200 {
                return ax * 100 + ay;
                // TODO
                //break 'outer;
            }
            found = true;
        }
        if !found {
            panic!("wtf");
        }
    }
    /*
    let key = |x: i32, y: i32| -(x as f32).atan2(y as f32);
    for (x, y) in [
        (0, -1),
        (3, -5),
        (5, -3),
        (1, 0),
        (5, 3),
        (3, 5),
        (0, 1),
        (-3, 5),
        (-5, 3),
        (-1, 0),
        (-5, -3),
        (-3, -5),
    ] {
        println!("{} {} -> {}", x, y, key(x, y));
    }
    */
}

xaoc::xaoc!();
