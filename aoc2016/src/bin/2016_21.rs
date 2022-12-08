use std::collections::HashMap;

use itertools::Itertools;
use sscanf::scanf;

fn part1(inp: &str) -> String {
    let mut pass = "abcdefgh".chars().collect_vec();
    // let inp = "swap position 4 with position 0
    // swap letter d with letter b
    // reverse positions 0 through 4
    // rotate left 1 step
    // move position 1 to position 4
    // move position 3 to position 0
    // rotate based on position of letter b
    // rotate based on position of letter d";
    for line in inp.lines() {
        let line = line.trim();
        if let Ok((x, y)) = scanf!(line, "swap position {} with position {}", usize, usize) {
            pass.swap(x, y);
        } else if let Ok((x, y)) = scanf!(line, "swap letter {} with letter {}", char, char) {
            let i1 = pass.iter().position(|&c| c == x).unwrap();
            let i2 = pass.iter().position(|&c| c == y).unwrap();
            pass.swap(i1, i2);
        } else if let Ok((x, _)) = scanf!(line, "rotate left {} ste{}", usize, str) {
            pass.rotate_left(x);
        } else if let Ok((x, _)) = scanf!(line, "rotate right {} ste{}", usize, str) {
            pass.rotate_right(x);
        } else if let Ok(x) = scanf!(line, "rotate based on position of letter {}", char) {
            let i = pass.iter().position(|&c| c == x).unwrap();
            let len = pass.len();
            if i >= 4 {
                pass.rotate_right((1 + i + 1) % len);
            } else {
                pass.rotate_right((1 + i) % len);
            }
        } else if let Ok((x, y)) = scanf!(line, "reverse positions {} through {}", usize, usize) {
            pass[x..=y].reverse();
        } else if let Ok((x, y)) = scanf!(line, "move position {} to position {}", usize, usize) {
            let c = pass.remove(x);
            pass.insert(y, c);
        } else {
            unreachable!();
        }
    }
    pass.into_iter().collect()
}

fn part2(inp: &str) -> String {
    let mut pass = "fbgdceah".chars().collect_vec();
    // let mut pass = "fbdecgha".chars().collect_vec();
    // let inp = "swap position 4 with position 0
    // swap letter d with letter b
    // reverse positions 0 through 4
    // rotate left 1 step
    // move position 1 to position 4
    // move position 3 to position 0
    // rotate based on position of letter b
    // rotate based on position of letter d";
    let mut rot_map = HashMap::new();
    for i in 0..pass.len() {
        let mut s = std::iter::repeat('_').take(pass.len()).collect_vec();
        s[i] = 'x';
        let len = pass.len();
        if i >= 4 {
            s.rotate_right((1 + i + 1) % len);
        } else {
            s.rotate_right((1 + i) % len);
        }
        rot_map.insert(i, s.iter().position(|&c| c == 'x').unwrap());
    }
    let rev_rot_map = HashMap::<usize, usize>::from_iter(rot_map.iter().map(|(k, v)| (*v, *k)));
    assert_eq!(rot_map.len(), rev_rot_map.len());
    for line in inp.lines().rev() {
        let line = line.trim();
        if let Ok((x, y)) = scanf!(line, "swap position {} with position {}", usize, usize) {
            pass.swap(x, y);
        } else if let Ok((x, y)) = scanf!(line, "swap letter {} with letter {}", char, char) {
            let i1 = pass.iter().position(|&c| c == x).unwrap();
            let i2 = pass.iter().position(|&c| c == y).unwrap();
            pass.swap(i1, i2);
        } else if let Ok((x, _)) = scanf!(line, "rotate left {} ste{}", usize, str) {
            pass.rotate_right(x);
        } else if let Ok((x, _)) = scanf!(line, "rotate right {} ste{}", usize, str) {
            pass.rotate_left(x);
        } else if let Ok(x) = scanf!(line, "rotate based on position of letter {}", char) {
            let i = pass.iter().position(|&c| c == x).unwrap();
            let len = pass.len();
            pass.rotate_right((len + rev_rot_map[&i] - i) % len);
        } else if let Ok((x, y)) = scanf!(line, "reverse positions {} through {}", usize, usize) {
            pass[x..=y].reverse();
        } else if let Ok((x, y)) = scanf!(line, "move position {} to position {}", usize, usize) {
            let c = pass.remove(y);
            pass.insert(x, c);
        } else {
            unreachable!();
        }
    }
    pass.into_iter().collect()
}

xaoc::xaoc!(no_sample = true);
