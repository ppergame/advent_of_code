use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Board {
    nums: Vec<Vec<usize>>,
    marks: Vec<Vec<bool>>,
}

impl Board {
    fn mark(&mut self, num: usize) {
        for (y, row) in self.nums.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if *cell == num {
                    self.marks[y][x] = true;
                }
            }
        }
    }

    fn check(&self) -> bool {
        for y in 0..5 {
            if (0..5).all(|x| self.marks[y][x]) {
                return true;
            }
        }
        for x in 0..5 {
            if (0..5).all(|y| self.marks[y][x]) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        let mut acc = 0;
        assert!(self.check());
        for y in 0..5 {
            for x in 0..5 {
                if !self.marks[y][x] {
                    acc += self.nums[y][x];
                }
            }
        }
        acc
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    calls: Vec<usize>,
    boards: Vec<Board>,
}

fn parse(inp: &str) -> Input {
    let mut chunks = inp.split("\n\n");
    let calls = chunks
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards = vec![];
    for chunk in &mut chunks {
        let mut nums = vec![];
        for line in chunk.lines() {
            nums.push(
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
        boards.push(Board {
            nums,
            marks: vec![vec![false; 5]; 5],
        });
    }
    Input { calls, boards }
}

fn find_winning_board(mut inp: Input) -> (Board, usize) {
    for call in &inp.calls {
        for board in &mut inp.boards {
            board.mark(*call);
            if board.check() {
                return (board.clone(), *call);
            }
        }
    }
    unreachable!();
}

fn part1(inp: &str) -> usize {
    let inp = parse(inp);
    let (board, last_call) = find_winning_board(inp);
    board.score() * last_call
}

fn find_last_winning_board(mut inp: Input) -> (Board, usize) {
    let mut winners = HashSet::new();
    let count = inp.boards.len();
    for call in &inp.calls {
        for (idx, board) in inp.boards.iter_mut().enumerate() {
            board.mark(*call);
            if board.check() {
                if !winners.contains(&idx) && winners.len() + 1 == count {
                    return (board.clone(), *call);
                }
                winners.insert(idx);
            }
        }
    }
    unreachable!();
}

fn part2(inp: &str) -> usize {
    let inp = parse(inp);
    let (board, last_call) = find_last_winning_board(inp);
    board.score() * last_call
}

xaoc::xaoc!();
