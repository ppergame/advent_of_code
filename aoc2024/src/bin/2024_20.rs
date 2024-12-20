use hashbrown::HashSet;
use pathfinding::prelude::astar;

struct Map {
    walls: HashSet<(i64, i64)>,
    max_row: i64,
    #[allow(unused)]
    max_col: i64,
    start: (i64, i64),
    end: (i64, i64),
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut walls = HashSet::new();
        let mut max_row = 0;
        let mut max_col = 0;
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, line) in inp.lines().enumerate() {
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col);
                match c {
                    '#' => {
                        walls.insert((row, col));
                    }
                    'S' => {
                        start = (row, col);
                    }
                    'E' => {
                        end = (row, col);
                    }
                    '.' => (),
                    _ => panic!("Unknown char: {c}"),
                }
            }
        }
        Self {
            walls,
            max_row,
            max_col,
            start,
            end,
        }
    }

    fn succ(&self, (row, col): (i64, i64)) -> Vec<((i64, i64), u64)> {
        let mut ret = vec![];
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_pos = (row + dr, col + dc);
            if !self.walls.contains(&new_pos) {
                ret.push((new_pos, 1));
            }
        }
        ret
    }
}

fn diff((r1, c1): (i64, i64), (r2, c2): (i64, i64)) -> u64 {
    r1.abs_diff(r2) + c1.abs_diff(c2)
}

fn solve2(map: &Map, threshold: u64, max_cheat: u64) -> u64 {
    let (path, base_cost) = astar(
        &map.start,
        |&pos| map.succ(pos),
        |&pos| diff(pos, map.end),
        |&pos| pos == map.end,
    )
    .unwrap();
    let mut ret = 0;
    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let cheat_cost = diff(path[i], path[j]);
            if cheat_cost > max_cheat {
                continue;
            }
            let start_cost = i as u64;
            let end_cost = (path.len() - j - 1) as u64;
            if start_cost + cheat_cost + end_cost + threshold <= base_cost {
                ret += 1;
            }
        }
    }
    ret
}

fn part1(inp: &str) -> u64 {
    let map = Map::parse(inp);
    let threshold = if map.max_row > 20 { 100 } else { 40 };
    solve2(&map, threshold, 2)
}

fn part2(inp: &str) -> u64 {
    let map = Map::parse(inp);
    let threshold = if map.max_row > 20 { 100 } else { 70 };
    solve2(&map, threshold, 20)
}

xaoc::xaoc!(
    sample = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
);
