use std::collections::HashMap;

use sscanf::scanf;

#[derive(Default)]
struct Machine {
    regs: HashMap<String, i64>,
}

impl Machine {
    fn reg(&self, reg: &str) -> i64 {
        self.regs.get(reg).copied().unwrap_or(0)
    }

    fn set_reg(&mut self, reg: &str, val: i64) {
        self.regs.insert(reg.to_string(), val);
    }

    fn step(&mut self, line: &str) {
        let (reg1, op1, imm1, reg2, op2, imm2) =
            scanf!(line, "{} {} {} if {} {} {}", str, str, i64, str, str, i64).unwrap();
        let val2 = self.reg(reg2);
        if match op2 {
            ">" => val2 > imm2,
            "<" => val2 < imm2,
            ">=" => val2 >= imm2,
            "<=" => val2 <= imm2,
            "==" => val2 == imm2,
            "!=" => val2 != imm2,
            _ => unreachable!(),
        } {
            let val1 = self.reg(reg1);
            let val1 = match op1 {
                "inc" => val1 + imm1,
                "dec" => val1 - imm1,
                _ => unreachable!(),
            };
            self.set_reg(reg1, val1);
        }
    }
}

fn part1(inp: &str) -> i64 {
    let mut m = Machine::default();
    for line in inp.lines() {
        m.step(line);
    }
    *m.regs.values().max().unwrap()
}

fn part2(inp: &str) -> i64 {
    let mut max = 0;
    let mut m = Machine::default();
    for line in inp.lines() {
        m.step(line);
        if let Some(new_max) = m.regs.values().max() {
            max = max.max(*new_max);
        }
    }
    max
}

xaoc::xaoc!(sample_idx = 1);
