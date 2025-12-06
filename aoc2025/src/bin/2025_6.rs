use array2d::Array2D;

fn part1(inp: &str) -> i64 {
    let mut prob = vec![];
    let mut ops = vec![];
    for l in inp.lines() {
        let mut row = l.split_ascii_whitespace().peekable();
        let first = *row.peek().unwrap();
        if first == "*" || first == "+" {
            ops = row.collect();
            break;
        }
        for (i, el) in row.enumerate() {
            if prob.len() <= i {
                prob.push(vec![]);
            }
            prob[i].push(el.parse::<i64>().unwrap());
        }
    }

    let mut ret = 0;
    for (op, col) in ops.into_iter().zip(prob.into_iter()) {
        match op {
            "*" => ret += col.iter().product::<i64>(),
            "+" => ret += col.iter().sum::<i64>(),
            _ => unreachable!(),
        }
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let arr =
        Array2D::from_rows(&inp.lines().map(|l| l.chars().collect()).collect::<Vec<_>>()).unwrap();
    let mut ret = 0;
    let mut cols = arr.columns_iter().peekable();
    while cols.peek().is_some() {
        let mut op = 'n';
        let mut nums = vec![];
        for col in &mut cols {
            let col = col.collect::<Vec<_>>();
            let (&&mop, dd) = col.split_last().unwrap();
            if mop != ' ' {
                op = mop;
            }
            let num = dd.iter().fold(0, |acc, &&d| {
                if d == ' ' {
                    acc
                } else {
                    acc * 10 + d.to_digit(10).unwrap() as i64
                }
            });
            if num == 0 {
                break;
            }
            nums.push(num);
        }
        ret += match op {
            '*' => nums.iter().product::<i64>(),
            '+' => nums.iter().sum::<i64>(),
            _ => unreachable!(),
        }
    }
    ret
}

xaoc::xaoc!(
    sample = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
);
