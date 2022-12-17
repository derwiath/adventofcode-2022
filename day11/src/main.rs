#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::str::FromStr;
use std::str::Lines;

fn monkey_id_from_str(s: &str) -> Result<usize, String> {
    //Monkey 0:
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"Monkey (\d*):").unwrap();
    }
    if let Some(captures) = RE.captures(s) {
        assert_eq!(captures.len(), 2);
        let monkey_id_str = captures.get(1).unwrap().as_str();
        if let Ok(monkey_id) = monkey_id_str.parse::<usize>() {
            Ok(monkey_id)
        } else {
            Err("Failed to parse monkey id".to_string())
        }
    } else {
        Err(format!("Failed to match monkey id regexp for '{}'", s))
    }
}

fn starting_items_from_str(s: &str) -> Result<Vec<usize>, String> {
    // Starting items: 79, 98
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"Starting items: ([0-9, ]*)").unwrap();
    }
    if let Some(captures) = RE.captures(s) {
        assert_eq!(captures.len(), 2);
        let items_str = captures.get(1).unwrap().as_str();
        let items: Vec<usize> = items_str
            .split(',')
            .map(|item_str| item_str.trim_start().trim_end())
            .map(|item_str| item_str.parse::<usize>().unwrap())
            .collect();
        Ok(items)
    } else {
        Err(format!("Failed to match test action regexp for '{}'", s))
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add(usize), // new = old + x
    Mul(usize), // new = old * x
    Sqr,        // new = old * old
}

impl Operation {
    fn inspect(&self, worry: usize) -> usize {
        match self {
            Operation::Add(x) => worry + x,
            Operation::Mul(x) => worry * x,
            Operation::Sqr => worry * worry,
        }
    }
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        // Operation: new = old * 19
        // Operation: new = old + 6
        // Operation: new = old * old
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"Operation: new = old ([+*]) ([\da-z]*)").unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            assert_eq!(captures.len(), 3);
            let operand = captures.get(1).unwrap().as_str();
            let x = captures.get(2).unwrap().as_str();
            if x == "old" {
                assert_eq!(operand, "*");
                Ok(Operation::Sqr)
            } else {
                match x.parse::<usize>() {
                    Ok(x_number) => {
                        if operand == "+" {
                            Ok(Operation::Add(x_number))
                        } else {
                            assert_eq!(operand, "*");
                            Ok(Operation::Mul(x_number))
                        }
                    }
                    _ => Err("Failed to parse number".to_string()),
                }
            }
        } else {
            Err(format!("Failed to match operation regexp for '{}'", s))
        }
    }
}

fn divisor_from_str(s: &str) -> Result<usize, String> {
    //  Test: divisible by 19
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"Test: divisible by (\d*)").unwrap();
    }
    if let Some(captures) = RE.captures(s) {
        assert_eq!(captures.len(), 2);
        let divisor = captures.get(1).unwrap().as_str();
        if let Ok(divisor_number) = divisor.parse::<usize>() {
            Ok(divisor_number)
        } else {
            Err("Failed to parse divisor number".to_string())
        }
    } else {
        Err(format!("Failed to match divisor regexp for '{}'", s))
    }
}

#[derive(Debug, PartialEq)]
struct Action {
    condition: bool,
    monkey: usize,
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Action, String> {
        //    If true: throw to monkey 2
        //    If false: throw to monkey 0
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^.*If ([a-z]*): throw to monkey ([0-9]*)").unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            assert_eq!(captures.len(), 3);
            let cond_str = captures.get(1).unwrap().as_str();
            let condition = match cond_str.parse::<bool>() {
                Ok(condition) => condition,
                Err(_) => return Err("Failed to parse bool condition".to_string()),
            };
            let monkey_str = captures.get(2).unwrap().as_str();
            let monkey = match monkey_str.parse::<usize>() {
                Ok(monkey) => monkey,
                Err(_) => return Err("Failed to parse monkey number".to_string()),
            };
            Ok(Action { condition, monkey })
        } else {
            Err(format!("Failed to match test action regexp for '{}'", s))
        }
    }
}

impl Action {
    fn new(condition: bool, monkey: usize) -> Action {
        Action { condition, monkey }
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Operation,
    test_divisor: usize,
    throw_to_true: usize,
    throw_to_false: usize,
    inspect_count: usize,
}

impl Monkey {
    fn new(
        id: usize,
        items: Vec<usize>,
        op: Operation,
        test_divisor: usize,
        throw_to_true: usize,
        throw_to_false: usize,
    ) -> Monkey {
        Monkey {
            id,
            items,
            op,
            test_divisor,
            throw_to_true,
            throw_to_false,
            inspect_count: 0,
        }
    }

    fn from_lines(mut lines: Lines) -> Result<(Monkey, Lines), String> {
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        assert!(lines.clone().count() >= 6);
        let id = monkey_id_from_str(lines.next().unwrap())?;
        let items = starting_items_from_str(lines.next().unwrap())?;
        let op = Operation::from_str(lines.next().unwrap())?;
        let test_divisor = divisor_from_str(lines.next().unwrap())?;
        let throw1 = Action::from_str(lines.next().unwrap())?;
        let throw2 = Action::from_str(lines.next().unwrap())?;
        let throws = if throw1.condition == true {
            (throw1, throw2)
        } else {
            (throw2, throw1)
        };
        Ok((
            Monkey::new(
                id,
                items,
                op,
                test_divisor,
                throws.0.monkey,
                throws.1.monkey,
            ),
            lines,
        ))
    }
}

fn parse_monkeys(input: &str) -> Result<Vec<Monkey>, String> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.clone().next() {
        if line.len() == 0 {
            lines.next();
            continue;
        }

        let monkey_pair = Monkey::from_lines(lines.clone())?;
        lines = monkey_pair.1;
        monkeys.push(monkey_pair.0);
    }
    Ok(monkeys)
}

