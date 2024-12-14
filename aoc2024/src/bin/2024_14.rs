use array2d::Array2D;
use hashbrown::HashMap;
use sscanf::scanf;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Bot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

struct Map {
    bots: Vec<Bot>,
    counts: Array2D<usize>,
    width: i64,
    height: i64,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let bots = inp
            .lines()
            .map(|l| {
                let (x, y, vx, vy) = scanf!(l, "p={i64},{i64} v={i64},{i64}").unwrap();
                Bot { x, y, vx, vy }
            })
            .collect::<Vec<_>>();
        let width;
        let height;
        if bots.len() < 30 {
            width = 11;
            height = 7;
        } else {
            width = 101;
            height = 103;
        }
        let mut counts = Array2D::filled_with(0, height as usize, width as usize);
        for b in &bots {
            counts[(b.y as usize, b.x as usize)] += 1;
        }
        Self {
            bots,
            counts,
            width,
            height,
        }
    }

    fn step(&mut self) {
        for b in &mut self.bots {
            self.counts[(b.y as usize, b.x as usize)] -= 1;
            b.x = (b.x + b.vx).rem_euclid(self.width);
            b.y = (b.y + b.vy).rem_euclid(self.height);
            self.counts[(b.y as usize, b.x as usize)] += 1;
        }
    }

    fn qq(&self) -> (usize, usize, usize, usize) {
        let mut ul = 0;
        let mut ur = 0;
        let mut bl = 0;
        let mut br = 0;
        for b in &self.bots {
            if b.x < self.width / 2 && b.y < self.height / 2 {
                ul += 1;
            } else if b.x > self.width / 2 && b.y < self.height / 2 {
                ur += 1;
            } else if b.x < self.width / 2 && b.y > self.height / 2 {
                bl += 1;
            } else if b.x > self.width / 2 && b.y > self.height / 2 {
                br += 1;
            }
        }
        (ul, ur, bl, br)
    }

    fn score(&self) -> usize {
        let (ul, ur, bl, br) = self.qq();
        ul * ur * bl * br
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut bots = HashMap::new();
        for b in &self.bots {
            *bots.entry((b.x, b.y)).or_insert(0) += 1;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = bots.get(&(x, y)) {
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn part1(inp: &str) -> usize {
    let mut map = Map::parse(inp);
    for _ in 0..100 {
        map.step();
    }
    map.score()
}

fn part2(inp: &str) -> i64 {
    let mut map = Map::parse(inp);
    if map.height < 30 {
        return 0;
    }
    // const CONVO: i64 = 5;
    let mut ret = 0;
    let mut best = i64::MAX;
    for i in 0..map.width * map.height {
        let x_avg = map.bots.iter().map(|b| b.x).sum::<i64>() / map.bots.len() as i64;
        let x_dist_avg =
            map.bots.iter().map(|b| (x_avg - b.x).abs()).sum::<i64>() / map.bots.len() as i64;
        let y_avg = map.bots.iter().map(|b| b.y).sum::<i64>() / map.bots.len() as i64;
        let y_dist_avg =
            map.bots.iter().map(|b| (y_avg - b.y).abs()).sum::<i64>() / map.bots.len() as i64;
        let score = x_dist_avg + y_dist_avg;
        if score < best {
            best = score;
            ret = i;
        }
        map.step();
    }
    ret
}

xaoc::xaoc!(sample = "p=2,4 v=2,-3");
// xaoc::xaoc!(
//     sample = "p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3"
// );
