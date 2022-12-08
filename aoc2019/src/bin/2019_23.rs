use aoc2019::intcode::*;
use std::collections::VecDeque;

type XY = (i64, i64);
type Message = (i64, XY);

fn work(ic: &mut Intcode, mut xy: Option<XY>) -> Vec<Message> {
    let mut ret = Vec::<Message>::new();
    loop {
        match ic.run().unwrap() {
            IntcodeStatus::Input => {
                if let Some((x, y)) = xy {
                    xy = None;
                    ic.input = Some(x);
                    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
                    ic.input = Some(y);
                } else {
                    ic.input = Some(-1);
                    return ret;
                }
            }
            IntcodeStatus::Output(output) => {
                let addr = output;
                let x = match ic.run().unwrap() {
                    IntcodeStatus::Output(output) => output,
                    _ => panic!("unexpected status"),
                };
                let y = match ic.run().unwrap() {
                    IntcodeStatus::Output(output) => output,
                    _ => panic!("unexpected status"),
                };
                ret.push((addr, (x, y)));
                if xy.is_none() {
                    return ret;
                }
            }
            _ => todo!(),
        };
    }
}

fn part1(inp: &str) -> i64 {
    let prog = inp
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut queue = VecDeque::<Message>::new();
    let mut computers = (0..50)
        .map(|x| {
            let mut ic = Intcode::new_with_seq(&prog);
            assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
            ic.input = Some(x);
            ic
        })
        .collect::<Vec<Intcode>>();
    loop {
        if let Some((addr, xy)) = queue.pop_front() {
            //println!("dispatching message {} {} {}", addr, xy.0, xy.1);
            if addr == 255 {
                return xy.1;
            }
            let todo = work(computers.get_mut(addr as usize).unwrap(), Some(xy));
            queue.extend(todo);
            continue;
        }
        for ic in computers.iter_mut() {
            let todo = work(ic, None);
            queue.extend(todo);
        }
    }
}

fn part2(inp: &str) -> i64 {
    let prog = inp
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut queue = VecDeque::<Message>::new();
    let mut computers = (0..50)
        .map(|x| {
            let mut ic = Intcode::new_with_seq(&prog);
            assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
            ic.input = Some(x);
            ic
        })
        .collect::<Vec<Intcode>>();
    let mut nat = None;
    let mut last_y = None;
    loop {
        if let Some((addr, xy)) = queue.pop_front() {
            //println!("dispatching message {} {} {}", addr, xy.0, xy.1);
            if addr == 255 {
                nat = Some(xy);
                continue;
            }
            let todo = work(computers.get_mut(addr as usize).unwrap(), Some(xy));
            queue.extend(todo);
            continue;
        }
        let mut idle = true;
        for ic in computers.iter_mut() {
            let todo = work(ic, None);
            if !todo.is_empty() {
                idle = false;
            }
            queue.extend(todo);
        }
        if idle {
            if let Some(xy) = nat.take() {
                if let Some(last_y) = last_y {
                    if last_y == xy.1 {
                        return last_y;
                    }
                }
                last_y = Some(xy.1);
                queue.push_back((0, xy));
            }
        }
    }
}

xaoc::xaoc!();
