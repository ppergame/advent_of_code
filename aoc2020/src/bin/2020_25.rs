fn parse(inp: &str) -> (usize, usize) {
    let (a, b) = inp.split_once('\n').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn find_loop(subj: usize, res: usize) -> usize {
    let mut l = 0;
    let mut val = 1;
    loop {
        if val == res {
            return l;
        }
        l += 1;
        val = (val * subj) % 20201227;
    }
}

fn transform(subj: usize, l: usize) -> usize {
    let mut val = 1;
    for _ in 0..l {
        val = (val * subj) % 20201227;
    }
    val
}

fn part1(inp: &str) -> usize {
    let (card_pubkey, door_pubkey) = parse(inp);
    let card_loop = find_loop(7, card_pubkey);
    let door_loop = find_loop(7, door_pubkey);
    let ret = transform(card_pubkey, door_loop);
    assert_eq!(ret, transform(door_pubkey, card_loop));
    ret
}

fn part2(_inp: &str) -> usize {
    0
}

xaoc::xaoc!();
