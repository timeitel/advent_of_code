use std::collections::BTreeMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    let part_one = process_part1(include_str!("main.txt"));
    println!("{part_one}");
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Sensor {
    x: i32,
    y: i32,
}

fn position(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}

fn tree_map(input: &str) -> IResult<&str, BTreeMap<Sensor, Beacon>> {
    let (input, list) = separated_list1(
        line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(
                position.map(|(x, y)| Sensor { x, y }),
                tag(": closest beacon is at "),
                position.map(|(x, y)| Beacon { x, y }),
            ),
        ),
    )(input)?;

    Ok((
        input,
        list.into_iter().collect::<BTreeMap<Sensor, Beacon>>(),
    ))
}

fn process_part1(input: &str) -> usize {
    let (_, tree_map) = tree_map(input).unwrap();
    let distances: BTreeMap<&Sensor, i32> = tree_map
        .iter()
        .map(|(sensor, beacon)| {
            (
                sensor,
                (beacon.x - sensor.x).abs() + (beacon.y - sensor.y).abs(),
            )
        })
        .collect();

    let line_number = 2_000_000;
    distances
        .iter()
        .filter(|(sensor, distance)| {
            let sensor_range = (sensor.y - **distance)..(sensor.y + **distance);
            sensor_range.contains(&line_number)
        })
        .flat_map(|(sensor, max_distance)| {
            let distance_to_line = sensor.y - line_number;
            let max_distance_on_line = max_distance - distance_to_line.abs();
            (sensor.x - max_distance_on_line)..=sensor.x + max_distance_on_line
        })
        .unique()
        .filter(|x| {
            !tree_map.values().contains(&Beacon {
                x: *x,
                y: line_number,
            })
        })
        .count()
}
