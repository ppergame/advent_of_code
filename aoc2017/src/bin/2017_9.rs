fn part1(inp: &str) -> i64 {
    let mut score = 0;
    let mut depth = 0;
    let mut in_garbage = false;
    let mut cancel = false;
    for c in inp.chars() {
        match (in_garbage, cancel) {
            (true, true) => cancel = false,
            (true, false) => match c {
                '>' => in_garbage = false,
                '!' => cancel = true,
                _ => (),
            },
            (false, true) => unreachable!(),
            (false, false) => match c {
                '{' => {
                    depth += 1;
                    score += depth;
                }
                '}' => depth -= 1,
                '<' => in_garbage = true,
                ',' => (),
                _ => unreachable!("{}", c),
            },
        }
    }
    score
}

fn part2(inp: &str) -> i64 {
    let mut count = 0;
    let mut in_garbage = false;
    let mut cancel = false;
    for c in inp.chars() {
        match (in_garbage, cancel) {
            (true, true) => cancel = false,
            (true, false) => match c {
                '>' => in_garbage = false,
                '!' => cancel = true,
                _ => count += 1,
            },
            (false, true) => unreachable!(),
            (false, false) => match c {
                '{' => (),
                '}' => (),
                '<' => in_garbage = true,
                ',' => (),
                _ => unreachable!("{}", c),
            },
        }
    }
    count
}

xaoc::xaoc!(sample = "{{<ab>},{<ab>},{<ab>},{<ab>}}");
