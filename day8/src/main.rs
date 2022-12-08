use std::collections::HashSet;
use std::env;
use std::fs;

fn solve_part1(input: &str) -> usize {
    let mut width = 0;
    let trees: Vec<u8> = input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let mut row: Vec<u8> = Vec::new();
            for c in l.chars() {
                assert!(c.is_digit(10));
                let digit = c.to_digit(10).unwrap();
                row.push(digit as u8);
            }
            width += 1;
            row
        })
        .flatten()
        .collect();
    let height = trees.len() / width;
    let border_visible = width * 2 + (height - 2) * 2;
    let mut visible: HashSet<usize> = HashSet::new();

    for row_index in 1..width - 1 {
        let row_start_index = row_index * width;
        let row = &trees[row_start_index..row_start_index + width];
        let mut left_max = row[0];
        let mut left_max_index = 0;
        for tree_index in 1..width - 1 {
            if row[tree_index] > left_max {
                visible.insert(row_start_index + tree_index);
                left_max_index = tree_index;
                left_max = row[tree_index];
            }
        }

        let mut right_max = row[width - 1];
        for tree_index in (left_max_index + 1..width - 1).rev() {
            if row[tree_index] > right_max {
                visible.insert(row_start_index + tree_index);
                right_max = row[tree_index];
            }
        }
    }

    border_visible + visible.len()
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
mod tests_day8 {
    use super::*;

    const EXAMPLE1: &str = "
30373
25512
65332
33549
35390";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 21);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
