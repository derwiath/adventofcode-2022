#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Vector2 {
    fn new(x: isize, y: isize) -> Vector2 {
        Vector2 { x, y }
    }

    fn cost_to(&self, other: &Vector2) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        ((x * x + y * y) as f64).sqrt() as f64
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Map {
    row_major: Vec<u8>,
    size: Vector2,
    start: Vector2,
    end: Vector2,
}

impl Map {
    fn new(row_major: Vec<u8>, size: Vector2, start: Vector2, end: Vector2) -> Map {
        Map {
            row_major,
            size,
            start,
            end,
        }
    }

    fn neighbours(&self, loc: &Vector2) -> [Option<Vector2>; 4] {
        let height = self.height(loc);
        let is_valid_neighbour = |n| self.is_valid_loc(n) && self.height(n) <= (height + 1);
        let left = Vector2::new(loc.x - 1, loc.y);
        let right = Vector2::new(loc.x + 1, loc.y);
        let up = Vector2::new(loc.x, loc.y - 1);
        let down = Vector2::new(loc.x, loc.y + 1);
        [
            if is_valid_neighbour(&left) {
                Some(left)
            } else {
                None
            },
            if is_valid_neighbour(&right) {
                Some(right)
            } else {
                None
            },
            if is_valid_neighbour(&up) {
                Some(up)
            } else {
                None
            },
            if is_valid_neighbour(&down) {
                Some(down)
            } else {
                None
            },
        ]
    }

    fn is_valid_loc(&self, loc: &Vector2) -> bool {
        return 0 <= loc.x && loc.x < self.size.x && 0 <= loc.y && loc.y < self.size.y;
    }

    fn height(&self, loc: &Vector2) -> u8 {
        let index = (loc.y * self.size.x + loc.x) as usize;
        self.row_major[index]
    }
}

struct DebugMap<'a> {
    map: &'a Map,
    loc: &'a Vector2,
}

impl<'a> DebugMap<'a> {
    fn new(map: &'a Map, loc: &'a Vector2) -> DebugMap<'a> {
        DebugMap { map, loc }
    }
}

impl<'a> fmt::Debug for DebugMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loc_index = self.loc.y * self.map.size.x + self.loc.x;
        for y in 0..self.map.size.y {
            let start_index = (y * self.map.size.x) as usize;
            let end_index = ((y + 1) * self.map.size.x) as usize;
            self.map.row_major[start_index..end_index]
                .iter()
                .enumerate()
                .for_each(|(i, h)| {
                    let index = (i + start_index) as isize;
                    let c = if index == loc_index {
                        '*'
                    } else {
                        (h + 'a' as u8) as char
                    };
                    write!(f, "{}", c).expect("Failed to write");
                });
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(input: &str) -> Result<Map, ()> {
        let mut width = 0;
        let mut start: Option<Vector2> = None;
        let mut end: Option<Vector2> = None;
        let row_major: Vec<u8> = input
            .lines()
            .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
            .enumerate()
            .map(|(y, l)| {
                let row: Vec<u8> = l
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let height_char = match c {
                            'a'..='z' => c,
                            'S' => {
                                start = Some(Vector2::new(x as isize, y as isize));
                                'a'
                            }
                            'E' => {
                                end = Some(Vector2::new(x as isize, y as isize));
                                'z'
                            }
                            _ => panic!("unknown char {}", c),
                        };
                        height_char as u8 - 'a' as u8
                    })
                    .collect();
                width = row.len();
                row
            })
            .flatten()
            .collect();
        let height = row_major.len() / width;
        let size = Vector2::new(width as isize, height as isize);
        Ok(Map::new(
            row_major,
            size,
            start.expect("No start (S) found on map"),
            end.expect("No end (E) found on Map"),
        ))
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            let start_index = (y * self.size.x) as usize;
            let end_index = ((y + 1) * self.size.x) as usize;
            self.row_major[start_index..end_index].iter().for_each(|h| {
                let c = (h + 'a' as u8) as char;
                write!(f, "{}", c).expect("Failed to write");
            });
            writeln!(f, "")?;
        }
        writeln!(f, "size: {}", self.size)?;
        writeln!(f, "start: {}", self.start)?;
        writeln!(f, "end: {}", self.end)?;
        Ok(())
    }
}

#[derive(Clone)]
struct Node {
    loc: Vector2,
    parent_loc: Vector2,
    dist_to_start: usize,
    cost_to_end: f64,
}

impl Node {
    fn new(loc: &Vector2, parent_loc: &Vector2, dist_to_start: usize, cost_to_end: f64) -> Node {
        Node {
            loc: loc.clone(),
            parent_loc: parent_loc.clone(),
            dist_to_start,
            cost_to_end,
        }
    }

    fn total_cost(&self) -> f64 {
        self.dist_to_start as f64 + self.cost_to_end
    }
}

struct OpenNode {
    loc: Vector2,
    total_cost: f64,
}

impl OpenNode {
    fn new(loc: &Vector2, total_cost: f64) -> OpenNode {
        OpenNode {
            loc: loc.clone(),
            total_cost,
        }
    }

    fn from_node(node: &Node) -> OpenNode {
        OpenNode {
            loc: node.loc,
            total_cost: node.total_cost(),
        }
    }

    fn cmp_by_total_cost(&self, other: &OpenNode) -> Ordering {
        let cost_diff = self.total_cost - other.total_cost;
        if cost_diff < 0.0 {
            Ordering::Less
        } else if cost_diff > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_by_total_cost(other)
            .then_with(|| self.loc.cmp(&other.loc))
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OpenNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_by_total_cost(other) == Ordering::Equal && self.loc == other.loc
    }
}

impl Eq for OpenNode {}

fn shortest_path(map: &Map, start: &Vector2) -> Option<usize> {
    let mut nodes: HashMap<Vector2, Node> = HashMap::new();
    let mut open: BTreeSet<OpenNode> = BTreeSet::new();

    {
        let start_node = Node::new(&start, &start, 0, start.cost_to(&map.end));
        open.insert(OpenNode::from_node(&start_node));
        nodes.insert(start.clone(), start_node);
    }

    while let Some(open_node) = open.pop_first() {
        let node = { (*nodes.get(&open_node.loc).unwrap()).clone() };
        if open_node.loc == map.end {
            return Some(node.dist_to_start);
        }

        let n_dist_to_start = node.dist_to_start + 1;
        for n_loc in map.neighbours(&open_node.loc).iter().filter_map(|n| *n) {
            let cost_to_end = n_loc.cost_to(&map.end);
            let total_cost = n_dist_to_start as f64 + cost_to_end;
            if let Some(n) = nodes.get(&n_loc) {
                if total_cost >= n.total_cost() {
                    continue;
                }

                let n_open = OpenNode::from_node(&n);
                open.remove(&n_open);
            } else {
                let n_node = Node::new(&n_loc, &node.loc, n_dist_to_start, cost_to_end);
                nodes.insert(n_loc.clone(), n_node);
            }

            let new_open_n = OpenNode::new(&n_loc, total_cost);
            open.insert(new_open_n);
        }
    }

    None
}

fn solve_part1(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();

    println!("{}", map);

    match shortest_path(&map, &map.start) {
        Some(n) => n,
        None => {
            println!("Failed to get shortest path");
            0
        }
    }
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
mod tests_day12 {
    use super::*;

    const EXAMPLE1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 31);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
