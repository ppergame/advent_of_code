struct Machine<'a> {
    a: u64,
    b: u64,
    prog: Vec<&'a str>,
    pc: i64,
}

impl<'a> Machine<'a> {
    fn new(inp: &'a str) -> Self {
        Self {
            a: 0,
            b: 0,
            prog: inp.lines().collect(),
            pc: 0,
        }
    }

    fn reg(&mut self, reg: &str) -> &mut u64 {
        match reg {
            "a" => &mut self.a,
            "b" => &mut self.b,
            _ => unreachable!(),
        }
    }

    fn step(&mut self) -> bool {
        let instr = self.prog[self.pc as usize];
        let sp = instr.split_whitespace().collect::<Vec<_>>();
        if sp[0] == "hlf" {
            let reg = self.reg(sp[1]);
            let val = *reg;
            *reg = val / 2;
        } else if sp[0] == "tpl" {
            let reg = self.reg(sp[1]);
            let val = *reg;
            *reg = val * 3;
        } else if sp[0] == "inc" {
            let reg = self.reg(sp[1]);
            let val = *reg;
            *reg = val + 1;
        } else if sp[0] == "jmp" {
            self.pc += sp[1].parse::<i64>().unwrap() - 1;
        } else if sp[0] == "jie" {
            let val = *self.reg(&sp[1][..1]);
            if val % 2 == 0 {
                self.pc += sp[2].parse::<i64>().unwrap() - 1;
            }
        } else if sp[0] == "jio" {
            let val = *self.reg(&sp[1][..1]);
            if val == 1 {
                self.pc += sp[2].parse::<i64>().unwrap() - 1;
            }
        }
        self.pc += 1;
        self.pc < self.prog.len() as i64
    }
}

fn part1(inp: &str) -> u64 {
    let mut m = Machine::new(inp);
    while m.step() {}
    m.b
}

fn part2(inp: &str) -> u64 {
    let mut m = Machine::new(inp);
    m.a = 1;
    while m.step() {}
    m.b
}

xaoc::xaoc!();
