use hashbrown::HashMap;

fn seq(row: i64, col: i64) -> Vec<Vec<(i64, i64)>> {
    vec![
        (row..row + 4).map(|r| (r, col)).collect(),
        (row - 4..=row).rev().map(|r| (r, col)).collect(),
        (col..col + 4).map(|c| (row, c)).collect(),
        (col - 4..=col).rev().map(|c| (row, c)).collect(),
        (0..4).map(|i| (row + i, col + i)).collect(),
        (0..4).map(|i| (row + i, col - i)).collect(),
        (0..4).map(|i| (row - i, col + i)).collect(),
        (0..4).map(|i| (row - i, col - i)).collect(),
    ]
}

fn part1(inp: &str) -> i64 {
    let mut map = HashMap::new();
    for (row, line) in inp.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map.insert((row as i64, col as i64), c);
        }
    }
    let mut count = 0;
    for (&(row, col), &c) in &map {
        if c == 'X' {
            for s in seq(row, col) {
                if s.iter()
                    .zip("XMAS".chars())
                    .all(|(a, b)| map.get(a) == Some(&b))
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn seq2(row: i64, col: i64) -> Vec<(i64, i64)> {
    vec![
        (row - 1, col - 1),
        (row - 1, col + 1),
        (row + 1, col - 1),
        (row + 1, col + 1),
    ]
}

fn part2(inp: &str) -> i64 {
    let mut map = HashMap::new();
    for (row, line) in inp.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            map.insert((row as i64, col as i64), c);
        }
    }
    let mut count = 0;
    for (&(row, col), &c) in &map {
        if c == 'A' {
            let s = seq2(row, col)
                .iter()
                .map(|s| map.get(s))
                .collect::<Vec<_>>();
            for p in ["MSMS", "MMSS", "SSMM", "SMSM"] {
                if s.iter().zip(p.chars()).all(|(a, b)| a == &Some(&b)) {
                    count += 1;
                }
            }
        }
    }
    count
}

xaoc::xaoc!(
    sample = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    sample2 = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
);
