use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash)]
enum Rule {
    Or(Vec<usize>, Vec<usize>),
    Seq(Vec<usize>),
    Char(char),
}

fn nsplit(s: &str) -> Vec<usize> {
    s.split(' ').map(|x| x.parse().unwrap()).collect()
}

impl Rule {
    fn parse(s: &str) -> Rule {
        if s.len() == 3 && s.starts_with('"') && s.ends_with('"') {
            let c = s.chars().nth(1).unwrap();
            assert!(c == 'a' || c == 'b');
            return Rule::Char(c);
        }

        if s.contains('|') {
            let (l, r) = s.split_once(" | ").unwrap();
            return Rule::Or(nsplit(l), nsplit(r));
        }

        Rule::Seq(nsplit(s))
    }
}

#[derive(Debug)]
struct Problem {
    rules: HashMap<usize, Rule>,
    cases: HashSet<String>,
}

impl Problem {
    fn parse(inp: &str) -> Problem {
        let mut rules = HashMap::new();
        let mut lines = inp.lines();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let (num, res) = line.split_once(": ").unwrap();
            let num = num.parse().unwrap();
            rules.insert(num, Rule::parse(res));
        }
        let cases = lines.map(|x| x.to_string()).collect();
        Problem { rules, cases }
    }

    fn recurse(
        expo: &HashMap<usize, HashSet<String>>,
        stack: &mut Vec<usize>,
        rr: &[usize],
    ) -> Option<HashSet<String>> {
        let mut need = false;
        for &sr in rr {
            if !expo.contains_key(&sr) {
                need = true;
                stack.push(sr);
            }
        }
        if need {
            None
        } else {
            Some(
                rr.iter()
                    .map(|sr| expo[sr].iter())
                    .multi_cartesian_product()
                    .map(|v| v.iter().cloned().join(""))
                    .collect(),
            )
        }
    }

    fn expo(&self) -> HashMap<usize, HashSet<String>> {
        let mut expo = HashMap::new();
        let mut stack = vec![0];
        while !stack.is_empty() {
            //println!("{:?} {:?}", stack, expo);
            let r = *stack.last().unwrap();
            if expo.contains_key(&r) {
                stack.pop();
                continue;
            }
            match &self.rules[&r] {
                Rule::Or(rr1, rr2) => {
                    let maybe_hs1 = Self::recurse(&expo, &mut stack, rr1);
                    let maybe_hs2 = Self::recurse(&expo, &mut stack, rr2);
                    if let Some(hs1) = maybe_hs1 {
                        if let Some(hs2) = maybe_hs2 {
                            expo.insert(r, hs1.union(&hs2).cloned().collect());
                            stack.pop();
                        }
                    }
                }
                Rule::Seq(rr) => {
                    if let Some(hs) = Self::recurse(&expo, &mut stack, rr) {
                        expo.insert(r, hs);
                        stack.pop();
                    }
                }
                Rule::Char(c) => {
                    expo.insert(r, vec![c.to_string()].into_iter().collect());
                    stack.pop();
                }
            }
        }
        expo
    }
}

fn part1(inp: &str) -> usize {
    let p = Problem::parse(inp);
    let expo = p.expo();
    expo[&0].intersection(&p.cases).count()
}

fn part2(inp: &str) -> usize {
    let p = Problem::parse(inp);
    assert_eq!(p.rules[&0], Rule::Seq(vec![8, 11]));
    let expo = p.expo();
    assert_eq!(
        expo[&42]
            .iter()
            .map(|x| x.len())
            .sorted()
            .dedup()
            .collect::<Vec<_>>(),
        vec![8]
    );
    assert_eq!(
        expo[&31]
            .iter()
            .map(|x| x.len())
            .sorted()
            .dedup()
            .collect::<Vec<_>>(),
        vec![8]
    );
    assert_eq!(expo[&42].intersection(&expo[&31]).count(), 0);
    let mut count = 0;
    let e42 = &expo[&42];
    let e31 = &expo[&31];
    for case in &p.cases {
        if case.len() % 8 != 0 {
            continue;
        }
        let mut pos = 0;
        let mut c42 = 0;
        let mut c31 = 0;
        while pos < case.len() {
            if !e42.contains(&case[pos..pos + 8]) {
                break;
            }
            c42 += 1;
            pos += 8;
        }
        while pos < case.len() {
            if !e31.contains(&case[pos..pos + 8]) {
                break;
            }
            c31 += 1;
            pos += 8;
        }
        if c42 <= c31 || c31 == 0 || pos < case.len() {
            continue;
        }
        count += 1;
    }
    count
}

xaoc::xaoc!();
