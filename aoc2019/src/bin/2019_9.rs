use aoc2019::intcode::*;

fn part1(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    //let mut ic = Intcode::new("104,1125899906842624,99");
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
    ic.input = Some(1);
    match ic.run().unwrap() {
        IntcodeStatus::Output(output) => output,
        _ => panic!("bad status"),
    }
}

fn part2(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    //let mut ic = Intcode::new("104,1125899906842624,99");
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
    ic.input = Some(2);
    match ic.run().unwrap() {
        IntcodeStatus::Output(output) => output,
        _ => panic!("bad status"),
    }
}

xaoc::xaoc!();
