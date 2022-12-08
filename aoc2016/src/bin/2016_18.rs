use itertools::Itertools;

#[allow(clippy::match_like_matches_macro)]
fn is_trap(a: char, b: char, c: char) -> char {
    match (a == '^', b == '^', c == '^') {
        (true, true, false) => '^',
        (false, true, true) => '^',
        (true, false, false) => '^',
        (false, false, true) => '^',
        _ => '.',
    }
}

fn next(last: &[char]) -> Vec<char> {
    let mut row = vec![];
    row.push(is_trap('.', last[0], last[1]));
    row.extend(
        last.iter()
            .copied()
            .tuple_windows()
            .map(|(a, b, c)| is_trap(a, b, c)),
    );
    row.push(is_trap(last[last.len() - 2], last[last.len() - 1], '.'));
    row
}

fn solve(inp: &str, iter: usize) -> usize {
    let mut count = 0;
    let mut row = inp.chars().collect_vec();
    for _ in 0..iter {
        count += row.iter().filter(|&&c| c == '.').count();
        row = next(&row);
    }
    count
}

fn part1(inp: &str) -> usize {
    solve(inp, 40)
}

fn part2(inp: &str) -> usize {
    solve(inp, 400000)
}

xaoc::xaoc!(sample = "..^^.");
