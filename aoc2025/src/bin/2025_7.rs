use hashbrown::{HashMap, HashSet};

#[derive(Debug)]
struct Map {
    map: HashSet<(i64, i64)>,
    s: (i64, i64),
    height: i64,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut map = HashSet::new();
        let mut s = (0, 0);
        let mut height = 0;
        for (row, l) in inp.lines().enumerate() {
            let row = row as i64;
            for (col, c) in l.chars().enumerate() {
                let col = col as i64;
                if c == 'S' {
                    s = (row, col);
                } else if c == '^' {
                    map.insert((row, col));
                }
            }
            height += 1;
        }
        Self { map, s, height }
    }
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    let mut splits = 0;
    let mut beams = HashSet::from([map.s.1]);
    let mut row = map.s.0 + 1;
    while row < map.height {
        let mut cont_beams = HashSet::new();
        let mut split_beams = HashSet::new();
        for bcol in beams {
            if map.map.contains(&(row + 1, bcol)) {
                split_beams.insert(bcol - 1);
                split_beams.insert(bcol + 1);
                splits += 1;
            } else {
                cont_beams.insert(bcol);
            }
        }
        beams = split_beams;
        beams.extend(cont_beams);
        row += 1;
    }
    splits
}

struct Memo<'a> {
    cache: HashMap<(i64, i64), i64>,
    map: &'a Map,
}

impl<'a> Memo<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            cache: HashMap::new(),
            map,
        }
    }
    fn count(&mut self, (row, col): (i64, i64)) -> i64 {
        if let Some(&ret) = self.cache.get(&(row, col)) {
            return ret;
        }
        if self.map.height == row {
            return 1;
        }
        if !self.map.map.contains(&(row + 1, col)) {
            let ret = self.count((row + 1, col));
            self.cache.insert((row, col), ret);
            return ret;
        }
        let ret = self.count((row + 1, col - 1)) + self.count((row + 1, col + 1));
        self.cache.insert((row, col), ret);
        ret
    }
}

fn part2(inp: &str) -> i64 {
    let map = Map::parse(inp);
    let mut memo = Memo::new(&map);
    memo.count(map.s)
}

xaoc::xaoc!(
    sample = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
);
