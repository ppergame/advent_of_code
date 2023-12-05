use itertools::iproduct;
use std::collections::{HashMap, HashSet};

const DIM: usize = 10;

#[derive(Clone, Copy, Debug)]
enum Side {
    U,
    R,
    B,
    L,
}

impl Side {
    fn to_idx(self) -> usize {
        match self {
            Side::U => 0,
            Side::R => 1,
            Side::B => 2,
            Side::L => 3,
        }
    }
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Tile {
    tile_id: usize,
    tile: HashSet<Coord>,
}

impl Tile {
    fn newtile(&self, flip: bool, rot: usize) -> HashSet<Coord> {
        let mut tile = self.tile.clone();
        if flip {
            let mut newtile = HashSet::new();
            for &(x, y) in &tile {
                newtile.insert((x, DIM - 1 - y));
            }
            tile = newtile;
        }
        for _ in 0..rot {
            let mut newtile = HashSet::new();
            for &(x, y) in &tile {
                newtile.insert((DIM - 1 - y, x));
            }
            tile = newtile;
        }
        tile
    }

    fn flip(&self, flip: bool, rot: usize) -> Cell {
        let tile = self.newtile(flip, rot);
        let coll = |it: std::iter::Map<_, Box<dyn Fn(usize) -> bool>>| {
            let mut acc = 0;
            let mut shift = 1;
            for bit in it {
                if bit {
                    acc |= shift;
                }
                shift <<= 1;
            }
            acc
        };
        let edges = [
            coll((0..9).map(Box::new(|x| tile.contains(&(x, 0))))),
            coll((0..9).map(Box::new(|y| tile.contains(&(DIM - 1, y))))),
            coll((0..9).map(Box::new(|x| tile.contains(&(x, DIM - 1))))),
            coll((0..9).map(Box::new(|y| tile.contains(&(0, y))))),
        ];
        Cell {
            tile_id: self.tile_id,
            flip,
            rot,
            edges,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Clone, Copy)]
struct Cell {
    tile_id: usize,
    flip: bool,
    rot: usize,
    edges: [usize; 4],
}

struct Grid {
    g: Vec<Cell>,
    alt: Vec<Vec<Cell>>,
}

#[derive(Debug)]
struct Tiles {
    tiles: HashMap<usize, Tile>,
    grid_size: usize,
}

impl Tiles {
    fn parse(inp: &str) -> Tiles {
        let mut tiles = HashMap::new();
        let mut lines = inp.lines();
        while let Some(line) = lines.next() {
            assert!(line.starts_with("Tile "));
            let num = line
                .trim_start_matches("Tile ")
                .trim_end_matches(':')
                .parse()
                .unwrap();
            let mut tile = HashSet::new();
            for (row, line) in (&mut lines).enumerate() {
                if line.is_empty() {
                    break;
                }
                for (col, c) in line.chars().enumerate() {
                    if c == '#' {
                        tile.insert((col, row));
                    }
                }
            }
            tiles.insert(num, Tile { tile_id: num, tile });
        }
        let grid_size = (tiles.len() as f64).sqrt() as usize;
        Tiles { tiles, grid_size }
    }

