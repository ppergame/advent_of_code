use aho_corasick::AhoCorasick;

fn part1(inp: &str) -> i64 {
    let mut cals = 0;
    for line in inp.lines() {
        let d1 = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let d2 = line.chars().filter(|c| c.is_ascii_digit()).last().unwrap();
        let cal = format!("{d1}{d2}").parse::<i64>().unwrap();
        cals += cal;
    }
    cals
}

fn part2(inp: &str) -> i64 {
    let patterns = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    let mut cals = 0;
    for line in inp.lines() {
        let all = ac
            .find_overlapping_iter(line)
            .map(|m| {
                let patt = m.pattern().as_u64() as i64;
                if patt < 9 {
                    patt + 1
                } else {
                    patt - 8
                }
            })
            .collect::<Vec<_>>();
        let cal = format!("{}{}", all[0], all.last().unwrap())
            .parse::<i64>()
            .unwrap();
        cals += cal;
    }
    cals
}

xaoc::xaoc!(
    sample2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
);
