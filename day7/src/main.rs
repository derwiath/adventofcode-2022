#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

fn solve_part1(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"^(\d*) (.*)").unwrap();
    }
    let sizes: Vec<usize> = input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            if let Some(captures) = RE.captures(l) {
                assert_eq!(captures.len(), 3);
                let size: usize = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                // let filename= captures.get(2).unwrap().as_str();
                size
            } else {
                0
            }
        })
        .collect();

    let mut sum = 0;
    let mut dir_size = 0;
    for size in sizes {
        if size > 0 {
            dir_size += size
        } else {
            if dir_size <= 100000 {
                sum += dir_size
            }
            dir_size = 0;
        }
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
mod tests_day7 {
    use super::*;

    const EXAMPLE1: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 95437);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
