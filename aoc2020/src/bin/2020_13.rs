fn part1(inp: &str) -> usize {
    let mut lines = inp.lines();
    let earliest: usize = lines.next().unwrap().parse().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|id| *id != "x")
        .map(|id| id.parse().unwrap())
        .collect::<Vec<usize>>();
    let (departure, id) = ids
        .iter()
        .map(|id| {
            let rem = earliest % id;
            if rem == 0 {
                (earliest, id)
            } else {
                (earliest - rem + id, id)
            }
        })
        .min_by_key(|(departure, _)| *departure)
        .unwrap();
    (departure - earliest) * id
}

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

fn part2(inp: &str) -> i128 {
    let mut lines = inp.lines();
    let _: i128 = lines.next().unwrap().parse().unwrap();
    let ids = lines.next().unwrap().split(',').map(|id| {
        if id == "x" {
            None
        } else {
            Some(id.parse::<i128>().unwrap())
        }
    });
    let (u, m): (Vec<i128>, Vec<i128>) = ids
        .enumerate()
        .filter_map(|(i, id)| id.map(|id| ((0 - (i as i128)).rem_euclid(id), id)))
        .unzip();
    let a = chinese_remainder(&u, &m).unwrap();

    for (u, m) in u.iter().zip(m.iter()) {
        assert_eq!((a - u) % m, 0);
    }
    a
}

xaoc::xaoc!();
