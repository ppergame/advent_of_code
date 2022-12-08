use std::collections::{HashMap, HashSet};

struct State {
    path: Vec<String>,
    used: HashSet<String>,
    doubled: bool,
}

impl State {
    // returns complete paths
    fn paths(&self, map: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
        let mut ret = vec![];
        for n in &map[self.path.last().unwrap()] {
            let mut doubled = self.doubled;
            if self.used.contains(n) {
                if !doubled && n != "start" {
                    doubled = true;
                } else {
                    continue;
                }
            }
            if n == "end" {
                let mut p = self.path.clone();
                p.push("end".to_owned());
                ret.push(p);
                continue;
            }
            let mut path = self.path.clone();
            path.push(n.to_owned());
            let mut used = self.used.clone();
            if n.chars().all(|c| c.is_lowercase()) {
                used.insert(n.to_owned());
            }
            let next = State {
                path,
                used,
                doubled,
            };
            ret.extend(next.paths(map));
        }
        ret
    }
}

fn parse(inp: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in inp.lines() {
        let (n1, n2) = line.split_once('-').unwrap();
        map.entry(n1.to_owned())
            .or_insert_with(Vec::new)
            .push(n2.to_owned());
        map.entry(n2.to_owned())
            .or_insert_with(Vec::new)
            .push(n1.to_owned());
    }
    map
}

fn part1(inp: &str) -> usize {
    let map = parse(inp);
    let initial = State {
        path: vec!["start".to_owned()],
        used: ["start".to_owned()].into_iter().collect::<HashSet<_>>(),
        doubled: true,
    };
    initial.paths(&map).len()
}

fn part2(inp: &str) -> usize {
    let map = parse(inp);
    let initial = State {
        path: vec!["start".to_owned()],
        used: ["start".to_owned()].into_iter().collect::<HashSet<_>>(),
        doubled: false,
    };
    initial.paths(&map).len()
}

xaoc::xaoc!();
