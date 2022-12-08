use hex::ToHex;
use pathfinding::directed::dijkstra::dijkstra;
use xaoc::md5;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    row: i64,
    col: i64,
    path: String,
}

fn is_open(c: char) -> bool {
    match c {
        '0'..='9' | 'a' => false,
        'b'..='f' => true,
        _ => unreachable!(),
    }
}

impl State {
    fn succ(&self) -> Vec<(State, i64)> {
        self.succ2().into_iter().map(|state| (state, 1)).collect()
    }

    fn succ2(&self) -> Vec<State> {
        let mut ret = vec![];
        let s = (&md5(&self.path)[0..2]).encode_hex::<Vec<char>>();
        if is_open(s[0]) && self.row > 0 {
            let mut next = self.clone();
            next.row -= 1;
            next.path.push('U');
            ret.push(next);
        }
        if is_open(s[1]) && self.row < 3 {
            let mut next = self.clone();
            next.row += 1;
            next.path.push('D');
            ret.push(next);
        }
        if is_open(s[2]) && self.col > 0 {
            let mut next = self.clone();
            next.col -= 1;
            next.path.push('L');
            ret.push(next);
        }
        if is_open(s[3]) && self.col < 3 {
            let mut next = self.clone();
            next.col += 1;
            next.path.push('R');
            ret.push(next);
        }
        ret
    }
}

fn part1(inp: &str) -> String {
    let initial = State {
        row: 0,
        col: 0,
        path: inp.to_string(),
    };
    dijkstra(
        &initial,
        |state| state.succ(),
        |state| state.row == 3 && state.col == 3,
    )
    .unwrap()
    .0
    .last()
    .unwrap()
    .path
    .chars()
    .skip(inp.len())
    .collect()
}

fn part2(inp: &str) -> usize {
    let initial = State {
        row: 0,
        col: 0,
        path: inp.to_string(),
    };
    let mut max = 0;
    let mut stack: Vec<State> = vec![initial];
    while let Some(state) = stack.pop() {
        if state.row == 3 && state.col == 3 {
            max = max.max(state.path.len());
        } else {
            stack.extend(state.succ2());
        }
    }
    max - inp.len()
}

xaoc::xaoc!(sample = "ihgpwlah");
