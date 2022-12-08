use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Player {
    pos: i64,
    score: i64,
}

impl Player {
    fn turn(&mut self, roll: i64) {
        self.pos = (self.pos - 1 + roll) % 10 + 1;
        self.score += self.pos;
    }
}

#[derive(Debug, Clone, Copy)]
struct Die {
    next: i64,
    rolls: i64,
}

impl Die {
    fn new() -> Self {
        Self { next: 1, rolls: 0 }
    }

    fn roll(&mut self) -> i64 {
        let ret = self.next;
        self.next += 1;
        if self.next == 101 {
            self.next = 1;
        }
        self.rolls += 1;
        ret
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Game {
    p1: Player,
    p2: Player,
    // false: p1, true: p2
    turn: bool,
    rolls: Vec<i64>,
}

impl Game {
    fn turn(&mut self, roll: i64) {
        self.rolls.push(roll);
        if self.rolls.len() < 3 {
            return;
        }
        let roll = self.rolls.drain(..).sum();
        if !self.turn {
            self.p1.turn(roll);
        } else {
            self.p2.turn(roll);
        }
        self.turn = !self.turn;
    }

    fn check_p1(&self) -> Option<i64> {
        if self.p1.score >= 1000 {
            return Some(self.p2.score);
        }
        if self.p2.score >= 1000 {
            return Some(self.p1.score);
        }
        None
    }

    fn check_p2(&self) -> Option<bool> {
        if self.p1.score >= 21 {
            return Some(false);
        }
        if self.p2.score >= 21 {
            return Some(true);
        }
        None
    }
}

fn parse(inp: &str) -> Game {
    let mut iter = inp
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap());
    Game {
        p1: Player {
            pos: iter.next().unwrap(),
            score: 0,
        },
        p2: Player {
            pos: iter.next().unwrap(),
            score: 0,
        },
        turn: false,
        rolls: vec![],
    }
}

fn part1(inp: &str) -> i64 {
    let mut game = parse(inp);
    let mut die = Die::new();
    loop {
        if let Some(score) = game.check_p1() {
            return score * die.rolls;
        }
        game.turn(die.roll());
    }
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let mut memo = HashMap::new();
    let mut stack = vec![inp.clone()];
    while let Some(game) = stack.pop() {
        if let Some(win) = game.check_p2() {
            if !win {
                memo.insert(game, (1, 0));
            } else {
                memo.insert(game, (0, 1));
            }
            continue;
        }
        let children = (1..=3)
            .map(|roll| {
                let mut game = game.clone();
                game.turn(roll);
                game
            })
            .collect::<Vec<_>>();
        let missing = children
            .iter()
            .cloned()
            .filter(|ch| !memo.contains_key(ch))
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            stack.push(game);
            stack.extend(missing);
            continue;
        }
        let (mut p1win, mut p2win) = (0, 0);
        for ch in children {
            let (a, b) = memo[&ch];
            p1win += a;
            p2win += b;
        }
        memo.insert(game, (p1win, p2win));
    }
    memo[&inp].0.max(memo[&inp].1)
}

xaoc::xaoc!();
