use aoc2019::intcode::*;

fn part1(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
    ic.input = Some(1);
    let mut answer = 0;
    loop {
        match ic.run().unwrap() {
            IntcodeStatus::Output(output) => answer = output,
            IntcodeStatus::Halt => break,
            _ => panic!("bad status"),
        }
    }
    answer
}

fn part2(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
    ic.input = Some(5);
    match ic.run().unwrap() {
        IntcodeStatus::Output(output) => output,
        _ => panic!("no output"),
    }
}

xaoc::xaoc!();
