use hashbrown::HashSet;
use itertools::Itertools as _;
use sscanf::scanf;

#[derive(Debug, Clone)]
struct Machine {
    pc: usize,
    a: i64,
    b: i64,
    c: i64,
    prog: Vec<i64>,
    output: Vec<i64>,
}

enum Status {
    Okay,
    Halt,
}

impl Machine {
    fn parse(inp: &str) -> Self {
        let mut it = inp.lines();
        let a = scanf!(it.next().unwrap(), "Register A: {i64}").unwrap();
        let b = scanf!(it.next().unwrap(), "Register B: {i64}").unwrap();
        let c = scanf!(it.next().unwrap(), "Register C: {i64}").unwrap();
        assert!(it.next().unwrap().is_empty());
        let s = scanf!(it.next().unwrap(), "Program: {str}").unwrap();
        let prog = s.split(',').map(|x| x.parse().unwrap()).collect();
        Self {
            pc: 0,
            a,
            b,
            c,
            prog,
            output: vec![],
        }
    }

    fn param(&self) -> i64 {
        self.prog[self.pc + 1]
    }

    fn combo(&self) -> i64 {
        let x = self.param();
        match x {
            0..=3 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Unknown combo {x}"),
        }
    }

    fn step(&mut self) -> Status {
        let mut jumped = false;
        match self.prog[self.pc] {
            // adv
            0 => {
                self.a /= (2i64)
                    .checked_pow(self.combo().try_into().unwrap())
                    .unwrap();
            }
            // bxl
            1 => {
                self.b ^= self.param();
            }
            // bst
            2 => {
                self.b = self.combo() & 0x7;
            }
            // jnz
            3 => {
                if self.a != 0 {
                    self.pc = self.param().try_into().unwrap();
                    jumped = true;
                }
            }
            // bxc
            4 => {
                self.b ^= self.c;
            }
            // out
            5 => {
                self.output.push(self.combo() & 0x7);
            }
            // bdv
            6 => {
                self.b = self.a
                    / (2i64)
                        .checked_pow(self.combo().try_into().unwrap())
                        .unwrap();
            }
            // cdv
            7 => {
                self.c = self.a
                    / (2i64)
                        .checked_pow(self.combo().try_into().unwrap())
                        .unwrap();
            }
            _ => panic!("Unknown opcode {}", self.prog[self.pc]),
        }
        if !jumped {
            self.pc += 2;
        }
        if self.pc >= self.prog.len() {
            Status::Halt
        } else {
            Status::Okay
        }
    }
}

fn output(mut m: Machine) -> Vec<i64> {
    loop {
        match m.step() {
            Status::Okay => (),
            Status::Halt => return m.output,
        }
    }
}

fn part1(inp: &str) -> String {
    let m = Machine::parse(inp);
    output(m).iter().join(",")
}

fn part2(inp: &str) -> i64 {
    let m = Machine::parse(inp);
    let len = m.prog.len();
    let mut stack = vec![(0, 0)];
    let mut answers = HashSet::new();
    while let Some((a, depth)) = stack.pop() {
        if depth == len {
            answers.insert(a);
            continue;
        }
        let new_depth = depth + 1;
        for val in 0..8 {
            let mut m2 = m.clone();
            let new_a = (a << 3) | val;
            m2.a = new_a;
            let out = output(m2);
            if out == m.prog[len - new_depth..len] {
                stack.push((new_a, new_depth));
            }
        }
    }
    answers.into_iter().min().unwrap()
}

xaoc::xaoc!(no_sample = true);
