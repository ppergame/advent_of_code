fn part1(inp: &str) -> i64 {
    let mut lines = inp.lines();
    let mut parse = || {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
    };
    let times = parse();
    let dists = parse();
    let mut ret = 1;
    for (time, dist) in times.zip(dists) {
        let mut wins = 0;
        for hold_t in 1..time {
            let remain_t = time - hold_t;
            let travel = hold_t * remain_t;
            if travel > dist {
                wins += 1;
            }
        }
        ret *= wins;
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let mut lines = inp.lines();
    let mut parse = || {
        lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse::<i64>()
            .unwrap()
    };
    let time = parse();
    let dist = parse();
    let mut wins = 0;
    for hold_t in 1..time {
        let remain_t = time - hold_t;
        let travel = hold_t * remain_t;
        if travel > dist {
            wins += 1;
        }
    }
    wins
}

xaoc::xaoc!();