fn solve_part1(input: &str) -> usize {
    let mut monkeys = match parse_monkeys(input) {
        Ok(monkeys) => monkeys,
        Err(e) => panic!("Error: {}", e),
    };

    monkeys.iter().for_each(|m| println!("{:?}", m));

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            let throws: Vec<(usize, usize)> = {
                let monkey = &monkeys[monkey_id];
                monkey
                    .items
                    .iter()
                    .map(|worry| {
                        let new_worry = monkey.op.inspect(*worry) / 3;
                        let next_monkey = if new_worry % monkey.test_divisor == 0 {
                            monkey.throw_to_true
                        } else {
                            monkey.throw_to_false
                        };
                        (next_monkey, new_worry)
                    })
                    .collect()
            };

            {
                let monkey = &mut monkeys[monkey_id];
                monkey.inspect_count += monkey.items.len();
                monkey.items.clear();
            }
            for (next_monkey, worry) in throws {
                monkeys[next_monkey].items.push(worry);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspect_count);
    assert!(monkeys.len() >= 2);
    monkeys.pop().unwrap().inspect_count * monkeys.pop().unwrap().inspect_count
}

fn run_monkey_rounds(input: &str, round_count: usize) -> Vec<usize> {
    let mut monkeys = match parse_monkeys(input) {
        Ok(monkeys) => monkeys,
        Err(e) => panic!("Error: {}", e),
    };

    let max_combined_divisor: usize = monkeys.iter().fold(1, |acc, m| acc * m.test_divisor);

    for _round in 0..round_count {
        for monkey_id in 0..monkeys.len() {
            let throws: Vec<(usize, usize)> = {
                let monkey = &monkeys[monkey_id];
                monkey
                    .items
                    .iter()
                    .map(|worry| {
                        let new_worry = monkey.op.inspect(*worry) % max_combined_divisor;
                        let next_monkey = if new_worry % monkey.test_divisor == 0 {
                            monkey.throw_to_true
                        } else {
                            monkey.throw_to_false
                        };
                        (next_monkey, new_worry)
                    })
                    .collect()
            };

            {
                let monkey = &mut monkeys[monkey_id];
                monkey.inspect_count += monkey.items.len();
                monkey.items.clear();
            }
            for (next_monkey, worry) in throws {
                monkeys[next_monkey].items.push(worry);
            }
        }
    }

    monkeys.iter().map(|m| m.inspect_count).collect()
}

fn solve_part2(input: &str) -> usize {
    let mut inspect_counts = run_monkey_rounds(input, 10000);
    inspect_counts.sort();
    assert!(inspect_counts.len() >= 2);
    let inspect_count1 = inspect_counts.pop().unwrap();
    let inspect_count2 = inspect_counts.pop().unwrap();
    inspect_count1 * inspect_count2
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
mod tests_day11 {
    use super::*;

    const EXAMPLE1: &str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 10605);
    }

    #[test]
    fn test1_monkey_id_1() {
        assert_eq!(monkey_id_from_str("Monkey 17:"), Ok(17));
    }

    #[test]
    fn test1_items_1() {
        assert_eq!(
            starting_items_from_str("Starting items: 79, 60, 97"),
            Ok(vec![79, 60, 97])
        );
    }

    #[test]
    fn test1_op_1() {
        assert_eq!(
            Operation::from_str("  Operation: new = old * 19"),
            Ok(Operation::Mul(19))
        );
    }

    #[test]
    fn test1_op_2() {
        assert_eq!(
            Operation::from_str("  Operation: new = old * old"),
            Ok(Operation::Sqr)
        );
    }

    #[test]
    fn test1_op_3() {
        assert_eq!(
            Operation::from_str("  Operation: new = old + 3"),
            Ok(Operation::Add(3))
        );
    }

    #[test]
    fn test1_action_1() {
        assert_eq!(
            Action::from_str("If true: throw to monkey 2"),
            Ok(Action::new(true, 2))
        );
    }

    #[test]
    fn test1_action_2() {
        assert_eq!(
            Action::from_str("If false: throw to monkey 0"),
            Ok(Action::new(false, 0))
        );
    }

    #[test]
    fn test1_divisor_1() {
        let s: &str = "Test: divisible by 19";
        assert_eq!(divisor_from_str(s), Ok(19));
    }

    #[test]
    fn test1_monkey_1() {
        let s: &str = "Monkey 1:
                       Starting items: 54, 65, 75, 74
                       Operation: new = old + 6
                       Test: divisible by 19
                         If true: throw to monkey 2
                         If false: throw to monkey 0";
        if let Ok((parsed_monkey, _)) = Monkey::from_lines(s.lines()) {
            assert_eq!(
                parsed_monkey,
                Monkey::new(1, vec![54, 65, 75, 74], Operation::Add(6), 19, 2, 0),
            );
        } else {
            assert!(false, "Failed to parse monkey from lines");
        }
    }

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE1), 2713310158);
    }

    #[test]
    fn test2_rounds_01() {
        assert_eq!(run_monkey_rounds(EXAMPLE1, 1), vec![2, 4, 3, 6]);
    }

    #[test]
    fn test2_rounds_20() {
        assert_eq!(run_monkey_rounds(EXAMPLE1, 20), vec![99, 97, 8, 103]);
    }
}
