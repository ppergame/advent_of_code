#[derive(Debug, Copy, Clone)]
struct Item {
    val: i64,
    prev: usize,
    next: usize,
}

#[derive(Debug)]
struct Nums(Vec<Item>);

impl Nums {
    fn parse(inp: &str) -> Self {
        let mut items = inp
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let val = l.parse::<i64>().unwrap();
                Item {
                    val,
                    prev: i.saturating_sub(1),
                    next: i + 1,
                }
            })
            .collect::<Vec<_>>();
        items[0].prev = items.len() - 1;
        Nums(items)
    }

    fn cycle(&mut self) {
        for cur in 0..self.0.len() {
            let item = self.0[cur];
            let val = item.val;
            let shift = (val % (self.0.len() as i64 - 1)).abs();
            if shift != 0 {
                // take out of linked list
                self.0[item.prev].next = item.next;
                self.0[item.next].prev = item.prev;

                let mut pos = cur;
                match val.signum() {
                    1 => {
                        for _ in 0..shift {
                            pos = self.0[pos].next;
                        }
                        // adjust moved item's pointers
                        self.0[cur].prev = pos;
                        self.0[cur].next = self.0[pos].next;
                        // put item into list after pos
                        let item = self.0[cur];
                        self.0[item.prev].next = cur;
                        self.0[item.next].prev = cur;
                    }
                    -1 => {
                        for _ in 0..shift {
                            pos = self.0[pos].prev;
                        }
                        // adjust moved item's pointers
                        self.0[cur].prev = self.0[pos].prev;
                        self.0[cur].next = pos;
                        // put item into list before pos
                        let item = self.0[cur];
                        self.0[item.prev].next = cur;
                        self.0[item.next].prev = cur;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn answer(&self) -> i64 {
        let mut cur = self.0.iter().position(|i| i.val == 0).unwrap();
        let mut ret = 0;
        for _ in 0..3 {
            for _ in 0..1000 {
                cur = self.0[cur].next;
            }
            ret += self.0[cur].val;
        }
        ret
    }
}

fn part1(inp: &str) -> i64 {
    let mut nums = Nums::parse(inp);
    nums.cycle();
    nums.answer()
}

fn part2(inp: &str) -> i64 {
    let mut nums = Nums::parse(inp);
    for item in nums.0.iter_mut() {
        item.val *= 811589153;
    }
    for _ in 0..10 {
        nums.cycle();
    }
    nums.answer()
}

xaoc::xaoc!(
    sample = r#"1
2
-3
3
-2
0
4"#
);
