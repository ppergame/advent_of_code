use itertools::Itertools as _;
use ndarray::{Array3, Dim, Ix3, NdIndex};
use sscanf::scanf;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord(usize, usize, usize);

unsafe impl NdIndex<Dim<[usize; 3]>> for Coord {
    fn index_checked(&self, dim: &Ix3, strides: &Ix3) -> Option<isize> {
        (self.0, self.1, self.2).index_checked(dim, strides)
    }

    fn index_unchecked(&self, strides: &Ix3) -> isize {
        (self.0, self.1, self.2).index_unchecked(strides)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Brick {
    idx: u16,
    start: Coord,
    end: Coord,
}

impl Brick {
    fn iter(&self) -> impl Iterator<Item = Coord> {
        let (start, end) = (self.start, self.end);
        (start.0..=end.0).flat_map(move |x| {
            (start.1..=end.1).flat_map(move |y| (start.2..=end.2).map(move |z| Coord(x, y, z)))
        })
    }

    fn coords_above(&self) -> Vec<Coord> {
        let (start, end) = (self.start, self.end);
        if start.2 != end.2 {
            return vec![Coord(end.0, end.1, end.2 + 1)];
        }
        self.iter()
            .map(|coord| Coord(coord.0, coord.1, coord.2 + 1))
            .collect()
    }

    fn coords_below(&self) -> Vec<Coord> {
        let (start, end) = (self.start, self.end);
        if start.2 != end.2 {
            return vec![Coord(start.0, start.1, start.2 - 1)];
        }
        self.iter()
            .map(|coord| Coord(coord.0, coord.1, coord.2 - 1))
            .collect()
    }
}

#[derive(Debug)]
struct Snapshot {
    space: Array3<u16>,
    bricks: Vec<Brick>,
}

fn parse_line(line: &str) -> (Coord, Coord) {
    let (x1, y1, z1, x2, y2, z2) =
        scanf!(line, "{usize},{usize},{usize}~{usize},{usize},{usize}").unwrap();
    (Coord(x1, y1, z1), Coord(x2, y2, z2))
}

impl Snapshot {
    fn parse(inp: &str) -> Snapshot {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for line in inp.lines() {
            let (start, end) = parse_line(line);
            assert!(start.0 <= end.0);
            assert!(start.1 <= end.1);
            assert!(start.2 <= end.2);
            max_x = max_x.max(start.0).max(end.0);
            max_y = max_y.max(start.1).max(end.1);
            max_z = max_z.max(start.2).max(end.2);
        }
        let mut space = Array3::zeros((max_x + 1, max_y + 1, max_z + 2));
        let mut bricks = vec![];
        for line in inp.lines() {
            let (start, end) = parse_line(line);
            bricks.push(Brick { idx: 0, start, end });
        }
        bricks.sort_by_key(|brick| brick.start.2);
        for (idx, brick) in bricks.iter_mut().enumerate() {
            brick.idx = idx as u16 + 1;
            add_brick(&mut space, brick);
        }
        Snapshot { space, bricks }
    }

    fn bricks_above(&self, brick: &Brick) -> Vec<u16> {
        brick
            .coords_above()
            .into_iter()
            .filter_map(|coord| {
                let idx = self.space[coord];
                if idx == 0 {
                    None
                } else {
                    assert!(coord.2 > 0);
                    Some(idx)
                }
            })
            .unique()
            .collect()
    }

    fn bricks_below(&self, brick: &Brick) -> Vec<u16> {
        brick
            .coords_below()
            .into_iter()
            .filter_map(|coord| {
                let idx = self.space[coord];
                if idx == 0 {
                    None
                } else {
                    assert!(coord.2 > 0);
                    Some(idx)
                }
            })
            .unique()
            .collect()
    }

    fn settle(mut self) -> Settled {
        for brick in self.bricks.iter_mut() {
            loop {
                let supported = brick
                    .coords_below()
                    .into_iter()
                    .any(|coord| coord.2 == 0 || self.space[coord] != 0);
                if supported {
                    break;
                }
                remove_brick(&mut self.space, brick);
                brick.start.2 -= 1;
                brick.end.2 -= 1;
                add_brick(&mut self.space, brick)
            }
        }
        let supports = std::iter::once(vec![])
            .chain(self.bricks.iter().map(|brick| self.bricks_above(brick)))
            .collect();
        let supported_by = std::iter::once(vec![])
            .chain(self.bricks.iter().map(|brick| self.bricks_below(brick)))
            .collect();
        Settled {
            space: self.space,
            bricks: self.bricks,
            supports,
            supported_by,
        }
    }
}

#[derive(Debug)]
struct Settled {
    space: Array3<u16>,
    bricks: Vec<Brick>,
    supports: Vec<Vec<u16>>,
    supported_by: Vec<Vec<u16>>,
}

impl Settled {
    #[allow(dead_code)]
    fn print_zx(&self) {
        eprintln!(" x ");
        for x in 0..=self.space.dim().0 {
            eprint!("{x}");
        }
        eprintln!();
        for z in (0..self.space.dim().2).rev() {
            for x in 0..self.space.dim().0 {
                let mut found = false;
                for y in 0..self.space.dim().1 {
                    let idx = self.space[(x, y, z)];
                    if idx != 0 {
                        let idx = (idx as u8 + b'A' - 1) as char;
                        eprint!("{}", idx);
                        found = true;
                        break;
                    }
                }
                if !found {
                    eprint!(".");
                }
            }
            eprintln!(" {z}");
        }
    }

    #[allow(dead_code)]
    fn print_zy(&self) {
        eprintln!(" y ");
        for y in 0..=self.space.dim().1 {
            eprint!("{y}");
        }
        eprintln!();
        for z in (0..self.space.dim().2).rev() {
            for y in 0..self.space.dim().1 {
                let mut found = false;
                for x in 0..self.space.dim().0 {
                    let idx = self.space[(x, y, z)];
                    if idx != 0 {
                        let idx = (idx as u8 + b'A' - 1) as char;
                        eprint!("{}", idx);
                        found = true;
                        break;
                    }
                }
                if !found {
                    eprint!(".");
                }
            }
            eprintln!(" {z}");
        }
    }
}

fn add_brick(space: &mut Array3<u16>, brick: &Brick) {
    brick.iter().for_each(|coord| space[coord] = brick.idx);
}

fn remove_brick(space: &mut Array3<u16>, brick: &Brick) {
    brick.iter().for_each(|coord| space[coord] = 0);
}

fn part1(inp: &str) -> usize {
    let snapshot = Snapshot::parse(inp);
    let settled = snapshot.settle();
    let one_support = settled
        .supports
        .iter()
        .filter(|above| {
            for idx in above.iter() {
                if settled.supported_by[*idx as usize].len() == 1 {
                    return true;
                }
            }
            false
        })
        .count();
    settled.bricks.len() - one_support
}

fn part2(inp: &str) -> usize {
    let snapshot = Snapshot::parse(inp);
    let settled = snapshot.settle();
    let mut ret = 0;
    for brick in &settled.bricks {
        let mut supported_by = settled.supported_by.clone();
        let mut todo = vec![brick.idx];
        let mut count = 0;
        while let Some(idx) = todo.pop() {
            count += 1;
            for &idx2 in &settled.supports[idx as usize] {
                let sb = &mut supported_by[idx2 as usize];
                sb.retain(|&i| i != idx);
                if sb.is_empty() {
                    todo.push(idx2);
                }
            }
        }
        ret += count - 1;
    }
    ret
}

xaoc::xaoc!();
