use std::collections::HashMap;

fn parse_orbits(s: &str) -> HashMap<&str, &str> {
    let mut ret = HashMap::new();
    for l in s.split_whitespace() {
        let sp = l.split(')').collect::<Vec<&str>>();
        ret.insert(sp[1], sp[0]);
    }
    ret
}

fn calc_depth<'a>(
    s: &'a str,
    orbits: &HashMap<&str, &'a str>,
    depths: &mut HashMap<&'a str, i32>,
) -> i32 {
    if let Some(v) = depths.get(s) {
        return *v;
    }
    let center = orbits.get(s).unwrap();
    let depth = calc_depth(center, orbits, depths) + 1;
    depths.insert(s, depth);
    depth
}

fn part1(inp: &str) -> i32 {
    let orbits = parse_orbits(inp);
    let mut depths = HashMap::<&str, i32>::new();
    depths.insert("COM", 0);
    let mut total = 0;
    for orbiter in orbits.keys() {
        total += calc_depth(orbiter, &orbits, &mut depths);
    }
    total
}

fn get_path<'a>(orbits: &HashMap<&str, &'a str>, from: &'a str, to: &str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    let mut at = from;
    loop {
        if let Some(prev) = orbits.get(at) {
            at = prev;
            ret.push(at);
        } else {
            panic!("ran out of path");
        }
        if at == to {
            break;
        }
    }
    ret.reverse();
    ret
}

fn part2(inp: &str) -> usize {
    let orbits = parse_orbits(inp);
    let p1 = get_path(&orbits, "YOU", "COM");
    let p2 = get_path(&orbits, "SAN", "COM");
    for (pos, (v1, v2)) in p1.iter().zip(p2.iter()).enumerate() {
        if v1 != v2 {
            return p1.len() - pos + p2.len() - pos;
        }
    }
    unimplemented!();
}

xaoc::xaoc!();
