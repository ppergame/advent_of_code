use hashbrown::HashSet;
use pathfinding::prelude::dijkstra;
use sscanf::scanf;

struct Map {
    walls: Vec<(i64, i64)>,
    max_row: i64,
    max_col: i64,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let walls = inp
            .lines()
            .map(|l| {
                let (col, row) = scanf!(l, "{i64},{i64}").unwrap();
                (row, col)
            })
            .collect::<Vec<_>>();
        let max_row = if walls.len() < 30 { 6 } else { 70 };
        let max_col = max_row;
        Self {
            walls,
            max_row,
            max_col,
        }
    }

    fn cost(&self, len: usize) -> Option<usize> {
        let walls = self.walls.iter().copied().take(len).collect();
        dijkstra(
            &(0, 0),
            |&p| self.succ(p, &walls),
            |&p| p == (self.max_row, self.max_col),
        )
        .map(|(_, cost)| cost)
    }

    fn succ(
        &self,
        (row, col): (i64, i64),
        walls: &HashSet<(i64, i64)>,
    ) -> Vec<((i64, i64), usize)> {
        let mut ret = vec![];
        if row > 0 {
            ret.push((row - 1, col));
        }
        if row < self.max_row {
            ret.push((row + 1, col));
        }
        if col > 0 {
            ret.push((row, col - 1));
        }
        if col < self.max_col {
            ret.push((row, col + 1));
        }
        ret.into_iter()
            .filter_map(|p| {
                if walls.contains(&p) {
                    None
                } else {
                    Some((p, 1))
                }
            })
            .collect()
    }
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    map.cost(if map.max_row == 6 { 12 } else { 1024 }).unwrap()
}

fn part2(inp: &str) -> String {
    let map = Map::parse(inp);
    let idx = (0..map.walls.len()).collect::<Vec<_>>();
    let p = idx.partition_point(|&len| map.cost(len).is_some());
    let (row, col) = map.walls[p - 1];
    format!("{},{}", col, row)
}

xaoc::xaoc!(
    sample = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
);
