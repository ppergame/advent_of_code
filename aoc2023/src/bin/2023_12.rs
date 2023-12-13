use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum State {
    Good,
    Bad,
    Unknown,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Good => write!(f, "."),
            State::Bad => write!(f, "#"),
            State::Unknown => write!(f, "?"),
        }
    }
}

fn arrangements(l: &str, rep: usize) -> usize {
    let (map, runs) = l.split_once(' ').unwrap();
    let runs = vec![runs; rep]
        .join(",")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let v = vec![map; rep]
        .join("?")
        .chars()
        .map(|c| match c {
            '.' => State::Good,
            '#' => State::Bad,
            '?' => State::Unknown,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut memo = Memo {
        memo: HashMap::new(),
        v,
        runs,
    };
    memo.arr(memo.v[0], 1, 0)
}

struct Memo {
    memo: HashMap<(State, usize, usize), usize>,
    v: Vec<State>,
    runs: Vec<usize>,
}

impl Memo {
    fn pop(&self, v_idx: &mut usize) -> Option<State> {
        self.v.get(*v_idx).map(|v| {
            *v_idx += 1;
            *v
        })
    }

    fn arr(&mut self, first: State, mut v_idx: usize, mut runs_idx: usize) -> usize {
        if let Some(res) = self.memo.get(&(first, v_idx, runs_idx)) {
            return *res;
        }
        match first {
            State::Bad => {
                let Some(&run) = self.runs.get(runs_idx) else {
                    return 0;
                };
                runs_idx += 1;
                for _ in 0..run - 1 {
                    if self.pop(&mut v_idx).unwrap_or(State::Good) == State::Good {
                        return 0;
                    }
                }
                if self.pop(&mut v_idx) == Some(State::Bad) {
                    return 0;
                }
            }
            State::Good => (),
            State::Unknown => {
                let res =
                    self.arr(State::Good, v_idx, runs_idx) + self.arr(State::Bad, v_idx, runs_idx);
                self.memo.insert((first, v_idx, runs_idx), res);
                return res;
            }
        }
        if self.v.len() == v_idx {
            if self.runs.len() == runs_idx {
                return 1;
            } else {
                return 0;
            }
        }
        let first = self.pop(&mut v_idx).unwrap();
        let res = self.arr(first, v_idx, runs_idx);
        self.memo.insert((first, v_idx, runs_idx), res);
        res
    }
}

fn part1(inp: &str) -> usize {
    inp.lines().map(|l| arrangements(l, 1)).sum()
}

fn part2(inp: &str) -> usize {
    inp.lines().map(|l| arrangements(l, 5)).sum()
}

xaoc::xaoc!(
    sample = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
);
