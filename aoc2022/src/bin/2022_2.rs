use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn part1(inp: &str) -> i64 {
    let mut score = 0;
    for line in inp.lines() {
        if let [p1, p2] = &line.split_ascii_whitespace().collect_vec()[..] {
            let p1 = match p1.chars().next().unwrap() {
                'A' => Move::Rock,
                'B' => Move::Paper,
                'C' => Move::Scissors,
                _ => unreachable!(),
            };
            let p2 = match p2.chars().next().unwrap() {
                'X' => Move::Rock,
                'Y' => Move::Paper,
                'Z' => Move::Scissors,
                _ => unreachable!(),
            };
            score += p2.score();
            score += match (p1, p2) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 6,
                (Move::Rock, Move::Scissors) => 0,
                (Move::Paper, Move::Rock) => 0,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissors) => 6,
                (Move::Scissors, Move::Rock) => 6,
                (Move::Scissors, Move::Paper) => 0,
                (Move::Scissors, Move::Scissors) => 3,
            };
        } else {
            unreachable!();
        }
    }
    score
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn part2(inp: &str) -> i64 {
    let mut score = 0;
    for line in inp.lines() {
        if let [p1, p2] = &line.split_ascii_whitespace().collect_vec()[..] {
            let p1 = match p1.chars().next().unwrap() {
                'A' => Move::Rock,
                'B' => Move::Paper,
                'C' => Move::Scissors,
                _ => unreachable!(),
            };
            let p2 = match p2.chars().next().unwrap() {
                'X' => Outcome::Lose,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => unreachable!(),
            };
            let p2 = match (p1, p2) {
                (Move::Rock, Outcome::Lose) => Move::Scissors,
                (Move::Rock, Outcome::Draw) => Move::Rock,
                (Move::Rock, Outcome::Win) => Move::Paper,
                (Move::Paper, Outcome::Lose) => Move::Rock,
                (Move::Paper, Outcome::Draw) => Move::Paper,
                (Move::Paper, Outcome::Win) => Move::Scissors,
                (Move::Scissors, Outcome::Lose) => Move::Paper,
                (Move::Scissors, Outcome::Draw) => Move::Scissors,
                (Move::Scissors, Outcome::Win) => Move::Rock,
            };
            score += p2.score();
            score += match (p1, p2) {
                (Move::Rock, Move::Rock) => 3,
                (Move::Rock, Move::Paper) => 6,
                (Move::Rock, Move::Scissors) => 0,
                (Move::Paper, Move::Rock) => 0,
                (Move::Paper, Move::Paper) => 3,
                (Move::Paper, Move::Scissors) => 6,
                (Move::Scissors, Move::Rock) => 6,
                (Move::Scissors, Move::Paper) => 0,
                (Move::Scissors, Move::Scissors) => 3,
            };
        } else {
            unreachable!();
        }
    }
    score
}

xaoc::xaoc!(sample_idx = 6);
