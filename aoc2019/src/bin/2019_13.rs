use std::{cmp::Ordering, collections::HashMap};

use aoc2019::intcode::*;

fn part1(inp: &str) -> usize {
    let mut ic = Intcode::new(inp);
    let mut screen = HashMap::<(i64, i64), i64>::new();
    loop {
        let x = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            IntcodeStatus::Halt => break,
            _ => panic!("bad status"),
        };
        let y = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            _ => panic!("bad status"),
        };
        let tileid = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            _ => panic!("bad status"),
        };
        screen.insert((x, y), tileid);
    }
    screen.values().filter(|&t| *t == 2).count()
}

type Screen = HashMap<(i64, i64), i64>;

fn draw(screen: &Screen, actually_draw: bool) -> (i64, i64) {
    if actually_draw {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let mut paddlex = -1;
    let mut ballx = -1;
    let maxx = screen.keys().max_by_key(|(x, _)| x).unwrap().0;
    let maxy = screen.keys().max_by_key(|(_, y)| y).unwrap().1;
    for row in 0..=maxy {
        for col in 0..=maxx {
            let tileid = screen.get(&(col, row)).unwrap_or(&0);
            match tileid {
                3 => paddlex = col,
                4 => ballx = col,
                _ => (),
            }
            if actually_draw {
                print!(
                    "{}",
                    match tileid {
                        0 => String::from(" "),
                        _ => tileid.to_string(),
                    }
                );
            }
        }
        if actually_draw {
            println!();
        }
    }
    if actually_draw {
        println!();
    }
    (ballx, paddlex)
}

fn part2(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    ic.cs[0] = 2;
    let mut screen = Screen::new();
    let mut score = 0;
    loop {
        let x = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            IntcodeStatus::Input => {
                let (ballx, paddlex) = draw(&screen, false);
                ic.input = Some(match ballx.cmp(&paddlex) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                });
                continue;
            }
            IntcodeStatus::Halt => break,
            _ => panic!("bad status"),
        };
        let y = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            _ => panic!("bad status"),
        };
        let tileid = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            _ => panic!("bad status"),
        };
        if x == -1 && y == 0 {
            score = tileid;
        } else {
            screen.insert((x, y), tileid);
        }
    }
    score
}

xaoc::xaoc!();
