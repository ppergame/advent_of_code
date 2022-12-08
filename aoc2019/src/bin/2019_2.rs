use aoc2019::intcode::*;

fn part1(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    ic.cs[1] = 12;
    ic.cs[2] = 2;
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Halt));
    ic.cs[0]
}

fn part2(inp: &str) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut ic = Intcode::new(inp);
            ic.cs[1] = noun;
            ic.cs[2] = verb;
            assert!(matches!(ic.run().unwrap(), IntcodeStatus::Halt));
            if ic.cs[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!();
}

xaoc::xaoc!();
