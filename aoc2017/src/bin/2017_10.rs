use itertools::Itertools;

fn part1(inp: &str) -> i64 {
    let mut marks = 256;
    if inp.len() < 20 {
        marks = 5;
    }
    let mut list = (0..marks).collect_vec();
    let mut pos = 0;
    for (skip, len) in inp
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .enumerate()
    {
        list[..len].reverse();
        pos = (pos + len + skip) % list.len();
        let list_len = list.len();
        list.rotate_left((len + skip) % list_len);
    }
    list.rotate_right(pos);
    list[0] * list[1]
}

fn part2(inp: &str) -> String {
    let mut inp = inp.chars().map(|c| c as usize).collect_vec();
    inp.extend([17, 31, 73, 47, 23]);
    let mut list = (0..=255).collect_vec();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for &len in &inp {
            list[..len].reverse();
            pos = (pos + len + skip) % list.len();
            let list_len = list.len();
            list.rotate_left((len + skip) % list_len);
            skip += 1;
        }
    }
    list.rotate_right(pos);
    let v = list
        .chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .collect_vec();
    hex::encode(v)
}

xaoc::xaoc!(sample = "3, 4, 1, 5");
