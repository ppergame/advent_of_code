use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;
use ndarray::array;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{Add, Sub};

lazy_static! {
    static ref ALL_ROT: Vec<Rotation> = [
        [-1, 0, 0, 0, 0, -1, 0, -1, 0],
        [0, -1, 0, -1, 0, 0, 0, 0, -1],
        [0, 0, -1, 0, -1, 0, -1, 0, 0],
        [-1, 0, 0, 0, -1, 0, 0, 0, 1],
        [0, -1, 0, 0, 0, -1, 1, 0, 0],
        [0, 0, -1, -1, 0, 0, 0, 1, 0],
        [-1, 0, 0, 0, 1, 0, 0, 0, -1],
        [0, -1, 0, 0, 0, 1, -1, 0, 0],
        [0, 0, -1, 1, 0, 0, 0, -1, 0],
        [-1, 0, 0, 0, 0, 1, 0, 1, 0],
        [0, -1, 0, 1, 0, 0, 0, 0, 1],
        [0, 0, -1, 0, 1, 0, 1, 0, 0],
        [1, 0, 0, 0, -1, 0, 0, 0, -1],
        [0, 1, 0, 0, 0, -1, -1, 0, 0],
        [0, 0, 1, -1, 0, 0, 0, -1, 0],
        [1, 0, 0, 0, 0, -1, 0, 1, 0],
        [0, 1, 0, -1, 0, 0, 0, 0, 1],
        [0, 0, 1, 0, -1, 0, 1, 0, 0],
        [1, 0, 0, 0, 0, 1, 0, -1, 0],
        [0, 1, 0, 1, 0, 0, 0, 0, -1],
        [0, 0, 1, 0, 1, 0, -1, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0, 1],
        [0, 1, 0, 0, 0, 1, 1, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 1, 0]
    ]
    .into_iter()
    .map(|a| Rotation(array!(a).into_shape((3, 3)).unwrap()))
    .collect();
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Coord(ndarray::Array1<i64>);

impl Coord {
    fn new(x: i64, y: i64, z: i64) -> Coord {
        Coord(ndarray::arr1(&[x, y, z]))
    }

    fn dot(&self, rot: &Rotation) -> Coord {
        Coord(self.0.dot(&rot.0))
    }
}

impl Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord(self.0.clone() + rhs.0.clone())
    }
}

impl Sub<&Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        Coord(self.0.clone() - rhs.0.clone())
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.to_vec())
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Rotation(ndarray::Array2<i64>);

// impl Rotation {
//     fn all() -> Vec<Rotation> {
//         let mut ret = vec![];
//         for ss in itertools::repeat_n([false, true], 3).multi_cartesian_product() {
//             for pp in (0..=2).permutations(3) {
//                 let rot = ndarray::arr2(
//                     &(0..=2)
//                         .map(|i| {
//                             let mut row = [0, 0, 0];
//                             row[pp[i]] = if ss[i] { 1 } else { -1 };
//                             row
//                         })
//                         .collect::<Vec<_>>(),
//                 );
//                 if rot.map(|x| *x as f64).det().unwrap() < 0.0 {
//                     continue;
//                 }
//                 ret.push(Rotation(rot));
//             }
//         }
//         ret
//     }
// }

impl Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.iter().collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone)]
pub struct Scanner {
    num: usize,
    beacons: Vec<Coord>,
}

impl Scanner {
    fn rotate(&self, rot: &Rotation) -> Self {
        Self {
            num: self.num,
            beacons: self.beacons.iter().map(|coo| coo.dot(rot)).collect(),
        }
    }

