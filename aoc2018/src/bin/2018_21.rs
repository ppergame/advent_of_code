use aoc2018::p21::Machine;
use std::collections::HashSet;

fn part1(inp: &str) -> i64 {
    let mut ret = 0;
    {
        let mut m = Machine::parse(inp);
        let reg = m.prog[28].a;
        assert_eq!(m.prog[28].b, 0);
        let ret = &mut ret;
        m.add_callback(28, move |_, regs| {
            *ret = regs[reg];
            true
        });
        m.run_wasm().unwrap();
        // m.run();
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let mut ret = 0;
    {
        let mut m = Machine::parse(inp);
        let mut memo = HashSet::new();
        let mut prev = 0;
        let reg = m.prog[28].a;
        assert_eq!(m.prog[28].b, 0);
        let ret = &mut ret;
        m.add_callback(28, move |_, regs| {
            let val = regs[reg];
            if memo.contains(&val) {
                *ret = prev;
                return true;
            }
            prev = val;
            memo.insert(val);
            false
        });
        m.run_wasm().unwrap();
    }
    ret
}

xaoc::xaoc!(no_sample = true);
