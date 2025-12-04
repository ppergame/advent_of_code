use hashbrown::HashSet;

struct Map {
    map: HashSet<(i64, i64)>,
}

impl Map {
    fn new(inp: &str) -> Self {
        let mut map = HashSet::new();
        for (row, l) in inp.lines().enumerate() {
            for (col, c) in l.chars().enumerate() {
                if c == '@' {
                    map.insert((row as i64, col as i64));
                }
            }
        }
        Self { map }
    }

    fn cadj(&self, (row, col): (i64, i64)) -> usize {
        adj((row, col))
            .filter(|(nr, nc)| self.map.contains(&(*nr, *nc)))
            .count()
    }

    fn adj(&self, (row, col): (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
        adj((row, col)).filter(|(nr, nc)| self.map.contains(&(*nr, *nc)))
    }
}

fn adj((row, col): (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .into_iter()
}

fn part1(inp: &str) -> usize {
    let map = Map::new(inp);
    map.map
        .iter()
        .filter(|(row, col)| {
            adj((*row, *col))
                .filter(|(nr, nc)| map.map.contains(&(*nr, *nc)))
                .count()
                < 4
        })
        .count()
}

fn part2(inp: &str) -> usize {
    let mut map = Map::new(inp);
    let mut todo = map
        .map
        .iter()
        .filter(|(row, col)| map.cadj((*row, *col)) < 4)
        .cloned()
        .collect::<Vec<_>>();
    let mut count = 0;
    while let Some((row, col)) = todo.pop() {
        if !map.map.remove(&(row, col)) {
            continue;
        }
        count += 1;
        for (nr, nc) in map.adj((row, col)) {
            if map.cadj((nr, nc)) < 4 {
                todo.push((nr, nc));
            }
        }
    }
    count
}

xaoc::xaoc!(
    sample = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
);
