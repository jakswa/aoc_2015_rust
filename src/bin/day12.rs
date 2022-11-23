use aoc_2015_rust::input;
use serde_json::Value;

fn main() {
    let inp = input("12");
    let json: Value = serde_json::from_str(&inp).unwrap();
    println!("part1 answer: {}", part1(&json));
    println!("part2 answer: {}", part2(&json));
}

fn part1(json: &Value) -> i64 {
    dig(json)
}

fn part2(json: &Value) -> i64 {
    dig2(json)
}

fn dig(json: &Value) -> i64 {
    let mut sum = 0;
    match json {
        Value::Number(i) => sum += i.as_i64().unwrap(),
        Value::Array(a) => a.iter().for_each(|v| sum += dig(v)),
        Value::Object(o) => o.values().for_each(|ov| sum += dig(ov)),
        _ => {}
    }
    sum
}

fn dig2(json: &Value) -> i64 {
    let mut sum = 0;
    match json {
        Value::Number(i) => sum += i.as_i64().unwrap(),
        Value::Array(a) => a.iter().for_each(|v| sum += dig2(v)),
        Value::Object(o) => {
            if !o.values().any(|k| k == &"red") {
                o.values().for_each(|ov| sum += dig2(ov));
            }
        }
        _ => {}
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_stuff() {
        assert_eq!(0, 0);
    }
}
