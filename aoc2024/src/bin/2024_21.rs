use enum_tools::EnumTools;
use hashbrown::HashMap;
use itertools::Itertools as _;
use num::FromPrimitive as _;
use num_derive::FromPrimitive as FromPrimitiveMacro;
use pathfinding::prelude::{astar_bag, dijkstra};

struct Layout {
    numpad: HashMap<NumpadKey, [Option<NumpadKey>; 4]>,
    dirpad: HashMap<DirpadKey, [Option<DirpadKey>; 4]>,
}

impl Layout {
    fn new() -> Self {
        let numpad = HashMap::from_iter([
            (
                NumpadKey::Seven,
                [None, Some(NumpadKey::Eight), Some(NumpadKey::Four), None],
            ),
            (
                NumpadKey::Eight,
                [
                    None,
                    Some(NumpadKey::Nine),
                    Some(NumpadKey::Five),
                    Some(NumpadKey::Seven),
                ],
            ),
            (
                NumpadKey::Nine,
                [None, None, Some(NumpadKey::Six), Some(NumpadKey::Eight)],
            ),
            (
                NumpadKey::Four,
                [
                    Some(NumpadKey::Seven),
                    Some(NumpadKey::Five),
                    Some(NumpadKey::One),
                    None,
                ],
            ),
            (
                NumpadKey::Five,
                [
                    Some(NumpadKey::Eight),
                    Some(NumpadKey::Six),
                    Some(NumpadKey::Two),
                    Some(NumpadKey::Four),
                ],
            ),
            (
                NumpadKey::Six,
                [
                    Some(NumpadKey::Nine),
                    None,
                    Some(NumpadKey::Three),
                    Some(NumpadKey::Five),
                ],
            ),
            (
                NumpadKey::One,
                [Some(NumpadKey::Four), Some(NumpadKey::Two), None, None],
            ),
            (
                NumpadKey::Two,
                [
                    Some(NumpadKey::Five),
                    Some(NumpadKey::Three),
                    Some(NumpadKey::Zero),
                    Some(NumpadKey::One),
                ],
            ),
            (
                NumpadKey::Three,
                [
                    Some(NumpadKey::Six),
                    None,
                    Some(NumpadKey::A),
                    Some(NumpadKey::Two),
                ],
            ),
            (
                NumpadKey::Zero,
                [Some(NumpadKey::Two), Some(NumpadKey::A), None, None],
            ),
            (
                NumpadKey::A,
                [Some(NumpadKey::Three), None, None, Some(NumpadKey::Zero)],
            ),
        ]);
        let dirpad = HashMap::from_iter([
            (
                DirpadKey::Up,
                [None, Some(DirpadKey::A), Some(DirpadKey::Down), None],
            ),
            (
                DirpadKey::A,
                [None, None, Some(DirpadKey::Right), Some(DirpadKey::Up)],
            ),
            (DirpadKey::Left, [None, Some(DirpadKey::Down), None, None]),
            (
                DirpadKey::Down,
                [
                    Some(DirpadKey::Up),
                    Some(DirpadKey::Right),
                    None,
                    Some(DirpadKey::Left),
                ],
            ),
            (
                DirpadKey::Right,
                [Some(DirpadKey::A), None, None, Some(DirpadKey::Down)],
            ),
        ]);
        Self { numpad, dirpad }
    }

    fn numpad_dir(&self, from: NumpadKey, to: NumpadKey) -> Option<Dir> {
        self.numpad[&from]
            .iter()
            .position(|&d| d == Some(to))
            .and_then(Dir::from_usize)
    }

    fn dirpad_dir(&self, from: DirpadKey, to: DirpadKey) -> Option<Dir> {
        self.dirpad[&from]
            .iter()
            .position(|&d| d == Some(to))
            .and_then(Dir::from_usize)
    }
}

