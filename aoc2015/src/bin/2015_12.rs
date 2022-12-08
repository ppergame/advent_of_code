use serde_json::Value;

fn part1(inp: &str) -> i64 {
    let val: Value = serde_json::from_str(inp).unwrap();
    let mut stack = vec![&val];
    let mut total = 0;
    while let Some(top) = stack.pop() {
        total += match top {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(num) => num.as_i64().unwrap(),
            Value::String(_) => 0,
            Value::Array(vec) => {
                stack.extend(vec.iter());
                0
            }
            Value::Object(obj) => {
                stack.extend(obj.values());
                0
            }
        };
    }
    total
}

fn part2(inp: &str) -> i64 {
    let val: Value = serde_json::from_str(inp).unwrap();
    let mut stack = vec![&val];
    let mut total = 0;
    while let Some(top) = stack.pop() {
        total += match top {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(num) => num.as_i64().unwrap(),
            Value::String(_) => 0,
            Value::Array(vec) => {
                stack.extend(vec.iter());
                0
            }
            Value::Object(obj) => {
                if !obj.values().any(|v| v.as_str() == Some("red")) {
                    stack.extend(obj.values());
                }
                0
            }
        };
    }
    total
}

xaoc::xaoc!();
