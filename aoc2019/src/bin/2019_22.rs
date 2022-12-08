use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Step {
    Reverse,
    Cut(i128),
    Skip(i128),
}

lazy_static::lazy_static! {
    static ref CUT_RE: Regex = Regex::new(r"cut (-?\d+)").unwrap();
    static ref SKIP_RE: Regex = Regex::new(r"deal with increment (\d+)").unwrap();
}

fn parse(inp: &str) -> Vec<Step> {
    let mut ret = Vec::<Step>::new();
    for line in inp.lines() {
        if line.contains("deal into new stack") {
            ret.push(Step::Reverse);
        } else if let Some(captures) = CUT_RE.captures(line) {
            ret.push(Step::Cut(captures[1].parse().unwrap()));
        } else if let Some(captures) = SKIP_RE.captures(line) {
            ret.push(Step::Skip(captures[1].parse().unwrap()));
        } else {
            panic!();
        }
    }
    ret
}

fn apply_steps(mut idx: i128, len: i128, steps: &[Step]) -> i128 {
    for step in steps {
        match step {
            Step::Reverse => idx = len - idx - 1,
            Step::Cut(n) => idx = (idx - n).rem_euclid(len),
            Step::Skip(n) => idx = (idx * n).rem_euclid(len),
        }
    }
    idx
}

fn part1(inp: &str) -> i128 {
    let mut steps = parse(inp);
    bubble_sort(10007, &mut steps);
    apply_steps(2019, 10007, &steps)
}

fn reverse_steps(mut idx: i128, len: i128, steps: &[Step]) -> i128 {
    for step in steps.iter().rev() {
        match step {
            Step::Reverse => idx = len - idx - 1,
            Step::Cut(n) => idx = (idx + n).rem_euclid(len),
            Step::Skip(n) => idx = (idx * modinverse::modinverse(*n, len).unwrap()).rem_euclid(len),
        }
    }
    idx
}

fn bubble_sort(len: i128, steps: &mut Vec<Step>) {
    loop {
        let mut found = false;
        // Move Skip upwards
        // Move Reverse downwards
        // Keep Cut in the middle
        let mut i = 0;
        while i + 1 < steps.len() {
            match (&steps[i], &steps[i + 1]) {
                (Step::Reverse, Step::Reverse) => {
                    steps.splice(i..=i + 1, []);
                    found = true;
                    i -= std::cmp::min(i, 2);
                }
                (Step::Cut(n1), Step::Cut(n2)) => {
                    let (n1, n2) = (*n1, *n2);
                    steps.splice(i..=i + 1, [Step::Cut((n1 + n2).rem_euclid(len))]);
                    found = true;
                    i -= std::cmp::min(i, 2);
                }
                (Step::Skip(n1), Step::Skip(n2)) => {
                    let (n1, n2) = (*n1, *n2);
                    steps.splice(i..=i + 1, [Step::Skip((n1 * n2).rem_euclid(len))]);
                    found = true;
                    i -= std::cmp::min(i, 2);
                }
                (Step::Reverse, Step::Skip(n)) => {
                    let n = *n;
                    steps.splice(
                        i..=i + 1,
                        [
                            Step::Skip(n),
                            Step::Cut((1 - n).rem_euclid(len)),
                            Step::Reverse,
                        ],
                    );
                    found = true;
                    i -= std::cmp::min(i, 2);
                }
                (Step::Cut(n), Step::Reverse) => {
                    let n = *n;
                    steps.splice(i..=i + 1, [Step::Reverse, Step::Cut((-n).rem_euclid(len))]);
                    found = true;
                    i -= std::cmp::min(i, 2);
                }
                (Step::Cut(n1), Step::Skip(n2)) => {
                    let (n1, n2) = (*n1, *n2);
                    steps.splice(
                        i..=i + 1,
                        [Step::Skip(n2), Step::Cut((n1 * n2).rem_euclid(len))],
                    );
                    found = true;
                    i -= std::cmp::min(i, 2);
                }
                _ => (),
            }
            i += 1;
        }
        if !found {
            break;
        }
    }
}

fn part2(inp: &str) -> i128 {
    let len = 119315717514047;
    let repeat = 101741582076661;
    let mut steps = parse(inp);
    bubble_sort(len, &mut steps);
    let mut stepses = std::collections::HashMap::<i128, Vec<Step>>::new();
    let mut i = 1;
    while i < repeat {
        stepses.insert(i, steps.clone());
        steps = [&steps[..], &steps[..]].concat();
        bubble_sort(len, &mut steps);
        i *= 2;
    }
    let mut countdown = repeat;
    let mut next = i / 2;
    let mut total_steps = Vec::<Step>::new();
    while countdown > 0 {
        if next > countdown {
            next /= 2;
            continue;
        }
        total_steps.extend(stepses[&next].clone());
        countdown -= next;
    }
    bubble_sort(len, &mut total_steps);
    reverse_steps(2020, len, &total_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_skip() {
        let deck = (0..101).collect::<Vec<i128>>();
        let len = deck.len() as i128;
        let mut steps = vec![Step::Cut(60), Step::Reverse];
        let expected_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        bubble_sort(len, &mut steps);
        let sorted_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        assert_eq!(expected_deck, sorted_deck);
    }

    #[test]
    fn cut_skip() {
        let deck = (0..101).collect::<Vec<i128>>();
        let len = deck.len() as i128;
        let mut steps = vec![Step::Cut(60), Step::Skip(3)];
        let expected_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        bubble_sort(len, &mut steps);
        let sorted_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        assert_eq!(expected_deck, sorted_deck);
    }

    #[test]
    fn skip_reverse() {
        let deck = (0..101).collect::<Vec<i128>>();
        let len = deck.len() as i128;
        let mut steps = vec![Step::Reverse, Step::Skip(3)];
        let expected_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        bubble_sort(len, &mut steps);
        let sorted_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        assert_eq!(expected_deck, sorted_deck);
    }

    #[test]
    fn reverse_reverse() {
        let deck = (0..101).collect::<Vec<i128>>();
        let len = deck.len() as i128;
        let mut steps = vec![Step::Reverse, Step::Reverse];
        let expected_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        bubble_sort(len, &mut steps);
        assert_eq!(steps.len(), 0);
        let sorted_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        assert_eq!(expected_deck, sorted_deck);
    }

    #[test]
    fn cut_cut() {
        let deck = (0..101).collect::<Vec<i128>>();
        let len = deck.len() as i128;
        let mut steps = vec![Step::Cut(80), Step::Cut(40)];
        let expected_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        bubble_sort(len, &mut steps);
        assert_eq!(steps.len(), 1);
        let sorted_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        assert_eq!(expected_deck, sorted_deck);
    }

    #[test]
    fn skip_skip() {
        let deck = (0..101).collect::<Vec<i128>>();
        let len = deck.len() as i128;
        let mut steps = vec![Step::Skip(7), Step::Skip(3)];
        let expected_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        bubble_sort(len, &mut steps);
        assert_eq!(steps.len(), 1);
        let sorted_deck = deck
            .iter()
            .map(|&x| apply_steps(x, len, &steps))
            .collect::<Vec<_>>();
        assert_eq!(expected_deck, sorted_deck);
    }
}

xaoc::xaoc!();
