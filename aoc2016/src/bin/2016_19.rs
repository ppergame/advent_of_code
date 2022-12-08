fn part1(inp: &str) -> usize {
    let inp = inp.parse().unwrap();
    let mut elves = vec![0];
    elves.extend(2..=inp);
    elves.push(1);
    let mut next = 1;
    while elves[next] != next {
        elves[next] = elves[elves[next]];
        next = elves[next];
    }
    next
}

fn part2(inp: &str) -> usize {
    let inp = inp.parse().unwrap();
    let mut elves = vec![0];
    elves.extend(2..=inp);
    elves.push(1);
    let mut next = 1;
    let mut across = inp / 2;
    let mut advance = inp % 2 == 1;
    while elves[next] != next {
        elves[across] = elves[elves[across]];
        next = elves[next];
        if advance {
            across = elves[across];
        }
        advance = !advance;
    }
    elves[next]
}

xaoc::xaoc!(sample = "5");
