fn part1(inp: &str) -> i32 {
    let exps = inp
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    for i in &exps {
        for j in &exps {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    unreachable!();
}

fn part2(inp: &str) -> i32 {
    let exps = inp
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    for i in &exps {
        for j in &exps {
            for k in &exps {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    unreachable!();
}

xaoc::xaoc!();
