fn parse(inp: &str) -> Vec<Option<i64>> {
    let mut ret = vec![];
    let mut is_file = true;
    let mut id = 0;
    for c in inp.chars() {
        let c = c.to_digit(10).unwrap();
        if is_file {
            for _ in 0..c {
                ret.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..c {
                ret.push(None);
            }
        }
        is_file = !is_file;
    }
    ret
}

fn part1(inp: &str) -> i64 {
    let mut v = parse(inp);
    let mut insert = v.iter().position(|x| x.is_none()).unwrap();
    let mut take = v.iter().rposition(|x| x.is_some()).unwrap();
    while insert < take {
        v.swap(insert, take);
        while v[insert].is_some() {
            insert += 1;
        }
        while v[take].is_none() {
            take -= 1;
        }
    }
    v.iter()
        .enumerate()
        .map(|(i, x)| i as i64 * x.unwrap_or(0))
        .sum()
}

struct Map {
    // size, position
    free: Vec<(usize, usize)>,
    // ID, size, position
    rfiles: Vec<(i64, usize, usize)>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut free = vec![];
        let mut rfiles = vec![];
        let mut is_file = true;
        let mut id = 0;
        let mut idx = 0;
        for c in inp.chars() {
            let c = c.to_digit(10).unwrap();
            if is_file {
                rfiles.push((id, c as usize, idx));
                id += 1;
            } else {
                free.push((c as usize, idx));
            }
            idx += c as usize;
            is_file = !is_file;
        }
        rfiles.reverse();
        Self { free, rfiles }
    }
}

fn part2(inp: &str) -> i64 {
    let mut map = Map::parse(inp);
    for (_, size, pos) in map.rfiles.iter_mut() {
        for (fs, fp) in map.free.iter_mut() {
            if *fs >= *size && *fp < *pos {
                *fs -= *size;
                *pos = *fp;
                *fp += *size;
                break;
            }
        }
    }
    map.rfiles
        .iter()
        .map(|&(id, size, pos)| (pos..pos + size).map(|x| x as i64 * id).sum::<i64>())
        .sum()
}

xaoc::xaoc!();
