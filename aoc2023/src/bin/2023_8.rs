use sscanf::scanf;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let mut lines = inp.lines();
    let instr = lines.next().unwrap().chars().cycle();
    assert!(lines.next().unwrap().is_empty());
    let mut map = HashMap::new();
    for line in lines {
        let (cur, left, right) = scanf!(line, "{str} = ({str}, {str})").unwrap();
        map.insert(cur, (left, right));
    }
    let nav = Nav::new("AAA", &map, instr);
    for (steps, pos) in nav.enumerate() {
        if pos == "ZZZ" {
            return steps + 1;
        }
    }
    unreachable!();
}

fn part2(inp: &str) -> usize {
    let mut lines = inp.lines();
    let line = lines.next().unwrap();
    let instr = line.chars().cycle();
    assert!(lines.next().unwrap().is_empty());
    let mut map = HashMap::new();
    for line in lines {
        let (cur, left, right) = scanf!(line, "{str} = ({str}, {str})").unwrap();
        map.insert(cur, (left, right));
    }
    let mut cycles = vec![];
    for start in map.keys().filter(|k| k.ends_with('A')) {
        let mut iter = Nav::new(start, &map, instr.clone());
        for (steps, pos) in (&mut iter).enumerate() {
            if pos.ends_with('Z') {
                cycles.push(steps + 1);
                break;
            }
        }
    }
    lcm(&cycles)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

struct Nav<'a, I>
where
    I: Iterator<Item = char> + 'a,
{
    pos: &'a str,
    map: &'a HashMap<&'a str, (&'a str, &'a str)>,
    instr: I,
}

impl<'a, I> Nav<'a, I>
where
    I: Iterator<Item = char> + 'a,
{
    fn new(pos: &'a str, map: &'a HashMap<&'a str, (&'a str, &'a str)>, instr: I) -> Self {
        Self { pos, map, instr }
    }
}

impl<'a, I> Iterator for Nav<'a, I>
where
    I: Iterator<Item = char> + 'a,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.map.get(self.pos).unwrap();
        self.pos = match self.instr.next().unwrap() {
            'L' => val.0,
            'R' => val.1,
            _ => unreachable!(),
        };
        Some(self.pos)
    }
}

xaoc::xaoc!(
    sample = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    sample2 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
);
