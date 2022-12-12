use array2d::Array2D;
use rayon::prelude::*;

struct Memo {
    c: Array2D<i64>,
}

impl Memo {
    fn new(inp: i64) -> Self {
        let mut c = Array2D::filled_with(0, 301, 301);
        for x in 1..=300 {
            for y in 1..=300 {
                let rack_id = x as i64 + 10;
                let level = (rack_id * y as i64 + inp) * rack_id;
                let level = (level / 100) % 10 - 5;
                c[(x, y)] = level;
            }
        }
        Memo { c }
    }

    fn run(&self, step: usize) -> ((usize, usize), i64) {
        let mut max = None;
        for x in 1..=300 - step + 1 {
            let mut sum = 0;
            for cx in x..x + step {
                for cy in 1..=step {
                    sum += self.c[(cx, cy)];
                }
            }
            for y in 1..=300 - step + 1 {
                if max.map_or(true, |(_, prev)| sum > prev) {
                    max = Some(((x, y), sum));
                }
                if y < 300 - step + 1 {
                    for cx in x..x + step {
                        sum -= self.c[(cx, y)];
                        sum += self.c[(cx, y + step)];
                    }
                }
            }
        }
        // if max.is_none() {
        //     eprintln!("{step}");
        // }
        max.unwrap()
    }
}

fn part1(inp: &str) -> String {
    let inp = inp.parse::<i64>().unwrap();
    let memo = Memo::new(inp);
    let ((x, y), _) = memo.run(3);
    format!("{x},{y}")
}

fn part2(inp: &str) -> String {
    let inp = inp.parse::<i64>().unwrap();
    let memo = Memo::new(inp);
    let ((x, y), step, _) = (1..=300)
        .into_par_iter()
        .map(|step| {
            let ((x, y), sum) = memo.run(step);
            ((x, y), step, sum)
        })
        .max_by_key(|(_, _, sum)| *sum)
        .unwrap();
    format!("{x},{y},{step}")
}

xaoc::xaoc!(sample = "42");
