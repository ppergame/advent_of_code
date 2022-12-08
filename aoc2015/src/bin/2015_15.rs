use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INGR_RE: Regex =
        Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)")
            .unwrap();
}

struct Ingr {
    _name: String,
    qual: Vec<i64>,
}

impl Ingr {
    fn calc(&self, amt: i64) -> Vec<i64> {
        self.qual.iter().map(|q| q * amt).collect()
    }
}

fn parse(inp: &str) -> Vec<Ingr> {
    inp.lines()
        .map(|line| {
            let caps = INGR_RE.captures(line).unwrap();
            let qual = caps
                .iter()
                .skip(2)
                .map(|m| m.unwrap().as_str().parse().unwrap())
                .collect();
            Ingr {
                _name: caps.get(1).unwrap().as_str().to_owned(),
                qual,
            }
        })
        .collect()
}

enum Iter<'a> {
    Rec(IterRec<'a>),
    Base(std::iter::Once<Vec<i64>>),
}

struct IterRec<'a> {
    val: i64,
    budget: i64,
    ingrs: &'a [Ingr],
    iter: Option<Box<Iter<'a>>>,
}

impl<'a> Iterator for IterRec<'a> {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.val > self.budget {
                return None;
            }
            match &mut self.iter {
                None => {
                    self.iter = Some(Box::new(Iter::new(
                        self.budget - self.val,
                        &self.ingrs[..self.ingrs.len() - 1],
                    )));
                }
                Some(iter) => match iter.next() {
                    None => {
                        self.val += 1;
                        self.iter = None;
                        continue;
                    }
                    Some(mut next) => {
                        next.push(self.val);
                        return Some(next);
                    }
                },
            }
        }
    }
}

impl<'a> Iter<'a> {
    fn new(budget: i64, ingrs: &'a [Ingr]) -> Self {
        if ingrs.len() == 1 {
            Self::Base(std::iter::once(vec![budget]))
        } else {
            Self::Rec(IterRec {
                val: 0,
                budget,
                ingrs,
                iter: None,
            })
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Rec(iter) => iter.next(),
            Iter::Base(base) => base.next(),
        }
    }
}

fn part1(inp: &str) -> i64 {
    let ingrs = parse(inp);
    Iter::new(100, &ingrs)
        .map(|recipe| {
            let mut scores = recipe
                .into_iter()
                .zip(&ingrs)
                .map(|(n, ingr)| ingr.calc(n))
                .reduce(|i1, i2| i1.iter().zip(i2).map(|(e1, e2)| e1 + e2).collect())
                .unwrap();
            scores.pop();
            scores.iter().map(|&i| i.max(0)).product()
        })
        .max()
        .unwrap()
}

fn part2(inp: &str) -> i64 {
    let ingrs = parse(inp);
    Iter::new(100, &ingrs)
        .filter_map(|recipe| {
            let mut scores = recipe
                .into_iter()
                .zip(&ingrs)
                .map(|(n, ingr)| ingr.calc(n))
                .reduce(|i1, i2| i1.iter().zip(i2).map(|(e1, e2)| e1 + e2).collect())
                .unwrap();
            let calories = scores.pop().unwrap();
            match calories {
                500 => Some(scores.iter().map(|&i| i.max(0)).product()),
                _ => None,
            }
        })
        .max()
        .unwrap()
}

xaoc::xaoc!();
