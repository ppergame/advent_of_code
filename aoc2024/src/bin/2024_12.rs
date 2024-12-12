use array2d::Array2D;
use hashbrown::HashSet;
use itertools::Itertools as _;

struct Map {
    m: Array2D<char>,
    seen: HashSet<(usize, usize)>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut v = vec![];
        for line in inp.lines() {
            let mut acc = vec![];
            for c in line.chars() {
                acc.push(c);
            }
            v.push(acc);
        }
        let m = Array2D::from_rows(&v).unwrap();
        Self {
            m,
            seen: HashSet::new(),
        }
    }

    fn succ(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];
        if row > 0 {
            res.push((row - 1, col));
        }
        if col > 0 {
            res.push((row, col - 1));
        }
        if row < self.m.num_rows() - 1 {
            res.push((row + 1, col));
        }
        if col < self.m.num_columns() - 1 {
            res.push((row, col + 1));
        }
        res
    }

    fn price_walk(&mut self, (row, col): (usize, usize)) -> usize {
        let mut area = 0;
        let mut peri = 0;
        let mut stack = vec![(row, col)];
        while let Some((row, col)) = stack.pop() {
            if self.seen.contains(&(row, col)) {
                continue;
            }
            self.seen.insert((row, col));
            area += 1;
            let mut same = 0;
            for (nr, nc) in self.succ((row, col)) {
                if self.m[(nr, nc)] == self.m[(row, col)] {
                    same += 1;
                    stack.push((nr, nc));
                }
            }
            peri += 4 - same;
        }
        area * peri
    }

    // (Neighbor coords if any, start of line segment down or right, is_vertical, parity)
    #[allow(clippy::type_complexity)]
    fn succ2(
        &self,
        (row, col): (usize, usize),
    ) -> Vec<(Option<(usize, usize)>, (usize, usize), bool, bool)> {
        let mut res = vec![];
        let mut c = None;
        if row > 0 {
            c = Some((row - 1, col));
        }
        res.push((c, (row, col), false, false));
        c = None;
        if col > 0 {
            c = Some((row, col - 1));
        }
        res.push((c, (row, col), true, false));
        c = None;
        if row < self.m.num_rows() - 1 {
            c = Some((row + 1, col));
        }
        res.push((c, (row + 1, col), false, true));
        c = None;
        if col < self.m.num_columns() - 1 {
            c = Some((row, col + 1));
        }
        res.push((c, (row, col + 1), true, true));
        res
    }

    fn price_walk2(&mut self, (row, col): (usize, usize)) -> usize {
        let mut area = 0;
        let mut segments_v = vec![];
        let mut segments_h = vec![];
        let mut stack = vec![(row, col)];
        while let Some((row, col)) = stack.pop() {
            if self.seen.contains(&(row, col)) {
                continue;
            }
            self.seen.insert((row, col));
            area += 1;
            for (maybe_n, (seg_row, seg_col), is_vertical, parity) in self.succ2((row, col)) {
                let mut same = false;
                if let Some((nr, nc)) = maybe_n {
                    if self.m[(nr, nc)] == self.m[(row, col)] {
                        same = true;
                        stack.push((nr, nc));
                    }
                }
                if !same {
                    if is_vertical {
                        segments_v.push((seg_row, seg_col, parity));
                    } else {
                        segments_h.push((seg_row, seg_col, parity));
                    }
                }
            }
        }
        let mut sides = 0;
        eprintln!("({row}, {col}): {area} {}", self.m[(row, col)]);
        segments_v.sort_by_key(|&(r, c, _)| (c, r));
        eprintln!("v: {segments_v:?}");
        for ((r1, c1, p1), (r2, c2, p2)) in segments_v.into_iter().circular_tuple_windows() {
            eprintln!("{:?} {:?}", (r1, c1), (r2, c2));
            if c1 != c2 || r1 + 1 != r2 || p1 != p2 {
                sides += 1;
            }
        }
        eprintln!("sides so far {sides}");
        segments_h.sort_by_key(|&(r, c, _)| (r, c));
        eprintln!("h: {segments_h:?}");
        for ((r1, c1, p1), (r2, c2, p2)) in segments_h.into_iter().circular_tuple_windows() {
            eprintln!("{:?} {:?}", (r1, c1), (r2, c2));
            if r1 != r2 || c1 + 1 != c2 || p1 != p2 {
                sides += 1;
            }
        }
        eprintln!("sides total {sides}, area {area} -> {}", area * sides);
        area * sides
    }
}

fn part1(inp: &str) -> usize {
    let mut m = Map::parse(inp);
    let mut ret = 0;
    for row in 0..m.m.num_rows() {
        for col in 0..m.m.num_columns() {
            if m.seen.contains(&(row, col)) {
                continue;
            }
            ret += m.price_walk((row, col));
        }
    }
    ret
}

fn part2(inp: &str) -> usize {
    let mut m = Map::parse(inp);
    let mut ret = 0;
    for row in 0..m.m.num_rows() {
        for col in 0..m.m.num_columns() {
            if m.seen.contains(&(row, col)) {
                continue;
            }
            ret += m.price_walk2((row, col));
        }
    }
    ret
}

xaoc::xaoc!(
    sample = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
    sample2 = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
);
