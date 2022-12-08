use sscanf::scanf;

// disc #A has B positions; at time=0, it is at position C.
// needs to be in position 0 at time X + A
// (C1 + X + A1) % B1 = 0
// (C2 + X + A2) % B2 = 0
// ...

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn part1(inp: &str) -> i128 {
    let mut residues = vec![];
    let mut modulii = vec![];
    for line in inp.lines() {
        let (a, b, t0, c) = scanf!(
            line,
            "Disc #{} has {} positions; at time={}, it is at position {}.",
            i128,
            i128,
            i128,
            i128
        )
        .unwrap();
        assert_eq!(t0, 0);
        residues.push((-a - c).rem_euclid(b));
        modulii.push(b);
    }
    chinese_remainder(&residues, &modulii).unwrap()
}

fn part2(inp: &str) -> i128 {
    let mut residues = vec![];
    let mut modulii = vec![];
    for line in inp.lines() {
        let (a, b, t0, c) = scanf!(
            line,
            "Disc #{} has {} positions; at time={}, it is at position {}.",
            i128,
            i128,
            i128,
            i128
        )
        .unwrap();
        assert_eq!(t0, 0);
        residues.push((-a - c).rem_euclid(b));
        modulii.push(b);
    }
    residues.push((-(residues.len() as i128 + 1)).rem_euclid(11));
    modulii.push(11);
    chinese_remainder(&residues, &modulii).unwrap()
}

xaoc::xaoc!(sample_idx = 6);
