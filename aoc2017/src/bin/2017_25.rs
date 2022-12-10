use sscanf::scanf;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone)]
enum Dir {
    Left = -1,
    Right = 1,
}

struct Rule {
    val: bool,
    dir: Dir,
    next_state: char,
}

struct State {
    rules: [Rule; 2],
}

struct Machine {
    tape: HashSet<i64>,
    cursor: i64,
    state: char,
    states: HashMap<char, State>,
}

impl Machine {
    fn new(initial: char, states: HashMap<char, State>) -> Self {
        Self {
            tape: HashSet::new(),
            cursor: 0,
            state: initial,
            states,
        }
    }

    fn step(&mut self) {
        let rule_idx = self.tape.contains(&self.cursor) as usize;
        let rule = &self.states[&self.state].rules[rule_idx];
        if rule.val {
            self.tape.insert(self.cursor);
        } else {
            self.tape.remove(&self.cursor);
        }
        self.cursor += rule.dir as i64;
        self.state = rule.next_state;
    }
}

const VAL0: &str = "    - Write the value 0.";
const VAL1: &str = "    - Write the value 1.";
const LEFT: &str = "    - Move one slot to the left.";
const RIGHT: &str = "    - Move one slot to the right.";

fn parse_rule<'a>(mut lines: impl Iterator<Item = &'a str>) -> Rule {
    let _ = scanf!(
        lines.next().unwrap(),
        "  If the current value is {}:",
        usize
    );
    let val = match lines.next().unwrap() {
        VAL0 => false,
        VAL1 => true,
        _ => unreachable!(),
    };
    let dir = match lines.next().unwrap() {
        LEFT => Dir::Left,
        RIGHT => Dir::Right,
        _ => unreachable!(),
    };
    let next_state = scanf!(lines.next().unwrap(), "    - Continue with state {}.", char).unwrap();
    Rule {
        val,
        dir,
        next_state,
    }
}

fn part1(inp: &str) -> usize {
    let mut lines = inp.lines();
    let initial = scanf!(lines.next().unwrap(), "Begin in state {}.", char).unwrap();
    let steps = scanf!(
        lines.next().unwrap(),
        "Perform a diagnostic checksum after {} steps.",
        usize
    )
    .unwrap();
    assert_eq!(lines.next().unwrap(), "");
    let mut states = HashMap::new();
    loop {
        let state = scanf!(lines.next().unwrap(), "In state {}:", char).unwrap();
        let rule0 = parse_rule(&mut lines);
        let rule1 = parse_rule(&mut lines);
        states.insert(
            state,
            State {
                rules: [rule0, rule1],
            },
        );
        match lines.next() {
            Some("") => (),
            None => break,
            _ => unreachable!(),
        }
    }
    let mut m = Machine::new(initial, states);
    for _ in 0..steps {
        m.step();
    }
    m.tape.len()
}

fn part2(_inp: &str) -> i64 {
    0
}

xaoc::xaoc!(sample_idx = 5);