    fn shift(&self, shift: &Coord) -> Self {
        Self {
            num: self.num,
            beacons: self.beacons.iter().map(|coo| coo - shift).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    scanners: Vec<Scanner>,
}

fn parse(inp: &str) -> Input {
    let scanners = inp
        .split("\n\n")
        .enumerate()
        .map(|(idx, sc)| {
            let beacons = sc
                .lines()
                .skip(1)
                .map(|line| {
                    let mut sp = line.split(',');
                    Coord::new(
                        sp.next().unwrap().parse().unwrap(),
                        sp.next().unwrap().parse().unwrap(),
                        sp.next().unwrap().parse().unwrap(),
                    )
                })
                .collect();
            Scanner { num: idx, beacons }
        })
        .collect();
    Input { scanners }
}

struct Cache {
    bec_matches: Vec<Vec<Option<(Rotation, Coord)>>>,
    by_rot: Vec<Vec<HashSet<Coord>>>,
}

impl Cache {
    fn new() -> Self {
        Self {
            bec_matches: Vec::new(),
            by_rot: Vec::new(),
        }
    }

    fn init(&mut self, ss: &[Scanner]) {
        self.by_rot = ss
            .par_iter()
            .map(|s| {
                ALL_ROT
                    .par_iter()
                    .map(|rot| HashSet::from_iter(s.rotate(rot).beacons))
                    .collect()
            })
            .collect();
        self.bec_matches = ss
            .par_iter()
            .map(|s1| {
                ss.par_iter()
                    .map(|s2| self.bec_match(s1, s2))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    }

    // (rot, shift) such that b1 = b2 * rot + shift
    fn bec_match(&self, s1: &Scanner, s2: &Scanner) -> Option<(Rotation, Coord)> {
        let mut ret = (0..ALL_ROT.len())
            .into_par_iter()
            .filter_map(|idx| {
                let becs = &self.by_rot[s2.num][idx];
                let shifts = iproduct!(&s1.beacons, becs)
                    .map(|(b1, b2)| b2 - b1)
                    .collect::<HashSet<_>>();
                let mut shifts = shifts
                    .into_iter()
                    .filter(|shift| {
                        s1.beacons
                            .iter()
                            .filter(|bec| becs.contains(&(*bec + shift)))
                            .count()
                            >= 12
                    })
                    .collect::<Vec<_>>();
                match shifts.len() {
                    0 => None,
                    1 => Some((ALL_ROT[idx].clone(), shifts.pop().unwrap())),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();
        match ret.len() {
            0 => None,
            1 => ret.pop(),
            _ => unreachable!(),
        }
    }
}

fn match_all(ss: &[Scanner], cache: &Cache) -> HashMap<usize, (Vec<Rotation>, Vec<Coord>)> {
    let mut rshifts_from0 = HashMap::new();
    rshifts_from0.insert(0, (vec![], vec![]));
    let mut pending = (1..ss.len()).collect::<HashSet<_>>();
    'outer: while !pending.is_empty() {
        for (num, (rots, shifts)) in rshifts_from0.iter_mut() {
            for p in &pending {
                if let Some((rot, shift)) = cache.bec_matches[*num][*p].clone() {
                    let p = *p;
                    let mut rots = rots.clone();
                    rots.push(rot);
                    let mut shifts = shifts.clone();
                    shifts.push(shift);
                    rshifts_from0.insert(p, (rots, shifts));
                    pending.remove(&p);
                    continue 'outer;
                }
            }
        }
        panic!(
            "not found, rshifts {:?} pending {:?}",
            rshifts_from0.keys().sorted().collect::<Vec<_>>(),
            pending.iter().sorted().collect::<Vec<_>>()
        );
    }
    rshifts_from0
}

fn part1(inp: &str) -> usize {
    let inp = parse(inp);
    let mut cache = Cache::new();
    cache.init(&inp.scanners);
    let rshifts_from0 = match_all(&inp.scanners, &cache);
    let mut beacons = HashSet::new();
    for (num, (rots, shifts)) in rshifts_from0 {
        let mut scanner = inp.scanners[num].clone();
        for (rot, shift) in rots.into_iter().zip(shifts).rev() {
            scanner = scanner.rotate(&rot);
            scanner = scanner.shift(&shift);
        }
        for bec in scanner.beacons {
            beacons.insert(bec);
        }
    }
    beacons.len()
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let mut cache = Cache::new();
    cache.init(&inp.scanners);
    let rshifts_from0 = match_all(&inp.scanners, &cache);
    let mut scanners = vec![];
    for (rots, shifts) in rshifts_from0.values() {
        let mut coo = Coord::new(0, 0, 0);
        for (rot, shift) in rots.iter().zip(shifts).rev() {
            coo = coo.dot(rot);
            coo = &coo - shift;
        }
        scanners.push(coo);
    }
    scanners
        .iter()
        .cartesian_product(&scanners)
        .map(|(s1, s2)| (s1 - s2).0.iter().map(|x| x.abs()).sum())
        .max()
        .unwrap()
}

xaoc::xaoc!();
