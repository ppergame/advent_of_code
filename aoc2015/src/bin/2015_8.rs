enum Mode {
    Normal,
    Slash,
    Hex,
    Hex1,
}

fn part1(inp: &str) -> i64 {
    let mut total = 0;
    let mut mem = 0;
    for line in inp.lines() {
        total += 2;
        let line = &line[1..line.len() - 1];
        let mut mode = Mode::Normal;
        for c in line.chars() {
            total += 1;
            match mode {
                Mode::Normal if c == '\\' => mode = Mode::Slash,
                Mode::Normal => mem += 1,
                Mode::Slash => match c {
                    '\\' | '"' => {
                        mem += 1;
                        mode = Mode::Normal;
                    }
                    'x' => mode = Mode::Hex,
                    _ => unreachable!(),
                },
                Mode::Hex => match c {
                    '0'..='9' | 'a'..='f' => mode = Mode::Hex1,
                    _ => unreachable!(),
                },
                Mode::Hex1 => match c {
                    '0'..='9' | 'a'..='f' => {
                        mem += 1;
                        mode = Mode::Normal;
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
    total - mem
}

fn part2(inp: &str) -> i64 {
    let mut mem = 0;
    let mut total = 0;
    for line in inp.lines() {
        total += 2;
        for c in line.chars() {
            mem += 1;
            match c {
                '\\' | '"' => total += 2,
                _ => total += 1,
            }
        }
    }
    total - mem
}

xaoc::xaoc!();