lazy_static::lazy_static! {
    static ref LAYOUT: Layout = Layout::new();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Bots {
    numpad: NumpadKey,
    dirpads: Vec<DirpadKey>,
}

impl Bots {
    fn new(n: usize) -> Self {
        Self {
            numpad: NumpadKey::A,
            dirpads: vec![DirpadKey::A; n],
        }
    }

    fn succ(&self) -> Vec<(Self, u64)> {
        let mut ret = vec![];
        for d_me in Dir::iter() {
            let mut next = self.clone();
            let last = next.dirpads.last_mut().unwrap();
            let Some(d_last) = LAYOUT.dirpad[last][d_me as usize] else {
                continue;
            };
            *last = d_last;
            ret.push((next, 1));
        }
        let mut next = self.clone();
        for i in (0..next.dirpads.len()).rev() {
            match next.dirpads[i].to_dir() {
                Some(d) => {
                    if i == 0 {
                        if let Some(dn) = LAYOUT.numpad[&next.numpad][d as usize] {
                            next.numpad = dn;
                            ret.push((next, 1));
                        }
                    } else if let Some(dn) = LAYOUT.dirpad[&next.dirpads[i - 1]][d as usize] {
                        next.dirpads[i - 1] = dn;
                        ret.push((next, 1));
                    }
                    break;
                }
                None => {
                    if i == 0 {
                        // this will always be a goal condition
                    }
                }
            }
        }
        ret
    }
}

#[derive(Debug, EnumTools, Copy, Clone, FromPrimitiveMacro)]
#[enum_tools(iter)]
#[repr(u8)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
enum NumpadKey {
    Seven,
    Eight,
    Nine,
    Four,
    Five,
    Six,
    One,
    Two,
    Three,
    Zero,
    #[default]
    A,
}

impl NumpadKey {
    fn from_char(c: char) -> Self {
        match c {
            '7' => NumpadKey::Seven,
            '8' => NumpadKey::Eight,
            '9' => NumpadKey::Nine,
            '4' => NumpadKey::Four,
            '5' => NumpadKey::Five,
            '6' => NumpadKey::Six,
            '1' => NumpadKey::One,
            '2' => NumpadKey::Two,
            '3' => NumpadKey::Three,
            '0' => NumpadKey::Zero,
            'A' => NumpadKey::A,
            _ => panic!("invalid numpad key"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
enum DirpadKey {
    Up,
    #[default]
    A,
    Left,
    Down,
    Right,
}

impl DirpadKey {
    fn to_dir(self) -> Option<Dir> {
        match self {
            DirpadKey::Up => Some(Dir::Up),
            DirpadKey::A => None,
            DirpadKey::Left => Some(Dir::Left),
            DirpadKey::Down => Some(Dir::Down),
            DirpadKey::Right => Some(Dir::Right),
        }
    }

    fn from_dir(d: Dir) -> Self {
        match d {
            Dir::Up => DirpadKey::Up,
            Dir::Right => DirpadKey::Right,
            Dir::Down => DirpadKey::Down,
            Dir::Left => DirpadKey::Left,
        }
    }
}

fn part1(inp: &str) -> u64 {
    let mut ret = 0;
    for l in inp.lines() {
        let mut keys = 0;
        let mut bots = Bots::new(2);
        for c in l.chars() {
            let mut goal = Bots::new(2);
            goal.numpad = NumpadKey::from_char(c);
            let (_, cost) = dijkstra(&bots, |b| b.succ(), |b| b == &goal).unwrap();
            keys += cost;
            keys += 1;
            bots = goal;
        }
        ret += l.trim_end_matches('A').parse::<u64>().unwrap() * keys;
    }
    ret
}

#[derive(Default)]
struct Cache {
    cache: HashMap<(DirpadKey, DirpadKey, u64), u64>,
}

impl Cache {
    fn count(&mut self, from: DirpadKey, to: DirpadKey, layers: u64) -> u64 {
        if from == to {
            return 0;
        }
        if let Some(&v) = self.cache.get(&(from, to, layers)) {
            return v;
        }
        let (it, cost) = astar_bag(
            &from,
            |dk| LAYOUT.dirpad[dk].iter().filter_map(|&d| d.map(|d| (d, 1))),
            |_| 0,
            |&dk| dk == to,
        )
        .unwrap();
        let ret = if layers == 0 {
            cost
        } else {
            it.map(|p| {
                let mut cost = 0;
                let mut dkey = DirpadKey::A;
                for (d1, d2) in p.iter().copied().tuple_windows() {
                    let d = LAYOUT.dirpad_dir(d1, d2).unwrap();
                    let next_dkey = DirpadKey::from_dir(d);
                    cost += self.count(dkey, next_dkey, layers - 1);
                    cost += 1;
                    dkey = next_dkey;
                }
                cost += self.count(dkey, DirpadKey::A, layers - 1);
                cost
            })
            .min()
            .unwrap()
        };
        self.cache.insert((from, to, layers), ret);
        ret
    }
}

fn part2(inp: &str) -> u64 {
    let mut cache = Cache::default();
    let mut ret = 0;
    for l in inp.lines() {
        let layers = 25;
        let mut keys = 0;
        let mut key = NumpadKey::A;
        for c in l.chars() {
            let goal = NumpadKey::from_char(c);
            let (it, _) = astar_bag(
                &key,
                |nk| LAYOUT.numpad[nk].iter().filter_map(|&d| d.map(|d| (d, 1))),
                |_| 0,
                |&nk| nk == goal,
            )
            .unwrap();
            keys += it
                .map(|p| {
                    let mut cost = 0;
                    let mut dkey = DirpadKey::A;
                    for (n1, n2) in p.into_iter().tuple_windows() {
                        let d = LAYOUT.numpad_dir(n1, n2).unwrap();
                        let next_dkey = DirpadKey::from_dir(d);
                        cost += cache.count(dkey, next_dkey, layers - 1);
                        cost += 1;
                        dkey = next_dkey;
                    }
                    cost += cache.count(dkey, DirpadKey::A, layers - 1);
                    cost
                })
                .min()
                .unwrap();
            keys += 1;
            key = goal;
        }
        ret += l.trim_end_matches('A').parse::<u64>().unwrap() * keys;
    }
    ret
}

xaoc::xaoc!(
    sample = "029A
980A
179A
456A
379A"
);
