fn parse(inp: &str) -> Vec<String> {
    inp.lines().map(|line| line.to_owned()).collect()
}

fn part1(inp: &str) -> i64 {
    let lines = parse(inp);
    let mut count1s = vec![];
    count1s.resize(lines[0].len(), 0);
    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            if c == '1' {
                count1s[idx] += 1;
            }
        }
    }
    let gamma = count1s
        .iter()
        .map(|&c| if c > 500 { '1' } else { '0' })
        .collect::<String>();
    let epsilon = count1s
        .iter()
        .map(|&c| if c > 500 { '0' } else { '1' })
        .collect::<String>();
    i64::from_str_radix(&gamma, 2).unwrap() * i64::from_str_radix(&epsilon, 2).unwrap()
}

fn count1s(lines: &[&[u8]]) -> Vec<usize> {
    let mut count1s = vec![];
    count1s.resize(lines[0].len(), 0);
    for line in lines {
        for (idx, b) in line.iter().enumerate() {
            if *b == b'1' {
                count1s[idx] += 1;
            }
        }
    }
    count1s
}

fn part2(inp: &str) -> i64 {
    let lines = parse(inp);
    let mut uu = lines.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();
    for bit in 0..lines[0].len() {
        let count1s = count1s(&uu);
        let len = uu.len();
        uu.retain(|u| {
            let mc = if count1s[bit] >= len - count1s[bit] {
                b'1'
            } else {
                b'0'
            };
            u[bit] == mc
        });
        if uu.len() == 1 {
            break;
        }
    }
    let mut mm = lines.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();
    for bit in 0..lines[0].len() {
        let count1s = count1s(&mm);
        let len = mm.len();
        mm.retain(|u| {
            let mc = if count1s[bit] >= len - count1s[bit] {
                b'0'
            } else {
                b'1'
            };
            u[bit] == mc
        });
        if mm.len() == 1 {
            break;
        }
    }
    i64::from_str_radix(std::str::from_utf8(mm[0]).unwrap(), 2).unwrap()
        * i64::from_str_radix(std::str::from_utf8(uu[0]).unwrap(), 2).unwrap()
}

xaoc::xaoc!();
