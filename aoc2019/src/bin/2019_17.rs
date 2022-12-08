use aoc2019::intcode::*;
use std::collections::HashMap;

type Coord = (i64, i64);

trait CoordMethods {
    fn neigh(&self) -> Vec<(Dir, Coord)>;
}

impl CoordMethods for Coord {
    fn neigh(&self) -> Vec<(Dir, Coord)> {
        let (x, y) = *self;
        vec![
            (Dir::N, (x, y - 1)),
            (Dir::E, (x + 1, y)),
            (Dir::S, (x, y + 1)),
            (Dir::W, (x - 1, y)),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Dir {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}

impl Dir {
    fn opp(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
    }

    fn delta(&self, c: Coord) -> Coord {
        let (x, y) = c;
        match self {
            Dir::N => (x, y - 1),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x - 1, y),
        }
    }
}

struct Map {
    map: HashMap<Coord, char>,
    #[allow(dead_code)]
    smap: String,
    heading: Dir,
    pos: Coord,
}

impl Map {
    fn new(ic: &mut Intcode) -> Map {
        let (smap, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Halt));
        let mut map = HashMap::<Coord, char>::new();
        let mut heading = None;
        let mut pos = None;
        for (row, line) in smap.lines().enumerate() {
            for (col, mut b) in line.chars().enumerate() {
                let new_heading = match b {
                    '^' => Some(Dir::N),
                    'v' => Some(Dir::S),
                    '<' => Some(Dir::W),
                    '>' => Some(Dir::E),
                    _ => None,
                };
                if new_heading.is_some() {
                    b = '#';
                    heading = new_heading;
                    pos = Some((col as i64, row as i64));
                }
                map.insert((col as i64, row as i64), b);
            }
        }
        Map {
            map,
            smap,
            heading: heading.unwrap(),
            pos: pos.unwrap(),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Turn {
    L,
    R,
}

impl ToString for Turn {
    fn to_string(&self) -> String {
        match self {
            Turn::L => String::from("L"),
            Turn::R => String::from("R"),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Step(Turn, u8);

impl std::fmt::Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.0).field(&self.1).finish()
    }
}

fn angle(d: Dir, newd: Dir) -> Turn {
    match (d, newd) {
        (Dir::N, Dir::W) => Turn::L,
        (Dir::N, Dir::E) => Turn::R,
        (Dir::S, Dir::E) => Turn::L,
        (Dir::S, Dir::W) => Turn::R,
        (Dir::W, Dir::S) => Turn::L,
        (Dir::W, Dir::N) => Turn::R,
        (Dir::E, Dir::N) => Turn::L,
        (Dir::E, Dir::S) => Turn::R,
        _ => panic!("bad turn {:?} {:?}", d, newd),
    }
}

fn part1(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    let map = Map::new(&mut ic);
    let mut align = 0;
    for (c, b) in &map.map {
        if *b == '#'
            && c.neigh()
                .into_iter()
                .all(|(_, c)| *map.map.get(&c).unwrap_or(&'z') == '#')
        {
            align += c.0 * c.1;
        }
    }
    align
}

fn to_ascii(steps: &[Step]) -> String {
    itertools::join(
        steps
            .iter()
            .map(|Step(t, m)| format!("{},{}", t.to_string(), m)),
        ",",
    )
}

fn split_by<'a, T>(haystack: &'a [T], needle: &[T]) -> Vec<&'a [T]>
where
    T: PartialEq,
{
    let mut pieces = Vec::new();
    let mut prev = 0;
    let mut cur = 0;
    while haystack.len() - cur >= needle.len() {
        if haystack[cur..cur + needle.len()] == *needle {
            if cur > prev {
                pieces.push(&haystack[prev..cur]);
            }
            prev = cur + needle.len();
            cur = prev;
            continue;
        }
        cur += 1;
    }
    if cur < haystack.len() {
        pieces.push(&haystack[prev..]);
    }
    pieces
}

fn find_abc(steps: &[Step]) -> (&[Step], &[Step], &[Step]) {
    for alen in (2..=steps.len()).rev() {
        let a = &steps[..alen];
        if to_ascii(a).len() > 20 {
            continue;
        }
        let a_pieces = split_by(steps, a);
        if a_pieces.len() < 2 {
            continue;
        }
        for blen in (2..=a_pieces[0].len()).rev() {
            let b = &a_pieces[0][..blen];
            if to_ascii(b).len() > 20 {
                continue;
            }
            let b_pieces = a_pieces
                .iter()
                .flat_map(|piece| split_by(piece, b))
                .collect::<Vec<&[Step]>>();
            if b_pieces.is_empty() {
                continue;
            }
            if to_ascii(b_pieces[0]).len() > 20 {
                continue;
            }
            if !b_pieces.iter().all(|p| *p == b_pieces[0]) {
                continue;
            }
            return (a, b, b_pieces[0]);
        }
    }
    panic!("did not find");
}

fn part2(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    let mut map = Map::new(&mut ic);

    assert!(matches!(map.heading, Dir::N));

    let mut steps = Vec::new();

    loop {
        let mut iter = map
            .pos
            .neigh()
            .into_iter()
            .filter(|(d, c)| *d != map.heading.opp() && matches!(map.map.get(c), Some('#')));
        let turn = match iter.next() {
            None => break,
            Some((d, _)) => {
                assert!(iter.next().is_none());
                let t = angle(map.heading, d);
                map.heading = d;
                t
            }
        };

        let mut units = 0;
        loop {
            let nextc = map.heading.delta(map.pos);
            match map.map.get(&nextc) {
                Some('#') => {
                    units += 1;
                    map.pos = nextc;
                }
                _ => break,
            }
        }
        assert!(units > 0);
        steps.push(Step(turn, units));
    }

    let (a, b, c) = find_abc(&steps);

    let mut abc = Vec::new();
    let mut cur = 0;
    while cur < steps.len() {
        if &steps[cur..cur + a.len()] == a {
            abc.push("A");
            cur += a.len();
            continue;
        }
        if &steps[cur..cur + b.len()] == b {
            abc.push("B");
            cur += b.len();
            continue;
        }
        if &steps[cur..cur + c.len()] == c {
            abc.push("C");
            cur += c.len();
            continue;
        }
    }

    ic = Intcode::new(inp);
    ic.cs[0] = 2;

    {
        let (output, status) = ic.collect_output();
        // Prints map, "Main:" prompt
        assert!(output.contains("Main:"));
        assert!(matches!(status, IntcodeStatus::Input));
    }
    for c in itertools::join(abc.iter(), ",").chars() {
        ic.input = Some(c as i64);
        let (_, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
    }
    ic.input = Some('\n' as i64);
    {
        let (output, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
        assert!(output.contains("Function A:"));
    }
    for c in to_ascii(a).chars() {
        ic.input = Some(c as i64);
        let (_, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
    }
    ic.input = Some('\n' as i64);
    {
        let (output, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
        assert!(output.contains("Function B:"));
    }
    for c in to_ascii(b).chars() {
        ic.input = Some(c as i64);
        let (_, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
    }
    ic.input = Some('\n' as i64);
    {
        let (output, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
        assert!(output.contains("Function C:"));
    }
    for c in to_ascii(c).chars() {
        ic.input = Some(c as i64);
        let (_, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
    }
    ic.input = Some('\n' as i64);
    {
        let (output, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
        assert!(output.contains("Continuous video feed?"));
    }
    ic.input = Some('n' as i64);
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
    ic.input = Some('\n' as i64);
    let mut result = 0;
    while let IntcodeStatus::Output(output) = ic.run().unwrap() {
        result = output;
    }
    result
}

xaoc::xaoc!();
