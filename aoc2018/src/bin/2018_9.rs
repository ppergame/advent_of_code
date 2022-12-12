use itertools::Itertools;
use sscanf::scanf;

fn parse(inp: &str) -> (usize, usize) {
    scanf!(
        inp,
        "{} players; last marble is worth {} points",
        usize,
        usize
    )
    .unwrap()
}

fn run(inp: &str, part2: bool) -> usize {
    let (player_count, marble_count) = parse(inp);
    let marble_count = if !part2 {
        marble_count + 1
    } else {
        marble_count * 100 + 1
    };
    let mut scores = std::iter::repeat(0).take(player_count).collect_vec();
    let mut prev = std::iter::repeat(1000000).take(marble_count).collect_vec();
    let mut next = std::iter::repeat(9999999).take(marble_count).collect_vec();
    prev[0] = 0;
    next[0] = 0;
    let mut cur_player = 0;
    let mut cur_marble = 0;
    for next_marble in 1..marble_count {
        if next_marble % 23 != 0 {
            let one = next[cur_marble];
            let two = next[one];
            prev[next_marble] = one;
            next[next_marble] = two;
            next[one] = next_marble;
            prev[two] = next_marble;
            cur_marble = next_marble;
        } else {
            scores[cur_player] += next_marble;
            for _ in 0..7 {
                cur_marble = prev[cur_marble];
            }
            scores[cur_player] += cur_marble;
            let one = prev[cur_marble];
            let two = next[cur_marble];
            next[one] = two;
            prev[two] = one;
            cur_marble = two;
        }
        cur_player = (cur_player + 1) % player_count;
    }
    *scores.iter().max().unwrap()
}

fn part1(inp: &str) -> usize {
    run(inp, false)
}

fn part2(inp: &str) -> usize {
    run(inp, true)
}

xaoc::xaoc!(sample = "30 players; last marble is worth 5807 points");
