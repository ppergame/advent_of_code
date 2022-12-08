use std::iter;

struct Reindeer {
    _name: String,
    speed: usize,
    flight_secs: usize,
    rest_secs: usize,
}

impl Reindeer {
    fn iter(&self) -> impl Iterator<Item = usize> {
        iter::repeat(self.speed)
            .take(self.flight_secs)
            .chain(iter::repeat(0).take(self.rest_secs))
            .cycle()
    }
}

fn parse(inp: &str) -> Vec<Reindeer> {
    let deer_re = regex::Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    inp.lines()
        .map(|line| {
            let caps = deer_re.captures(line).unwrap();
            Reindeer {
                _name: caps.get(1).unwrap().as_str().to_owned(),
                speed: caps.get(2).unwrap().as_str().parse().unwrap(),
                flight_secs: caps.get(3).unwrap().as_str().parse().unwrap(),
                rest_secs: caps.get(4).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn part1(inp: &str) -> usize {
    let reindeer = parse(inp);
    reindeer
        .into_iter()
        .map(|one| one.iter().take(2503).sum())
        .max()
        .unwrap()
}

fn part2(inp: &str) -> usize {
    let reindeer = parse(inp);
    let mut iters = reindeer
        .into_iter()
        .map(|one| one.iter())
        .collect::<Vec<_>>();
    let mut scores = vec![0; iters.len()];
    let mut dists = vec![0; iters.len()];
    for _ in 0..2503 {
        let inc_dists = iters.iter_mut().map(|x| x.next().unwrap());
        dists
            .iter_mut()
            .zip(inc_dists)
            .for_each(|(dist, inc)| *dist += inc);
        let max = dists.iter().max().unwrap();
        scores.iter_mut().zip(&dists).for_each(|(score, dist)| {
            if dist == max {
                *score += 1;
            }
        });
    }
    scores.into_iter().max().unwrap()
}

xaoc::xaoc!();
