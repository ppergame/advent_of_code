use primal_sieve::Sieve;
use sscanf::scanf;
use std::collections::HashMap;

type Reg = char;

#[derive(sscanf::FromScanf, Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    #[sscanf(format = "{}")]
    Imm(i64),
    #[sscanf(format = "{}")]
    Reg(Reg),
}

#[derive(sscanf::FromScanf, Copy, Clone, Debug, PartialEq, Eq)]
enum Cmd {
    #[sscanf(format = "set {} {}")]
    Set(Reg, Op),
    #[sscanf(format = "sub {} {}")]
    Sub(Reg, Op),
    #[sscanf(format = "mul {} {}")]
    Mul(Reg, Op),
    #[sscanf(format = "jnz {} {}")]
    Jnz(Op, Op),
}

struct Machine {
    regs: HashMap<char, i64>,
    pc: i64,
    prog: Vec<Cmd>,
    mul_count: i64,
}

impl Machine {
    fn parse(inp: &str) -> Self {
        let prog = inp
            .lines()
            .map(|line| scanf!(line, "{}", Cmd).unwrap())
            .collect();
        Machine {
            regs: HashMap::new(),
            pc: 0,
            prog,
            mul_count: 0,
        }
    }

    fn val(&self, val: Op) -> i64 {
        match val {
            Op::Reg(r) => self.regs.get(&r).copied().unwrap_or(0),
            Op::Imm(i) => i,
        }
    }

    fn step(&mut self) -> bool {
        let pc = self.pc;
        self.pc += 1;
        let cmd = self.prog[pc as usize];
        match cmd {
            Cmd::Set(x, y) => {
                self.regs.insert(x, self.val(y));
            }
            Cmd::Sub(x, y) => {
                self.regs.insert(x, self.val(Op::Reg(x)) - self.val(y));
            }
            Cmd::Mul(x, y) => {
                self.mul_count += 1;
                self.regs.insert(x, self.val(Op::Reg(x)) * self.val(y));
            }
            Cmd::Jnz(x, y) => {
                if self.val(x) != 0 {
                    self.pc = pc + self.val(y);
                }
            }
        }
        self.pc < 0 || self.pc >= self.prog.len() as i64
    }
}

fn part1(inp: &str) -> i64 {
    let mut m = Machine::parse(inp);
    while !m.step() {}
    m.mul_count
}

static TEMPLATE: &str = "set b INP
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23";

fn prog(arg: usize) -> usize {
    let mut h = 0;
    let b = arg * 100 + 100000;
    let c = b + 17000;
    let sieve = Sieve::new(c);
    for b in (b..=c).step_by(17) {
        if !sieve.is_prime(b) {
            h += 1;
        }
    }
    h
}

fn part2(inp: &str) -> usize {
    let arg = scanf!(inp.lines().next().unwrap(), "set b {}", usize).unwrap();
    let s = TEMPLATE.replace("INP", &arg.to_string());
    assert_eq!(s, inp);
    prog(arg)
}

xaoc::xaoc!(no_sample = true);
