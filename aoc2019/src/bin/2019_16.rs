fn patt(num: usize) -> impl Iterator<Item = i8> {
    let i1 = std::iter::repeat(0).take(num);
    let i2 = std::iter::repeat(1).take(num);
    let i3 = std::iter::repeat(0).take(num);
    let i4 = std::iter::repeat(-1).take(num);
    i1.chain(i2).chain(i3).chain(i4).cycle().skip(1)
}

fn phase(sig: &[u8]) -> Vec<u8> {
    let mut ret = Vec::with_capacity(sig.len());
    for i in 1..=sig.len() {
        let mut acc: i32 = 0;
        for (e, m) in sig.iter().zip(patt(i)) {
            acc += *e as i32 * m as i32
        }
        ret.push((acc.abs() % 10) as u8);
    }
    ret
}

fn coll_digits(sig: &[u8], offset: usize, size: usize) -> usize {
    sig[offset..offset + size]
        .iter()
        .fold(0, |x, y| x * 10 + *y as usize)
}

fn part1(inp: &str) -> usize {
    let mut sig = inp
        .chars()
        .map(|i| i.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    for _ in 1..=100 {
        sig = phase(&sig);
    }
    coll_digits(&sig, 0, 8)
}

fn part2(inp: &str) -> usize {
    let mut sig = inp
        .chars()
        .map(|i| i.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    let len = sig.len();
    sig = sig.into_iter().cycle().take(len * 10000).collect();
    let offset = coll_digits(&sig, 0, 7);

    assert!(offset > len / 2);

    for _ in 1..=100 {
        let mut acc = 0;
        for i in (sig.len() / 2..sig.len()).rev() {
            acc = (acc + sig[i]) % 10;
            sig[i] = acc;
        }
    }

    coll_digits(&sig, offset, 8)
}

xaoc::xaoc!();
