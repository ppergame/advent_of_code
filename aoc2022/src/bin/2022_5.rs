use sscanf::scanf;

fn solve(inp: &str, rev: bool) -> String {
    let mut stacks = vec![vec![]];
    let mut lines = inp.lines();
    for line in &mut lines {
        if line.starts_with(" 1") {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if i % 4 != 1 {
                continue;
            }
            let idx = i / 4 + 1;
            if c == ' ' {
                continue;
            }
            while stacks.len() < idx + 1 {
                stacks.push(vec![]);
            }
            stacks[idx].push(c);
        }
    }
    for v in stacks.iter_mut() {
        v.reverse();
    }
    lines.next();
    for line in lines {
        let (count, from, to) = scanf!(line, "move {} from {} to {}", i64, i64, i64).unwrap();
        let s = &mut stacks[from as usize];
        let mut acc = s.split_off(s.len() - count as usize);
        if rev {
            acc.reverse();
        }
        stacks[to as usize].extend(acc);
    }
    let mut acc = String::new();
    for v in stacks.into_iter().skip(1) {
        acc.push(*v.last().unwrap());
    }
    acc
}

fn part1(inp: &str) -> String {
    solve(inp, true)
}

fn part2(inp: &str) -> String {
    solve(inp, false)
}

xaoc::xaoc!();
