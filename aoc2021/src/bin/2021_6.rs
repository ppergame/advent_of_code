fn parse(inp: &str) -> Vec<usize> {
    inp.split(',').map(|i| i.parse().unwrap()).collect()
}

fn part1(inp: &str) -> usize {
    let mut fish = parse(inp);
    for _ in 0..80 {
        let mut newfish = 0;
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                newfish += 1;
            } else {
                *f -= 1;
            }
        }
        fish.resize(fish.len() + newfish, 8);
    }
    fish.len()
}

fn part2(inp: &str) -> usize {
    let fish = parse(inp);
    let mut by_age = [0; 9];
    for f in fish {
        *by_age.get_mut(f).unwrap() += 1;
    }
    for _ in 0..256 {
        by_age.rotate_left(1);
        by_age[6] += by_age[8];
    }
    by_age.iter().sum()
}

xaoc::xaoc!();
