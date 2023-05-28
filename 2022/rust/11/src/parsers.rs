use core::panic;

use nom::bytes::complete::{tag, take, take_until, take_while};
use nom::character::complete::{digit0, digit1, multispace0, newline};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::{multi::separated_list0, IResult};

pub struct Monkey {
    items: Vec<u32>,
    operation: Op,
    test: Test,
}

pub struct Test {
    operation: Op,
    if_true: u32,
    if_false: u32,
}

pub enum Op {
    Multiply(u32),
    Add(u32),
    Divide(u32),
    Subtract(u32),
}

pub fn operation(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, operator_raw) = take(1usize)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, num) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = newline(input)?;

    let op = match operator_raw {
        "+" => Op::Add(num),
        "-" => Op::Subtract(num),
        "*" => Op::Multiply(num),
        "/" => Op::Divide(num),
        _ => panic!(),
    };

    Ok((input, op))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, matched) = tag("\n\n")(input)?;

    Ok((
        input,
        Test {
            if_true: 2,
            if_false: 2,
            operation: Op::Add(2),
        },
    ))
}

fn items(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, items) = separated_list0(tag(", "), digit0)(input)?;
    let items = items
        .iter()
        .filter_map(|&s| s.parse::<u32>().ok())
        .collect();
    let (input, _) = newline(input)?;

    Ok((input, items))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = take_until("Starting items: ")(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = items(input)?;
    let (input, operation) = operation(input)?;
    let (input, test) = test(input)?;

    let monkey = Monkey {
        operation,
        items,
        test,
    };

    Ok((input, monkey))
}

pub fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (rest, _) = separated_list1(tag("\n\n"), monkey)(input)?;

    Ok((rest, vec![]))
}
