use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::Parser;
use std::collections::HashSet;
use std::fs;

use nom::branch::alt;
use nom::character::complete::{self, newline};
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (rest, direction) = alt((
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
        complete::char('D').map(|_| Direction::Down),
        complete::char('U').map(|_| Direction::Up),
    ))(input)?;

    Ok((rest, direction))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, vecs) = separated_list1(
        newline,
        separated_pair(parse_direction, tag(" "), complete::u32),
    )(input)?;

    let vecs = vecs
        .iter()
        .flat_map(|(dir, repeat)| vec![*dir; *repeat as usize])
        .collect();

    Ok((input, vecs))
}

fn process(file: &str) -> usize {
    let (_, moves) = parse_moves(file).unwrap();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = HashSet::from([tail]);

    for head_move in moves.iter() {
        match head_move {
            Direction::Left => {
                head.0 -= 1;
            }
            Direction::Right => {
                head.0 += 1;
            }
            Direction::Up => {
                head.1 += 1;
            }
            Direction::Down => {
                head.1 -= 1;
            }
        }
        let x_range = (head.0 - 1)..=(head.0 + 1);
        let y_range = (head.1 - 1)..=(head.1 + 1);

        let is_tail_connected = x_range
            .cartesian_product(y_range)
            .any(|point| point == tail);

        if !is_tail_connected {
            let mut new_tail = head.clone();
            match head_move {
                Direction::Left => {
                    new_tail.0 += 1;
                }
                Direction::Right => {
                    new_tail.0 -= 1;
                }
                Direction::Up => {
                    new_tail.1 -= 1;
                }
                Direction::Down => {
                    new_tail.1 += 1;
                }
            }
            tail = new_tail;
            tail_positions.insert(new_tail);
        }
    }

    tail_positions.len()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}")
}

#[test]
fn passes() {
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let size = process(INPUT);
    assert_eq!(size, 13);
}
