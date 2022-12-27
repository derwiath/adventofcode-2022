#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(PartialEq, Debug)]
struct Valve<'a> {
    name: &'a str,
    flow: usize,
    neighbours: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(name: &'a str, flow: usize, neighbours: Vec<&'a str>) -> Valve<'a> {
        Valve {
            name,
            flow,
            neighbours,
        }
    }

    fn from_str(s: &'a str) -> Valve<'a> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(
                r"Valve ([A-Z]*) has flow rate=(\d*); tunnels lead to valves ([A-Z, ]*)"
            )
            .unwrap();
        }
        let captures = RE.captures(s).unwrap();
        assert_eq!(captures.len(), 4);
        let name = captures.get(1).unwrap().as_str();
        let flow: usize = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let neighbours: Vec<&'a str> = captures.get(3).unwrap().as_str().split(", ").collect();
        Valve::new(name, flow, neighbours)
    }
}

fn solve_part1(input: &str) -> usize {
    let valves: HashMap<&str, Valve> = input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let valve = Valve::from_str(l);
            (valve.name, valve)
        })
        .collect();
    valves.len()
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
mod tests_day16 {
    use super::*;

    const EXAMPLE1: &str = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 1651);
    }

    #[test]
    fn test1_valve_from_str1() {
        assert_eq!(
            Valve::from_str("Valve AA has flow rate=123; tunnels lead to valves DD, II, BB"),
            Valve::new("AA", 123, vec!["DD", "II", "BB"])
        );
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
