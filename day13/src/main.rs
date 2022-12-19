#![allow(dead_code)]

use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Value {
    Int(usize),
    List(Vec<Value>),
}

impl Value {
    fn push(&mut self, v: Value) {
        if let Value::List(values) = self {
            values.push(v);
        } else {
            panic!("An Int cannot be parent");
        }
    }
}

impl FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Value, ()> {
        let mut values: Vec<Value> = Vec::<Value>::new();
        let mut integer_str: String = String::new();

        for c in s.chars() {
            if c == '[' {
                values.push(Value::List(Vec::<Value>::new()));
            } else if c == ']' {
                if !integer_str.is_empty() {
                    let integer: usize = integer_str.parse::<usize>().unwrap();
                    integer_str.clear();
                    assert!(values.len() > 0);
                    let value_count = values.len();
                    values[value_count - 1].push(Value::Int(integer));
                }
                if values.len() > 1 {
                    let value = values.pop().unwrap();
                    let value_count = values.len();
                    values[value_count - 1].push(value);
                }
            } else if c == ',' {
                if !integer_str.is_empty() {
                    let integer: usize = integer_str.parse::<usize>().unwrap();
                    integer_str.clear();
                    assert!(values.len() > 0);
                    let value_count = values.len();
                    values[value_count - 1].push(Value::Int(integer));
                }
            } else {
                assert!(c.to_digit(10).is_some());
                integer_str.push(c);
            }
        }

        assert_eq!(values.len(), 1);
        Ok(values.pop().unwrap())
    }
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| Value::from_str(l))
        .count()
}

fn solve_part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const PACKAGE_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    let filename = args
        .get(1)
        .expect(format!("Usage: {} input-filename", PACKAGE_NAME.unwrap()).as_str());

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests_day13 {
    use super::*;

    const EXAMPLE1: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 13);
    }

    #[test]
    fn test1_value_1() {
        assert_eq!(
            Value::from_str("[1,1,3,1,1]"),
            Ok(Value::List(vec![
                Value::Int(1),
                Value::Int(1),
                Value::Int(3),
                Value::Int(1),
                Value::Int(1)
            ]))
        );
    }

    #[test]
    fn test1_value_2() {
        assert_eq!(
            Value::from_str("[[1],[2,3,4]]"),
            Ok(Value::List(vec![
                Value::List(vec![Value::Int(1),]),
                Value::List(vec![Value::Int(2), Value::Int(3), Value::Int(4),]),
            ]))
        );
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
