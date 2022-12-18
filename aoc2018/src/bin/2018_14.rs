use itertools::Itertools;

fn part1(inp: &str) -> String {
    let inp = inp.parse::<usize>().unwrap();
    let mut rec = vec!['3', '7'];
    let mut p1 = 0;
    let mut p2 = 1;
    while rec.len() < inp + 10 {
        let d1 = rec[p1].to_digit(10).unwrap() as usize;
        let d2 = rec[p2].to_digit(10).unwrap() as usize;
        let next = d1 + d2;
        rec.extend(next.to_string().chars());
        p1 = (p1 + d1 + 1) % rec.len();
        p2 = (p2 + d2 + 1) % rec.len();
    }
    rec[inp..inp + 10].iter().collect()
}

fn part2(inp: &str) -> usize {
    let patt = inp.chars().collect_vec();
    let mut rec = vec!['3', '7'];
    let mut p1 = 0;
    let mut p2 = 1;
    loop {
        let d1 = rec[p1].to_digit(10).unwrap() as usize;
        let d2 = rec[p2].to_digit(10).unwrap() as usize;
        let next = d1 + d2;
        rec.extend(next.to_string().chars());
        for l in [0, 1] {
            if let Some(s) = rec.get(rec.len() - patt.len() - l..rec.len() - l) {
                if s == patt {
                    return rec.len() - patt.len() - l;
                }
            }
        }
        p1 = (p1 + d1 + 1) % rec.len();
        p2 = (p2 + d2 + 1) % rec.len();
    }
}

xaoc::xaoc!(sample = "2018", sample2 = "01245");
