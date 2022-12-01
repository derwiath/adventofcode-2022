use std::env;
use std::fs;

fn solve_part1(input: &str) -> usize {
    let mut cals = 0;
    let mut max_cals = 0;
    for line in input.lines() {
        if line.len() > 0 {
            let count = line.parse::<usize>().unwrap();
            cals += count;
        } else {
            max_cals = if cals > max_cals { cals } else { max_cals };
            cals = 0;
        }
    }
    if cals > 0 {
        max_cals = if cals > max_cals { cals } else { max_cals };
    }
    max_cals
}

fn solve_part2(input: &str) -> usize {
    let mut cals = 0;
    let mut cals_list: Vec<usize> = Vec::new();
    for line in input.lines() {
        if line.trim().len() > 0 {
            let count = line.parse::<usize>().unwrap();
            cals += count;
        } else {
            cals_list.push(cals);
            cals = 0;
        }
    }
    if cals > 0 {
        cals_list.push(cals);
    }
    cals_list.sort_unstable_by(|a, b| b.cmp(a));
    cals_list.iter().take(3).fold(0, |acc, c| acc + c)
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
mod tests_day1 {
    use super::*;

    const EXAMPLE1: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 24000);
    }

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE1), 45000);
    }
}
