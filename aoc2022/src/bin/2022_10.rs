use sscanf::scanf;

fn emit1(reg: i64, cycle: i64) -> i64 {
    if (cycle - 20) % 40 == 0 {
        reg * cycle
    } else {
        0
    }
}

fn part1(inp: &str) -> i64 {
    let mut sum = 0;
    let mut cycle = 1;
    let mut reg = 1;
    for line in inp.lines() {
        if line == "noop" {
            cycle += 1;
            sum += emit1(reg, cycle);
        } else if let Ok(n) = scanf!(line, "addx {}", i64) {
            cycle += 1;
            sum += emit1(reg, cycle);
            cycle += 1;
            reg += n;
            sum += emit1(reg, cycle);
        } else {
            unreachable!();
        }
    }
    sum
}

fn emit2(s: &mut String, reg: i64, cycle: i64) {
    let crt_pos = (cycle - 1) % 40;
    if crt_pos == reg - 1 || crt_pos == reg || crt_pos == reg + 1 {
        s.push('█');
    } else {
        s.push(' ');
    }
    if crt_pos == 39 {
        s.push('\n');
    }
}

fn part2(inp: &str) -> String {
    let mut cycle = 1;
    let mut reg = 1;
    let mut s = String::new();
    for line in inp.lines() {
        if line == "noop" {
            emit2(&mut s, reg, cycle);
            cycle += 1;
        } else if let Ok(n) = scanf!(line, "addx {}", i64) {
            emit2(&mut s, reg, cycle);
            cycle += 1;
            emit2(&mut s, reg, cycle);
            cycle += 1;
            reg += n;
        } else {
            unreachable!();
        }
    }
    s
}

xaoc::xaoc!(sample_idx = 30);
