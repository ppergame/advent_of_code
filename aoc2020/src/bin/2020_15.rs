use std::collections::HashMap;

fn part1(inp: &str) -> u64 {
    let nums = inp
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut ages = HashMap::<u64, u64>::new();
    let mut turn = 1;
    for &num in &nums[0..nums.len() - 1] {
        ages.insert(num, turn);
        turn += 1;
    }
    let mut num = *nums.last().unwrap();
    for turn in turn + 1..=2020 {
        //println!("ages {:?} turn {} num {}", ages, turn, num);
        let next = match ages.get(&num) {
            Some(age) => turn - 1 - age,
            None => 0,
        };
        //println!("  next {}", next);
        ages.insert(num, turn - 1);
        num = next;
    }
    num
}

fn part2(inp: &str) -> u64 {
    let nums = inp
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut ages = HashMap::<u64, u64>::new();
    let mut turn = 1;
    for &num in &nums[0..nums.len() - 1] {
        ages.insert(num, turn);
        turn += 1;
    }
    let mut num = *nums.last().unwrap();
    for turn in turn + 1..=30000000 {
        //println!("ages {:?} turn {} num {}", ages, turn, num);
        let next = match ages.get(&num) {
            Some(age) => turn - 1 - age,
            None => 0,
        };
        //println!("  next {}", next);
        ages.insert(num, turn - 1);
        num = next;
    }
    num
}

xaoc::xaoc!();
