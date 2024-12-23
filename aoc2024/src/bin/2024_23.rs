use hashbrown::{HashMap, HashSet};
use itertools::Itertools as _;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tag([char; 2]);

impl Tag {
    fn parse(s: &str) -> Self {
        let mut chars = s.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        assert!(chars.next().is_none());
        Self([a, b])
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

type Map = HashMap<Tag, HashSet<Tag>>;

fn parse(inp: &str) -> Map {
    let mut ret = Map::new();
    for l in inp.lines() {
        let (a, b) = l.split_once('-').unwrap();
        let a = Tag::parse(a);
        let b = Tag::parse(b);
        ret.entry(a).or_default().insert(b);
        ret.entry(b).or_default().insert(a);
    }
    ret
}

fn part1(inp: &str) -> i64 {
    let m = parse(inp);
    let groups = m
        .iter()
        .flat_map(|(k, v)| {
            let mut ret = vec![];
            for (a, b) in v.iter().tuple_combinations() {
                if !m[a].contains(b) {
                    continue;
                }
                let mut candidate = vec![*k, *a, *b];
                candidate.sort_unstable();
                ret.push(candidate);
            }
            ret
        })
        .collect::<HashSet<_>>();

    let mut ret = 0;
    for g in groups {
        if g[0].0[0] == 't' || g[1].0[0] == 't' || g[2].0[0] == 't' {
            ret += 1;
        }
    }
    ret
}

fn solve2(m: &Map, r: HashSet<Tag>, mut p: HashSet<Tag>, mut x: HashSet<Tag>) -> HashSet<Tag> {
    if p.is_empty() && x.is_empty() {
        return r;
    }
    let mut ret = HashSet::new();
    for v in p.clone().into_iter() {
        let mut r1 = r.clone();
        r1.insert(v);
        let p1 = p.clone().intersection(&m[&v]).cloned().collect();
        let x1 = x.clone().intersection(&m[&v]).cloned().collect();
        let cand = solve2(m, r1, p1, x1);
        if cand.len() > ret.len() {
            ret = cand;
        }
        p.remove(&v);
        x.insert(v);
    }
    ret
}

fn part2(inp: &str) -> String {
    let m = parse(inp);
    let ret = solve2(
        &m,
        HashSet::new(),
        m.keys().copied().collect(),
        HashSet::new(),
    );
    ret.into_iter().sorted().join(",")
}

xaoc::xaoc!();
