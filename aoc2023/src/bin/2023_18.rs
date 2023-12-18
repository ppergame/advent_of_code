use itertools::Itertools as _;
use sscanf::scanf;

fn solve(verts: impl Iterator<Item = (char, i64)>) -> i64 {
    let mut prow = 0;
    let mut pcol = 0;
    let mut area = 0;
    let mut len = 0;
    for (d, n) in verts {
        let (drow, dcol) = match d {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => panic!("Unknown direction {}", d),
        };
        len += n;
        let row = prow + drow * n;
        let col = pcol + dcol * n;
        area -= prow * col;
        area += row * pcol;
        (prow, pcol) = (row, col);
    }
    let area = area / 2;
    let inner = area - len / 2 + 1;
    inner + len
}

fn part1(inp: &str) -> i64 {
    let iter = inp.lines().map(|line| {
        let (d, n, _) = scanf!(line, "{char} {i64} (#{str})").unwrap();
        (d, n)
    });
    solve(iter)
}

fn part2(inp: &str) -> i64 {
    let iter = inp.lines().map(|line| {
        let (_, _, hex) = scanf!(line, "{char} {i64} (#{str})").unwrap();
        let mut cc = hex.chars();
        let n = i64::from_str_radix(&cc.by_ref().take(5).join(""), 16).unwrap();
        let d = match cc.next().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => unreachable!(),
        };
        assert!(cc.next().is_none());
        (d, n)
    });
    solve(iter)
}

xaoc::xaoc!(
    sample = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
);