    fn arrange(&self) -> Grid {
        let make_by_side = |side: Side| {
            let mut by = HashMap::<usize, Vec<Cell>>::new();
            for (tile, &flip, rot) in iproduct!(self.tiles.values(), &[false, true], 0..=3) {
                let cell = tile.flip(flip, rot);
                by.entry(cell.edges[side.to_idx()]).or_default().push(cell);
            }
            by
        };
        let by_left = make_by_side(Side::L);
        let by_up = make_by_side(Side::U);

        let mut alt0 = Vec::new();
        for (tile, &flip, rot) in iproduct!(self.tiles.values(), &[false, true], 0..=3) {
            alt0.push(tile.flip(flip, rot));
        }
        let mut grid = Grid {
            g: vec![alt0.pop().unwrap()],
            alt: vec![alt0],
        };
        let mut used = HashSet::new();
        used.insert(grid.g[0].tile_id);
        while grid.g.len() < self.tiles.len() {
            let pos = grid.g.len();
            let edge_up = if pos >= self.grid_size {
                let cell = grid.g[pos - self.grid_size];
                Some(cell.edges[Side::B.to_idx()])
            } else {
                None
            };
            let edge_left = if pos % self.grid_size != 0 {
                let cell = grid.g[pos - 1];
                Some(cell.edges[Side::R.to_idx()])
            } else {
                None
            };
            let mut altn = if let Some(edge_up) = edge_up {
                if let Some(edge_left) = edge_left {
                    by_left[&edge_left]
                        .iter()
                        .collect::<HashSet<_>>()
                        .intersection(&by_up[&edge_up].iter().collect::<HashSet<_>>())
                        .copied()
                        .copied()
                        .collect()
                } else {
                    by_up[&edge_up].clone()
                }
            } else {
                by_left[&edge_left.unwrap()].clone()
            };
            altn.retain(|x| !used.contains(&x.tile_id));
            if let Some(c) = altn.pop() {
                grid.g.push(c);
                grid.alt.push(altn);
                used.insert(c.tile_id);
            } else {
                loop {
                    let pc = grid.g.pop().unwrap();
                    used.remove(&pc.tile_id);
                    let mut altp = grid.alt.pop().unwrap();
                    if let Some(pc) = altp.pop() {
                        grid.g.push(pc);
                        used.insert(pc.tile_id);
                        grid.alt.push(altp);
                        break;
                    }
                }
            }
        }
        grid
    }
}

fn part1(inp: &str) -> usize {
    let tt = Tiles::parse(inp);
    let grid = tt.arrange();
    [
        0,
        tt.grid_size - 1,
        tt.tiles.len() - tt.grid_size,
        tt.tiles.len() - 1,
    ]
    .iter()
    .map(|&x| grid.g[x].tile_id)
    .product()
}

const MONSTER: &str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";

fn part2(inp: &str) -> usize {
    let tt = Tiles::parse(inp);
    let grid = tt.arrange();
    let mut map = HashSet::<Coord>::new();
    for (i, cell) in grid.g.iter().enumerate() {
        let g_col = i % tt.grid_size;
        let g_row = i / tt.grid_size;
        for &(x, y) in &tt.tiles[&cell.tile_id].newtile(cell.flip, cell.rot) {
            if x == 0 || y == 0 || x == DIM - 1 || y == DIM - 1 {
                continue;
            }
            let (x, y) = (x - 1, y - 1);
            map.insert((g_col * (DIM - 2) + x, g_row * (DIM - 2) + y));
        }
    }
    let size = tt.grid_size * (DIM - 2);
    /*
    for row in 0..size {
        for col in 0..size {
            print!("{}", if map.contains(&(col, row)) { '#' } else { '.' });
        }
        println!();
    }
    */
    let mut patt = HashSet::new();
    for (row, line) in MONSTER.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                patt.insert((col, row));
            }
        }
    }
    for (&flip, rot) in iproduct!(&[false, true], 0..=3) {
        let mut newmap = map.clone();
        if flip {
            let mut temp = HashSet::new();
            for &(x, y) in &newmap {
                temp.insert((x, size - 1 - y));
            }
            newmap = temp;
        }
        for _ in 0..rot {
            let mut temp = HashSet::new();
            for &(x, y) in &newmap {
                temp.insert((size - 1 - y, x));
            }
            newmap = temp;
        }
        let mut found = false;
        for col in 0..size {
            for row in 0..size {
                let mut has = true;
                for (px, py) in &patt {
                    if !newmap.contains(&((col + px), (row + py))) {
                        has = false;
                        break;
                    }
                }
                if has {
                    found = true;
                    for (px, py) in &patt {
                        newmap.remove(&((col + px), (row + py)));
                    }
                }
            }
        }
        if found {
            return newmap.len();
        }
    }
    panic!();
}

xaoc::xaoc!();
