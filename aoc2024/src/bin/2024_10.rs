use array2d::Array2D;
use hashbrown::HashSet;

struct Map {
    map: Array2D<i8>,
    starts: Vec<(usize, usize)>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut v = vec![];
        let mut starts = vec![];
        for (row, line) in inp.lines().enumerate() {
            let mut acc = vec![];
            for (col, c) in line.chars().enumerate() {
                let c = c.to_digit(10).unwrap() as i8;
                if c == 0 {
                    starts.push((row, col));
                }
                acc.push(c);
            }
            v.push(acc);
        }
        let map = Array2D::from_rows(&v).unwrap();
        Self { map, starts }
    }

    fn succ(&self, (row, col): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        let mut res = vec![];
        if row > 0 {
            res.push((row - 1, col));
        }
        if col > 0 {
            res.push((row, col - 1));
        }
        if row < self.map.num_rows() - 1 {
            res.push((row + 1, col));
        }
        if col < self.map.num_columns() - 1 {
            res.push((row, col + 1));
        }
        let c = self.map[(row, col)];
        res.into_iter().filter(move |&cc| self.map[cc] == c + 1)
    }

    fn score(&self, (row, col): (usize, usize)) -> usize {
        let mut seen = HashSet::new();
        let mut stack = vec![(row, col)];
        let mut ends = HashSet::new();
        while let Some((row, col)) = stack.pop() {
            if seen.contains(&(row, col)) {
                continue;
            }
            seen.insert((row, col));
            let c = self.map[(row, col)];
            if c == 9 {
                ends.insert((row, col));
                continue;
            }
            for cc in self.succ((row, col)) {
                stack.push(cc);
            }
        }
        ends.len()
    }
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    let mut ret = 0;
    for &(row, col) in &map.starts {
        ret += map.score((row, col));
    }
    ret
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    map.starts
        .iter()
        .map(|&(row, col)| {
            pathfinding::directed::count_paths::count_paths(
                (row, col),
                |&(row, col)| map.succ((row, col)),
                |&(row, col)| map.map[(row, col)] == 9,
            )
        })
        .sum()
}

xaoc::xaoc!();
