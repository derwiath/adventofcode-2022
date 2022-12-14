use std::env;
use std::fs;

const LOWER_A_VALUE: u32 = ('a' as char) as u32;
const LOWER_Z_VALUE: u32 = ('z' as char) as u32;
const UPPER_A_VALUE: u32 = ('A' as char) as u32;
const UPPER_Z_VALUE: u32 = ('Z' as char) as u32;

fn get_value(c: char) -> Option<u32> {
    let value = c as u32;
    if value >= LOWER_A_VALUE && value <= LOWER_Z_VALUE {
        Some(value - LOWER_A_VALUE + 1)
    } else if value >= UPPER_A_VALUE && value <= UPPER_Z_VALUE {
        Some(value - UPPER_A_VALUE + 27)
    } else {
        None
    }
}

fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines().filter(|l| l.trim().len() > 0) {
        let compartment_size = line.len() / 2;
        let compartment1 = &line[0..compartment_size];
        let compartment2 = &line[compartment_size..];

        let prio = compartment1
            .chars()
            .filter(|c| compartment2.contains(|c2| &c2 == c))
            .take(1)
            .map(|c| get_value(c).expect("Unexpected char"))
            .fold(0, |acc, prio| acc + prio);

        sum += prio
    }
    sum
}

fn solve_part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut lines_iter = input.lines().filter(|l| l.trim().len() > 0);
    while let Some(line0) = lines_iter.next() {
        let line1 = lines_iter.next().unwrap();
        let line2 = lines_iter.next().unwrap();
        let badge_prio = line0
            .chars()
            .filter(|c| line1.contains(|c1| &c1 == c))
            .filter(|c| line2.contains(|c2| &c2 == c))
            .take(1)
            .map(|badge| get_value(badge).expect("Unexpected char"))
            .next()
            .unwrap();

        sum += badge_prio;
    }
    sum
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
mod tests_day3 {
    use super::*;

    const EXAMPLE1: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 157);
    }

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE1), 70);
    }
}
