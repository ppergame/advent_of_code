fn parse(inp: &str) -> Vec<(String, i64)> {
    inp.lines()
        .map(|line| {
            let sp = line.split_once(' ').unwrap();
            (sp.0.to_owned(), sp.1.parse().unwrap())
        })
        .collect()
}

fn part1(inp: &str) -> i64 {
    let cmds = parse(inp);
    let (mut x, mut y) = (0, 0);
    for (cmd, n) in cmds {
        match cmd.as_str() {
            "forward" => x += n,
            "up" => y -= n,
            "down" => y += n,
            _ => unreachable!(),
        }
    }
    x * y
}

fn part2(inp: &str) -> i64 {
    let cmds = parse(inp);
    let mut aim = 0;
    let mut depth = 0;
    let mut x = 0;
    for (cmd, n) in cmds {
        match cmd.as_str() {
            "forward" => {
                x += n;
                depth += aim * n;
            }
            "up" => aim -= n,
            "down" => aim += n,
            _ => unreachable!(),
        }
    }
    x * depth
}

xaoc::xaoc!();
