use fancy_regex::Regex;
use std::collections::HashSet;

fn has_dub(s: &str) -> bool {
    let mut prev = None;
    for c in s.chars() {
        if let Some(prevc) = prev {
            if prevc == c {
                return true;
            }
        }
        prev = Some(c);
    }
    false
}

fn part1(inp: &str) -> usize {
    let vowels = "aeiou".chars().collect::<HashSet<char>>();
    let bad_re = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    inp.lines()
        .filter(|line| {
            line.chars().filter(|c| vowels.contains(c)).count() >= 3
                && has_dub(line)
                && !bad_re.is_match(line).unwrap()
        })
        .count()
}

fn part2(inp: &str) -> usize {
    let rep_re = Regex::new(r"(..).*\1").unwrap();
    let xyx_re = Regex::new(r"(.).\1").unwrap();
    inp.lines()
        .filter(|line| rep_re.is_match(line).unwrap() && xyx_re.is_match(line).unwrap())
        .count()
}

xaoc::xaoc!();
