mod parse;

use itertools::Itertools;
use parse::{Point, Record};

struct Map {
    records: Vec<Record>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let records = input.lines().map(Record::must_parse).collect();
        Self { records }
    }

    fn num_impossible_positions(&self, y: i64) -> usize {
        let mut total = 0;
        let min_x = -4;
        let max_x = 26;

        for x in min_x..=max_x {
            let point = Point { x, y };
            if self.records.iter().any(|rec| rec.beacon == point) {
            } else if self.records.iter().any(|rec| {
                let radius = rec.sensor.manhattan_dist(rec.beacon);
                rec.sensor.manhattan_dist(point) <= radius
            }) {
                total += 1
            }
        }

        total
    }
}

fn main() {
    let input = include_str!("main.txt");
    let part_one = process_part1(input, 10);
    // let part_two = process_part2(include_str!("main.txt"), 10_000_000);
    println!("{part_one}");
    // println!("{part_two}");
}

fn process_part1(input: &str, line_number: i64) -> String {
    let map = Map::parse(input);
    let positions = map.num_impossible_positions(line_number);
    positions.to_string()
}

// TODO
fn process_part2(input: &str) -> String {
    "asd".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part_1_works() {
        debug_assert_eq!(process_part1(INPUT, 10), "26");
    }

    #[test]
    #[ignore]
    fn part_2_works() {
        debug_assert_eq!(process_part2(INPUT), "26");
    }
}
