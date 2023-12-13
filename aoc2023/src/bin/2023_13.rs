struct Patt {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}

impl Patt {
    fn parse(s: &str) -> Self {
        let mut rows = Vec::<Vec<bool>>::new();
        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                });
            }
            rows.push(row);
        }
        let mut cols = vec![];
        for cidx in 0..rows[0].len() {
            let mut col = vec![];
            for row in &rows {
                col.push(row[cidx]);
            }
            cols.push(col);
        }
        Self { rows, cols }
    }

    fn mirror(&self) -> (usize, usize) {
        let find = |v: &[Vec<bool>]| {
            'outer: for col in 0..v.len() - 1 {
                for (c1, c2) in v[col + 1..].iter().zip(v[..=col].iter().rev()) {
                    if c1 != c2 {
                        continue 'outer;
                    }
                }
                return col + 1;
            }
            0
        };
        (find(&self.cols), find(&self.rows))
    }

    fn mirror2(&self) -> (usize, usize) {
        let (v1, h1) = self.mirror();
        let find = |v: &[Vec<bool>], prev: usize| {
            'outer: for col in 0..v.len() - 1 {
                if col + 1 == prev {
                    continue;
                }
                let mut flipped = false;
                for (c1, c2) in v[col + 1..].iter().zip(v[..=col].iter().rev()) {
                    let res = vcmp(c1, c2, !flipped);
                    if !res.eq {
                        continue 'outer;
                    }
                    if res.flipped {
                        flipped = true;
                    }
                }
                return col + 1;
            }
            0
        };
        (find(&self.cols, v1), find(&self.rows, h1))
    }
}

struct CmpRes {
    flipped: bool,
    eq: bool,
}

fn vcmp(a: &[bool], b: &[bool], allow_flip: bool) -> CmpRes {
    assert_eq!(a.len(), b.len());
    let mut flipped = false;
    let mut eq = true;
    for (a, b) in a.iter().zip(b.iter()) {
        if a != b {
            if allow_flip && !flipped {
                flipped = true;
            } else {
                eq = false;
                break;
            }
        }
    }
    CmpRes { flipped, eq }
}

fn part1(inp: &str) -> usize {
    let mut vert = 0;
    let mut hor = 0;
    for s in inp.split("\n\n") {
        let patt = Patt::parse(s);
        let (v, h) = patt.mirror();
        vert += v;
        hor += h;
    }
    vert + 100 * hor
}

fn part2(inp: &str) -> usize {
    let mut vert = 0;
    let mut hor = 0;
    for s in inp.split("\n\n") {
        let patt = Patt::parse(s);
        let (v, h) = patt.mirror2();
        vert += v;
        hor += h;
    }
    vert + 100 * hor
}

xaoc::xaoc!(
    sample = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
);
