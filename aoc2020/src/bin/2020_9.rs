use itertools::Itertools;

fn a1(nums: &[i64]) -> i64 {
    for (i, &el) in nums.iter().enumerate() {
        if i < 25 {
            continue;
        }
        let mut found = false;
        for v in nums[i - 25..i].iter().combinations(2) {
            let (&a, &b) = (v[0], v[1]);
            if a + b == el {
                found = true;
                break;
            }
        }
        if !found {
            return el;
        }
    }
    unreachable!();
}

fn part1(inp: &str) -> i64 {
    let nums = inp
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i64>>();
    a1(&nums)
}

fn part2(inp: &str) -> i64 {
    let nums = inp
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i64>>();
    let num = a1(&nums);
    for i in 0..nums.len() - 1 {
        let mut sum = nums[i];
        for j in i + 1..nums.len() {
            sum += nums[j];
            if sum == num {
                return *nums[i..=j].iter().min().unwrap() + *nums[i..=j].iter().max().unwrap();
            }
            if sum > num {
                break;
            }
        }
    }
    unreachable!();
}

xaoc::xaoc!();
