use sscanf::scanf;

struct Gen {
    val: i64,
    factor: i64,
}

impl Gen {
    fn step(&mut self) {
        self.val = (self.val * self.factor) % 2147483647;
    }
}

fn parse(inp: &str) -> (i64, i64) {
    let mut lines = inp.lines();
    let a = scanf!(lines.next().unwrap(), "Generator A starts with {}", i64).unwrap();
    let b = scanf!(lines.next().unwrap(), "Generator B starts with {}", i64).unwrap();
    (a, b)
}

fn part1(inp: &str) -> i64 {
    let (a, b) = parse(inp);
    let mut gen_a = Gen {
        val: a,
        factor: 16807,
    };
    let mut gen_b = Gen {
        val: b,
        factor: 48271,
    };
    let mut count = 0;
    for _ in 0..40000000 {
        gen_a.step();
        gen_b.step();
        if (gen_a.val & 0xFFFF) == (gen_b.val & 0xFFFF) {
            count += 1;
        }
    }
    count
}

fn part2(inp: &str) -> i64 {
    let (a, b) = parse(inp);
    let mut gen_a = Gen {
        val: a,
        factor: 16807,
    };
    let mut gen_b = Gen {
        val: b,
        factor: 48271,
    };
    let mut count = 0;
    for _ in 0..5000000 {
        gen_a.step();
        gen_b.step();
        while gen_a.val % 4 != 0 {
            gen_a.step();
        }
        while gen_b.val % 8 != 0 {
            gen_b.step();
        }
        if (gen_a.val & 0xFFFF) == (gen_b.val & 0xFFFF) {
            count += 1;
        }
    }
    count
}

xaoc::xaoc!(sample = "Generator A starts with 65\nGenerator B starts with 8921");
