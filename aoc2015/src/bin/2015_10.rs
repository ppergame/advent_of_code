fn part1(inp: &str) -> usize {
    let mut cur = inp.chars().collect::<Vec<_>>();
    for _ in 0..40 {
        let mut next = vec![];
        let mut prev = None;
        let mut count = 0;
        for c in cur {
            if Some(c) == prev {
                count += 1;
            } else {
                if let Some(prev) = prev {
                    next.push(std::char::from_digit(count, 10).unwrap());
                    next.push(prev);
                }
                prev = Some(c);
                count = 1;
            }
        }
        if let Some(prev) = prev {
            next.push(std::char::from_digit(count, 10).unwrap());
            next.push(prev);
        }
        cur = next;
    }
    cur.len()
}

fn part2(inp: &str) -> usize {
    let mut cur = inp.chars().collect::<Vec<_>>();
    for _ in 0..50 {
        let mut next = vec![];
        let mut prev = None;
        let mut count = 0;
        for c in cur {
            if Some(c) == prev {
                count += 1;
            } else {
                if let Some(prev) = prev {
                    next.push(std::char::from_digit(count, 10).unwrap());
                    next.push(prev);
                }
                prev = Some(c);
                count = 1;
            }
        }
        if let Some(prev) = prev {
            next.push(std::char::from_digit(count, 10).unwrap());
            next.push(prev);
        }
        cur = next;
    }
    cur.len()
}

xaoc::xaoc!();
