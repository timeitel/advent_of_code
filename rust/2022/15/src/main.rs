mod parse;

use std::ops::RangeInclusive;

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

    fn ranges(&self, y: i64) -> impl Iterator<Item = RangeInclusive<i64>> {
        let mut ranges = vec![];
        for rec in &self.records {
            let radius = rec.sensor.manhattan_dist(rec.beacon);
            let y_dist = (y - rec.sensor.y).abs();
            if y_dist > radius {
                continue;
            }
            let d = radius - y_dist;
            let middle = rec.sensor.x;
            let start = middle - d;
            let end = middle + d;
            let range = start..=end;
            ranges.push(range);
        }
        ranges.sort_by_key(|r| *r.start());

        ranges.into_iter().coalesce(|a, b| {
            if b.start() - 1 <= *a.end() {
                if b.end() > a.end() {
                    Ok(*a.start()..=*b.end())
                } else {
                    Ok(a)
                }
            } else {
                Err((a, b))
            }
        })
    }

    fn ranges_clamped(
        &self,
        y: i64,
        x_range: RangeInclusive<i64>,
    ) -> impl Iterator<Item = RangeInclusive<i64>> {
        self.ranges(y).filter_map(move |r| {
            let r = *r.start().max(x_range.start())..=*r.end().min(x_range.end());
            if r.start() > r.end() {
                None
            } else {
                Some(r)
            }
        })
    }

    fn beacon_position(
        &self,
        y_range: &RangeInclusive<i64>,
        x_range: &RangeInclusive<i64>,
    ) -> Option<Point> {
        y_range.clone().find_map(|y| {
            self.ranges_clamped(y, x_range.clone())
                .nth(1)
                .map(|r| Point {
                    x: r.start() - 1,
                    y,
                })
        })
    }
}

fn main() {
    for (input, range) in [(include_str!("main.txt"), 0..=4_000_000)] {
        let map = Map::parse(input);
        let bp = map.beacon_position(&range, &range).unwrap();
        println!("tuning frequency: {}", bp.x * 4_000_000 + bp.y);
    }
}

#[cfg(test)]
mod tests {
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
    #[ignore]
    fn part_2_works() {}
}
