use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::multi::separated_list1;
use nom::IResult;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Add(i32),
    NoOp,
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Instruction::NoOp))
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, x) = complete::i32(input)?;

    Ok((input, Instruction::Add(x)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, parsed) = separated_list1(newline, alt((parse_noop, parse_addx)))(input)?;
    Ok((input, parsed))
}

fn process(input: &str) -> usize {
    let (_, instructions) = parse_instructions(input).unwrap();
    dbg!(instructions);

    0
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}")
}

#[test]
fn passes() {
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    let size = process(INPUT);
    assert_eq!(size, 13140);
}
