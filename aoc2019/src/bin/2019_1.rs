fn part1(inp: &str) -> u32 {
    let mods: Vec<u32> = inp.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut total = 0;
    for m in mods {
        total += m / 3 - 2;
    }
    total
}

fn part2(inp: &str) -> i32 {
    let mods: Vec<i32> = inp.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut total = 0;
    for m in mods {
        let mut fuel = m / 3 - 2;
        while fuel > 0 {
            total += fuel;
            fuel = fuel / 3 - 2;
        }
    }
    total
}

xaoc::xaoc!();
