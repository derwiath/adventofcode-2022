#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::Lines;

fn process<'a>(mut lines: Lines<'a>, path: String) -> (usize, Lines<'a>, HashMap<String, usize>) {
    lazy_static! {
        static ref CD_RE: regex::Regex = regex::Regex::new(r"^\$ cd ([a-z]*|/)").unwrap();
        static ref CD_UP_RE: regex::Regex = regex::Regex::new(r"^\$ cd \.\.").unwrap();
        static ref LS_RE: regex::Regex = regex::Regex::new(r"^\$ ls\b").unwrap();
        static ref FILE_RE: regex::Regex = regex::Regex::new(r"^(\d*) (.*)").unwrap();
        static ref DIR_RE: regex::Regex = regex::Regex::new(r"^dir (.*)").unwrap();
    }

    println!("Enter {}", path);

    let mut size_map: HashMap<String, usize> = HashMap::new();

    let mut dir_size = 0;
    while let Some(line) = lines.next() {
        println!("{}", line);
        if let Some(_) = CD_UP_RE.captures(line) {
            println!("CD_UP_RE");
            break;
        } else if let Some(cap) = CD_RE.captures(line) {
            println!("CD_RE");
            assert_eq!(cap.len(), 2);
            let name = cap.get(1).unwrap().as_str();
            let subpath = format!("{}/{}", path, name);
            let (sub_size, sub_lines, sub_size_map) = process(lines, subpath);
            lines = sub_lines;
            sub_size_map.into_iter().for_each(|(p, s)| {
                assert_eq!(size_map.insert(p, s), None);
            });
            dir_size += sub_size;
        } else if let Some(_) = LS_RE.captures(line) {
        } else if let Some(_) = DIR_RE.captures(line) {
        } else if let Some(cap) = FILE_RE.captures(line) {
            assert_eq!(cap.len(), 3);
            let file_size: usize = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            dir_size += file_size;
        } else if line.len() > 0 {
            assert!(false, "Failed to parse {}", line);
        }
    }
    println!("Exit {}, {}", path, dir_size);
    assert_eq!(size_map.insert(path, dir_size), None);
    (dir_size, lines, size_map)
}

fn solve_part1(input: &str) -> usize {
    let (_, _, size_map) = process(input.lines(), "C:".to_owned());

    size_map
        .into_values()
        .filter(|size| size <= &100000)
        .fold(0, |acc, size| acc + size)
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
