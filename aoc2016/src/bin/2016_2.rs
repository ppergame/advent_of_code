static KEYPAD: &[&[i64]] = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];

fn part1(inp: &str) -> i64 {
    let mut x = 1i64;
    let mut y = 1i64;
    let mut acc = 0;
    for line in inp.lines() {
        for c in line.chars() {
            let (dx, dy) = match c {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => unreachable!(),
            };
            if (0..3).contains(&(x + dx)) && (0..3).contains(&(y + dy)) {
                x += dx;
                y += dy;
            }
        }
        acc = acc * 10 + KEYPAD[y as usize][x as usize];
    }
    acc
}

static KEYPAD2: &[&[char]] = &[
    &[' ', ' ', '1', ' ', ' '],
    &[' ', '2', '3', '4', ' '],
    &['5', '6', '7', '8', '9'],
    &[' ', 'A', 'B', 'C', ' '],
    &[' ', ' ', 'D', ' ', ' '],
];

fn part2(inp: &str) -> String {
    let mut x = 0i64;
    let mut y = 2i64;
    let mut acc = String::new();
    for line in inp.lines() {
        for c in line.chars() {
            let (dx, dy) = match c {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => unreachable!(),
            };
            let nx = x + dx;
            let ny = y + dy;
            if (0..5).contains(&(x + dx)) && (0..5).contains(&(y + dy)) {
                let nc = KEYPAD2[ny as usize][nx as usize];
                if nc != ' ' {
                    x = nx;
                    y = ny;
                }
            }
        }
        acc.push(KEYPAD2[y as usize][x as usize]);
    }
    acc
}

xaoc::xaoc!();
