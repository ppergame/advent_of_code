use std::collections::HashSet;

const WIDTH: i64 = 100;

fn neigh(x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut ret = vec![];
    for nx in x - 1..=x + 1 {
        for ny in y - 1..=y + 1 {
            if (nx, ny) == (x, y)
                || !(0..=WIDTH - 1).contains(&nx)
                || !(0..=WIDTH - 1).contains(&ny)
            {
                continue;
            }
            ret.push((nx, ny));
        }
    }
    ret
}

fn parse(inp: &str) -> HashSet<(i64, i64)> {
    inp.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[allow(dead_code)]
fn print(map: &HashSet<(i64, i64)>) {
    for y in 0..WIDTH {
        for x in 0..WIDTH {
            print!("{}", if map.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

fn part1(inp: &str) -> usize {
    let mut map = parse(inp);
    for _ in 0..100 {
        let mut newmap = HashSet::new();
        for x in 0..WIDTH {
            for y in 0..WIDTH {
                let nc = neigh(x, y).into_iter().filter(|c| map.contains(c)).count();
                if map.contains(&(x, y)) {
                    if nc == 2 || nc == 3 {
                        newmap.insert((x, y));
                    }
                } else if nc == 3 {
                    newmap.insert((x, y));
                }
            }
        }
        map = newmap;
    }
    map.len()
}

fn part2(inp: &str) -> usize {
    let mut map = parse(inp);
    for _ in 0..100 {
        let mut newmap = HashSet::new();
        for x in 0..WIDTH {
            for y in 0..WIDTH {
                let nc = neigh(x, y).into_iter().filter(|c| map.contains(c)).count();
                if map.contains(&(x, y)) {
                    if nc == 2 || nc == 3 {
                        newmap.insert((x, y));
                    }
                } else if nc == 3 {
                    newmap.insert((x, y));
                }
            }
        }
        newmap.insert((0, 0));
        newmap.insert((0, WIDTH - 1));
        newmap.insert((WIDTH - 1, 0));
        newmap.insert((WIDTH - 1, WIDTH - 1));
        map = newmap;
    }
    map.len()
}

xaoc::xaoc!();
