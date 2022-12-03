use std::env;
use std::fs;

fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;
    let lower_a_value = 'a'.to_digit(10).unwrap_or(97);
    let lower_z_value = 'z'.to_digit(10).unwrap_or(122);
    let upper_a_value = 'A'.to_digit(10).unwrap_or(65);
    let upper_z_value = 'Z'.to_digit(10).unwrap_or(90);
    for input_line in input.lines() {
        let line = input_line.trim();
        if line.len() == 0 {
            continue;
        }
        let compartment_size = line.len() / 2;
        let compartment1 = &line[0..compartment_size];
        let compartment2 = &line[compartment_size..];

        println!("1: {}", compartment1);
        println!("2: {}", compartment2);

        let prio = compartment1
            .chars()
            .filter(|c| compartment2.contains(|c2| &c2 == c))
            .take(1)
            .map(|c| {
                println!(" both: {}", c);
                let value = c as u32;
                if value >= lower_a_value && value <= lower_z_value {
                    value - lower_a_value + 1
                } else if value >= upper_a_value && value <= upper_z_value {
                    value - upper_a_value + 27
                } else {
                    panic!("Unexpected char {}", c);
                }
            })
            .fold(0, |acc, prio| acc + prio);

        println!("{}", prio);
        sum += prio
    }
    sum
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

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
