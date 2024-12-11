use arrayvec::ArrayVec;
use hashbrown::HashMap;

fn part1(inp: &str) -> usize {
    let mut st = inp
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..25 {
        let mut nst = vec![];
        for v in st {
            if v == 0 {
                nst.push(1);
                continue;
            }
            let s = v.to_string();
            if s.len() % 2 == 0 {
                let (s1, s2) = s.split_at(s.len() / 2);
                nst.push(s1.parse::<i64>().unwrap());
                nst.push(s2.parse::<i64>().unwrap());
                continue;
            }
            nst.push(v * 2024);
        }
        st = nst;
    }
    st.len()
}

#[derive(Default)]
struct Memo {
    cache: HashMap<(i64, usize), usize>,
}

impl Memo {
    fn step(&mut self, v: i64, steps: usize) -> usize {
        if steps == 0 {
            return 1;
        }
        if let Some(&ret) = self.cache.get(&(v, steps)) {
            return ret;
        }
        let mut nst = ArrayVec::<i64, 2>::new();
        if v == 0 {
            nst.push(1);
        } else {
            let s = v.to_string();
            if s.len() % 2 == 0 {
                let (s1, s2) = s.split_at(s.len() / 2);
                nst.push(s1.parse::<i64>().unwrap());
                nst.push(s2.parse::<i64>().unwrap());
            } else {
                nst.push(v * 2024);
            }
        }
        let ret = nst.into_iter().map(|v| self.step(v, steps - 1)).sum();
        self.cache.insert((v, steps), ret);
        ret
    }
}

fn part2(inp: &str) -> usize {
    let st = inp
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut memo = Memo::default();
    st.into_iter().map(|v| memo.step(v, 75)).sum()
}

xaoc::xaoc!(sample = "125 17");
