use pathfinding::directed::dijkstra::{dijkstra, dijkstra_all};

struct Input {
    num: i64,
    limit: i64,
}

impl Input {
    fn valid(&self, (x, y): (i64, i64)) -> bool {
        if x > self.limit || y > self.limit {
            return false;
        }
        if x < 0 || y < 0 {
            return false;
        }
        (x * x + 3 * x + 2 * x * y + y + y * y + self.num).count_ones() % 2 == 0
    }

    fn succ(&self, (x, y): (i64, i64)) -> Vec<((i64, i64), i64)> {
        let mut ret = vec![
            ((x - 1, y), 1),
            ((x, y - 1), 1),
            ((x + 1, y), 1),
            ((x, y + 1), 1),
        ];
        ret.retain(|&(s, _)| self.valid(s));
        ret
    }
}

fn part1(inp: &str) -> i64 {
    let input = Input {
        num: inp.parse().unwrap(),
        limit: 50,
    };
    let init = (1, 1);
    dijkstra(
        &init,
        |&state| input.succ(state),
        |&state| state == (31, 39),
    )
    .unwrap()
    .1
}

fn part2(inp: &str) -> usize {
    let input = Input {
        num: inp.parse().unwrap(),
        limit: 60,
    };
    let init = (1, 1);
    let map = dijkstra_all(&init, |&state| input.succ(state));
    map.into_iter().filter(|(_, (_, cost))| *cost <= 51).count()
}

xaoc::xaoc!();
