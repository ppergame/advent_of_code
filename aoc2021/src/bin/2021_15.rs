use std::collections::HashMap;

type Point = (i64, i64);

pub struct Input {
    map: HashMap<Point, i64>,
    width: i64,
    height: i64,
}

fn parse(inp: &str) -> Input {
    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in inp.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            width = width.max(x as i64);
            height = height.max(y as i64);
            map.insert((x as i64, y as i64), c.to_digit(10).unwrap() as i64);
        }
    }
    width += 1;
    height += 1;
    Input { map, width, height }
}

fn part1(inp: &str) -> i64 {
    let inp = parse(inp);
    pathfinding::directed::dijkstra::dijkstra(
        &(0, 0),
        |(x, y)| {
            let (x, y) = (*x, *y);
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter_map(|(nx, ny)| inp.map.get(&(nx, ny)).map(|c| ((nx, ny), *c)))
        },
        |(x, y)| *x == inp.width - 1 && *y == inp.height - 1,
    )
    .unwrap()
    .1
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let mut map = inp.map.clone();
    let mut prev = map.clone();
    for _ in 0..4 {
        let mut cur = HashMap::new();
        for ((x, y), c) in prev {
            let mut c = c + 1;
            if c == 10 {
                c = 1;
            }
            cur.insert((x + inp.width, y), c);
        }
        map.extend(&cur);
        prev = cur;
    }

    let mut prev = map.clone();
    for _ in 0..4 {
        let mut cur = HashMap::new();
        for ((x, y), c) in prev {
            let mut c = c + 1;
            if c == 10 {
                c = 1;
            }
            cur.insert((x, y + inp.height), c);
        }
        map.extend(&cur);
        prev = cur;
    }

    let inp = Input {
        map,
        width: inp.width * 5,
        height: inp.height * 5,
    };
    pathfinding::directed::dijkstra::dijkstra(
        &(0, 0),
        |(x, y)| {
            let (x, y) = (*x, *y);
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter_map(|(nx, ny)| inp.map.get(&(nx, ny)).map(|c| ((nx, ny), *c)))
        },
        |(x, y)| *x == inp.width - 1 && *y == inp.height - 1,
    )
    .unwrap()
    .1
}

xaoc::xaoc!();
