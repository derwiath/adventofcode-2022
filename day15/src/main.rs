#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Vector2 {
    const fn new(x: isize, y: isize) -> Vector2 {
        Vector2 { x, y }
    }

    fn add(&self, rhs: &Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }

    fn sub(&self, rhs: &Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Eq, PartialEq)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn new(min: isize, max: isize) -> Range {
        Range { min, max }
    }

    fn from_center_and_dist(center: isize, dist: isize) -> Range {
        Range {
            min: center - dist,
            max: center + dist,
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        let min_cmp = self.min.cmp(&other.min);
        if min_cmp == Ordering::Equal {
            self.max.cmp(&other.max)
        } else {
            min_cmp
        }
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

struct SensorWithBeacon {
    sensor: Vector2,
    beacon: Vector2,
}

impl SensorWithBeacon {
    fn new(sensor: &Vector2, beacon: &Vector2) -> SensorWithBeacon {
        SensorWithBeacon {
            sensor: *sensor,
            beacon: *beacon,
        }
    }

    fn manhattan_distance(&self) -> isize {
        let diff = self.sensor.sub(&self.beacon);
        diff.x.abs() + diff.y.abs()
    }

    fn range_on_row(&self, y: isize) -> Option<Range> {
        let dist = (self.sensor.y - y).abs() - self.manhattan_distance();
        if dist <= 0 {
            Some(Range::from_center_and_dist(self.sensor.x, dist.abs()))
        } else {
            None
        }
    }
}

impl fmt::Debug for SensorWithBeacon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} <-> {:?}", self.sensor, self.beacon)
    }
}

fn parse_sensors(input: &str) -> Vec<SensorWithBeacon> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(
            r"Sensor at x=([-0-9]*), y=([-0-9]*): closest beacon is at x=([-0-9]*), y=([-0-9]*)"
        )
        .unwrap();
    }
    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            assert_eq!(captures.len(), 5);
            let sensor_x: isize = captures.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let sensor_y: isize = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let beacon_x: isize = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();
            let beacon_y: isize = captures.get(4).unwrap().as_str().parse::<isize>().unwrap();
            SensorWithBeacon::new(
                &Vector2::new(sensor_x, sensor_y),
                &Vector2::new(beacon_x, beacon_y),
            )
        })
        .collect()
}

fn count_known_locations(sensor_beacons: &[SensorWithBeacon], row: isize) -> usize {
    let ranges: Vec<Range> = {
        let mut ranges: Vec<Range> = sensor_beacons
            .iter()
            .filter_map(|s| {
                if let Some(r) = s.range_on_row(row) {
                    Some(r)
                } else {
                    None
                }
            })
            .collect();
        ranges.sort_unstable();
        ranges
    };
    if ranges.len() == 0 {
        return 0;
    }

    let mut count = 0;
    let mut prev_max = ranges[0].min - 1;
    for r in ranges {
        count += (r.max - r.min.max(prev_max)).max(0) as usize;
        prev_max = r.max.max(prev_max);
    }
    count as usize
}

fn solve_part1(input: &str) -> usize {
    let sensors_beacons = parse_sensors(input);
    count_known_locations(&sensors_beacons[..], 2000000)
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
mod tests_day15 {
    use super::*;

    const EXAMPLE1: &str = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test1_1() {
        let sensors_beacons = parse_sensors(EXAMPLE1);
        assert_eq!(count_known_locations(&sensors_beacons[..], 10), 26);
    }

    /*
     6    x      0
     5   xxx    -1
     4  xxxxx   -2
     3 xxxcxxx  -3
     2  xxxxx   -2
     1   xxx    -1
     0    B      0
    */
    #[test]
    fn test1_range_on_row_1() {
        let s = SensorWithBeacon::new(&Vector2::new(0, 3), &Vector2::new(0, 0));
        assert_eq!(s.range_on_row(0), Some(Range::new(0, 0)));
        assert_eq!(s.range_on_row(1), Some(Range::new(-1, 1)));
        assert_eq!(s.range_on_row(2), Some(Range::new(-2, 2)));
        assert_eq!(s.range_on_row(3), Some(Range::new(-3, 3)));
        assert_eq!(s.range_on_row(4), Some(Range::new(-2, 2)));
        assert_eq!(s.range_on_row(5), Some(Range::new(-1, 1)));
        assert_eq!(s.range_on_row(6), Some(Range::new(0, 0)));

        assert_eq!(s.range_on_row(-1), None);
        assert_eq!(s.range_on_row(7), None);
    }

    #[test]
    fn test1_range_on_row_2() {
        // "Sensor at x=17, y=20: closest beacon is at x=21, y=22"
        let s = SensorWithBeacon::new(&Vector2::new(17, 20), &Vector2::new(21, 22));
        assert_eq!(s.range_on_row(10), None);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
