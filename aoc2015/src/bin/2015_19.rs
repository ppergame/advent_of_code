use itertools::Itertools;
use rand::seq::IteratorRandom;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse(inp: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut map = HashMap::new();
    let mut lines = inp.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (src, res) = line.split_once(" => ").unwrap();
        map.entry(src).or_insert_with(Vec::new).push(res);
    }
    let mol = lines.next().unwrap();
    assert!(lines.next().is_none());
    (map, mol)
}

fn part1(inp: &str) -> usize {
    let (map, mol) = parse(inp);
    let re = Regex::new(&("(".to_owned() + &map.keys().map(|k| regex::escape(k)).join("|") + ")"))
        .unwrap();
    let mut seen = HashSet::new();
    for m in re.find_iter(mol) {
        for v in &map[m.as_str()] {
            let s = mol[..m.start()].to_owned() + v + &mol[m.end()..];
            seen.insert(s);
        }
    }
    seen.len()
}

struct Replacer<'a> {
    count: &'a mut usize,
    v: &'a str,
}

impl<'a> regex::Replacer for Replacer<'a> {
    fn replace_append(&mut self, _caps: &regex::Captures<'_>, dst: &mut String) {
        *self.count += 1;
        dst.push_str(self.v);
    }
}

fn part2(inp: &str) -> usize {
    let (fmap, goal) = parse(inp);
    let map = fmap
        .into_iter()
        .flat_map(|(k, v)| v.into_iter().map(move |c| (c, k)))
        .collect::<HashMap<_, _>>();
    let mut re_cache = HashMap::new();
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let mut mol = goal.to_owned();
        let mut count = 0;
        for _ in 0..1000 {
            if mol == "e" {
                return count;
            }
            let k = map.keys().choose(&mut rng).unwrap();
            let v = map.get(k).unwrap();
            let mre_s = regex::escape(k);
            let mre = re_cache
                .entry(k)
                .or_insert_with(|| Regex::new(&mre_s).unwrap());
            mol = mre
                .replace_all(
                    &mol,
                    Replacer {
                        count: &mut count,
                        v,
                    },
                )
                .to_string();
        }
        println!("random walk failed, retrying");
    }
    unreachable!();
}

xaoc::xaoc!();
