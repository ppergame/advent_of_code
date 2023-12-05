use sscanf::scanf;

fn part1(inp: &str) -> i64 {
    let mut res = 0;
    for line in inp.lines() {
        let (g, rest) = scanf!(line, "Game {i64}: {String}").unwrap();
        let mut feasible = true;
        for round in rest.split(';') {
            for color in round.trim().split(',') {
                let (n, c) = scanf!(color.trim(), "{i64} {String}").unwrap();
                match c.as_str() {
                    "red" => {
                        if n > 12 {
                            feasible = false;
                        }
                    }
                    "green" => {
                        if n > 13 {
                            feasible = false;
                        }
                    }
                    "blue" => {
                        if n > 14 {
                            feasible = false;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        if feasible {
            res += g;
        }
    }
    res
}

fn part2(inp: &str) -> i64 {
    let mut res = 0;
    for line in inp.lines() {
        let (_g, rest) = scanf!(line, "Game {i64}: {String}").unwrap();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in rest.split(';') {
            for color in round.trim().split(',') {
                let (n, c) = scanf!(color.trim(), "{i64} {String}").unwrap();
                match c.as_str() {
                    "red" => min_red = min_red.max(n),
                    "green" => min_green = min_green.max(n),

                    "blue" => min_blue = min_blue.max(n),

                    _ => unreachable!(),
                }
            }
        }
        res += min_red * min_green * min_blue;
    }
    res
}

xaoc::xaoc!(
    sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
);
