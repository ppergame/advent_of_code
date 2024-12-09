use sscanf::scanf;

fn parse(inp: &str) -> impl Iterator<Item = (i64, Vec<i64>)> + use<'_> {
    inp.lines().map(|line| {
        let mut it = line.split_whitespace();
        let res = scanf!(it.next().unwrap(), "{}:", i64).unwrap();
        let vals = it.map(|x| x.parse().unwrap()).collect();
        (res, vals)
    })
}

fn validate(goal: i64, partial: i64, remaining: &[i64]) -> bool {
    if partial > goal {
        return false;
    }
    if remaining.is_empty() {
        return partial == goal;
    }
    validate(goal, partial + remaining[0], &remaining[1..])
        || validate(goal, partial * remaining[0], &remaining[1..])
}

fn part1(inp: &str) -> i64 {
    parse(inp)
        .filter_map(|(res, vals)| {
            if validate(res, vals[0], &vals[1..]) {
                Some(res)
            } else {
                None
            }
        })
        .sum()
}

fn validate2(goal: i64, partial: i64, remaining: &[i64]) -> bool {
    if partial > goal {
        return false;
    }
    if remaining.is_empty() {
        return partial == goal;
    }
    if validate2(goal, partial + remaining[0], &remaining[1..]) {
        return true;
    }
    if validate2(goal, partial * remaining[0], &remaining[1..]) {
        return true;
    }
    let shift = remaining[0].checked_ilog10().unwrap_or(0) + 1;
    let partial = partial * 10i64.pow(shift) + remaining[0];
    validate2(goal, partial, &remaining[1..])
}

fn part2(inp: &str) -> i64 {
    parse(inp)
        .filter_map(|(res, vals)| {
            if validate2(res, vals[0], &vals[1..]) {
                Some(res)
            } else {
                None
            }
        })
        .sum()
}

xaoc::xaoc!();
