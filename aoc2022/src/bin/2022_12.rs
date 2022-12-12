use array2d::Array2D;
use pathfinding::prelude::*;

fn succ((row, col): (usize, usize), map: &Array2D<char>) -> Vec<((usize, usize), usize)> {
    let mut res = vec![];
    if row > 0 {
        res.push((row - 1, col));
    }
    if col > 0 {
        res.push((row, col - 1));
    }
    if row < map.num_rows() - 1 {
        res.push((row + 1, col));
    }
    if col < map.num_columns() - 1 {
        res.push((row, col + 1));
    }
    let c = map[(row, col)] as u8;
    res.into_iter()
        .filter(|&cc| map[cc] as u8 <= c + 1)
        .map(|cc| (cc, 1))
        .collect()
}

fn part1(inp: &str) -> usize {
    let mut v = vec![];
    let mut start = None;
    let mut end = None;
    for (row, line) in inp.lines().enumerate() {
        let mut acc = vec![];
        for (col, c) in line.chars().enumerate() {
            let c = match c {
                'S' => {
                    start = Some((row, col));
                    'a'
                }
                'E' => {
                    end = Some((row, col));
                    'z'
                }
                _ => c,
            };
            acc.push(c);
        }
        v.push(acc);
    }
    let map = Array2D::from_rows(&v).unwrap();
    let start = start.unwrap();
    let end = end.unwrap();
    dijkstra(&start, |&cc| succ(cc, &map), |&cc| cc == end)
        .unwrap()
        .1
}

fn rev_succ((row, col): (usize, usize), map: &Array2D<char>) -> Vec<((usize, usize), usize)> {
    let mut res = vec![];
    if row > 0 {
        res.push((row - 1, col));
    }
    if col > 0 {
        res.push((row, col - 1));
    }
    if row < map.num_rows() - 1 {
        res.push((row + 1, col));
    }
    if col < map.num_columns() - 1 {
        res.push((row, col + 1));
    }
    let c = map[(row, col)] as u8;
    res.into_iter()
        .filter(|&cc| map[cc] as u8 >= c - 1)
        .map(|cc| (cc, 1))
        .collect()
}

fn part2(inp: &str) -> usize {
    let mut v = vec![];
    let mut end = None;
    for (row, line) in inp.lines().enumerate() {
        let mut acc = vec![];
        for (col, c) in line.chars().enumerate() {
            let c = match c {
                'S' => 'a',
                'E' => {
                    end = Some((row, col));
                    'z'
                }
                _ => c,
            };
            acc.push(c);
        }
        v.push(acc);
    }
    let map = Array2D::from_rows(&v).unwrap();
    let end = end.unwrap();
    let paths = dijkstra_all(&end, |&cc| rev_succ(cc, &map));
    paths
        .into_iter()
        .filter_map(|((row, col), (_, cost))| {
            if map.get(row, col) == Some(&'a') {
                Some(cost)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

xaoc::xaoc!(
    sample = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#
);
