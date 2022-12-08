fn part1(inp: &str) -> usize {
    let goal = inp.parse::<usize>().unwrap() / 10;
    let mut sieve = vec![1; goal];
    for elf in 2..goal {
        for house in (elf..goal).step_by(elf) {
            sieve[house] += elf;
        }
    }
    sieve.into_iter().position(|x| x >= goal).unwrap()
}

fn part2(inp: &str) -> usize {
    let goal = inp.parse::<usize>().unwrap();
    let mut sieve = vec![0; goal];
    for elf in 1..goal {
        for house in (elf..goal).step_by(elf).take(50) {
            sieve[house] += elf * 11;
        }
    }
    sieve.into_iter().position(|x| x >= goal).unwrap()
}

xaoc::xaoc!();
