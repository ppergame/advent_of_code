use sscanf::scanf;
use std::collections::HashMap;

struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Item {
    fn parse(s: &str) -> Vec<Item> {
        s.lines()
            .map(|l| {
                let (x, m, a, s) = scanf!(l, "{{x={i64},m={i64},a={i64},s={i64}}}").unwrap();
                Item { x, m, a, s }
            })
            .collect()
    }
}

enum Prop {
    X,
    M,
    A,
    S,
}

enum Verdict {
    Workflow(String),
    Accept,
    Reject,
}

impl Verdict {
    fn parse(s: &str) -> Verdict {
        match s {
            "A" => Verdict::Accept,
            "R" => Verdict::Reject,
            _ => Verdict::Workflow(s.to_string()),
        }
    }
}

enum Op {
    Lt,
    Gt,
}

struct Rule {
    prop: Prop,
    op: Op,
    val: i64,
    target: Verdict,
}

impl Rule {
    fn parse(s: &str) -> Rule {
        let (prop, op, val, target) = scanf!(s, "{char}{char}{i64}:{str}").unwrap();
        let prop = match prop {
            'x' => Prop::X,
            'm' => Prop::M,
            'a' => Prop::A,
            's' => Prop::S,
            _ => unreachable!(),
        };
        let op = match op {
            '<' => Op::Lt,
            '>' => Op::Gt,
            _ => unreachable!(),
        };
        let target = Verdict::parse(target);
        Rule {
            prop,
            op,
            val,
            target,
        }
    }
}

struct Workflow {
    #[allow(dead_code)]
    name: String,
    rules: Vec<Rule>,
    target: Verdict,
}

impl Workflow {
    fn parse(l: &str) -> Workflow {
        let (name, rest) = scanf!(l, "{String}{{{str}}}").unwrap();
        let (rest, target) = rest.rsplit_once(',').unwrap();
        let target = Verdict::parse(target);
        let rules = rest.split(',').map(Rule::parse).collect();
        Workflow {
            name,
            rules,
            target,
        }
    }
}

struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    fn parse(s: &str) -> Workflows {
        Workflows(
            s.lines()
                .map(|l| {
                    let w = Workflow::parse(l);
                    (w.name.clone(), w)
                })
                .collect(),
        )
    }

    fn accept(&self, item: &Item) -> bool {
        let mut workflow = "in";
        loop {
            let mut verdict = None;
            for rule in &self.0[workflow].rules {
                let val = match rule.prop {
                    Prop::X => item.x,
                    Prop::M => item.m,
                    Prop::A => item.a,
                    Prop::S => item.s,
                };
                if match rule.op {
                    Op::Lt => val < rule.val,
                    Op::Gt => val > rule.val,
                } {
                    verdict = Some(&rule.target);
                    break;
                }
            }
            let verdict = verdict.unwrap_or(&self.0[workflow].target);
            match verdict {
                Verdict::Workflow(w) => workflow = w,
                Verdict::Accept => return true,
                Verdict::Reject => return false,
            }
        }
    }
}

fn part1(inp: &str) -> i64 {
    let (workflows, items) = inp.split_once("\n\n").unwrap();
    let workflows = Workflows::parse(workflows);
    let items = Item::parse(items);
    let mut ret = 0;
    for item in items {
        if workflows.accept(&item) {
            ret += item.x + item.m + item.a + item.s;
        }
    }
    ret
}

#[derive(Clone)]
struct Range {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl Range {
    fn new() -> Range {
        Range {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        }
    }

    fn len(&self) -> i64 {
        (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)
    }

    fn sel(&mut self, prop: &Prop) -> &mut (i64, i64) {
        match prop {
            Prop::X => &mut self.x,
            Prop::M => &mut self.m,
            Prop::A => &mut self.a,
            Prop::S => &mut self.s,
        }
    }

    fn split(mut self, rule: &Rule) -> (Range, Range) {
        let mut range2 = self.clone();
        match &rule.op {
            Op::Lt => {
                intersect(self.sel(&rule.prop), (1, rule.val));
                intersect(range2.sel(&rule.prop), (rule.val, 4001));
            }
            Op::Gt => {
                intersect(self.sel(&rule.prop), (rule.val + 1, 4001));
                intersect(range2.sel(&rule.prop), (1, rule.val + 1));
            }
        };
        (self, range2)
    }
}

fn intersect(a: &mut (i64, i64), b: (i64, i64)) {
    a.0 = a.0.max(b.0);
    a.1 = a.1.min(b.1);
    a.1 = a.1.max(a.0);
}

impl Workflows {
    fn recurse(&self, range: Range, verdict: &Verdict) -> i64 {
        match verdict {
            Verdict::Accept => range.len(),
            Verdict::Reject => 0,
            Verdict::Workflow(w) => {
                if range.len() == 0 {
                    return 0;
                }
                self.search(range, w)
            }
        }
    }

    fn search(&self, mut range: Range, workflow: &str) -> i64 {
        // eprintln!("search {} {}", range.len(), workflow);
        let mut sum = 0;
        let mut new_range;
        let workflow = &self.0[workflow];
        for rule in &workflow.rules {
            (new_range, range) = range.split(rule);
            sum += self.recurse(new_range, &rule.target);
        }
        sum + self.recurse(range, &workflow.target)
    }
}

fn part2(inp: &str) -> i64 {
    let (workflows, _) = inp.split_once("\n\n").unwrap();
    let workflows = Workflows::parse(workflows);
    workflows.search(Range::new(), "in")
}

xaoc::xaoc!(
    sample = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
);
