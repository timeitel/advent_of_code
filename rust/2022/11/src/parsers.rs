use core::panic;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till, take_until};
use nom::character::complete::{digit0, digit1, multispace0, multispace1, newline};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::{multi::separated_list0, IResult};

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Op,
    pub test: Test,
    pub inspections: u64,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            items: vec![],
            operation: Op::Add(OpValue::Old),
            test: Test {
                operation: Op::Add(OpValue::Old),
                if_true: 0,
                if_false: 0,
            },
            inspections: 0,
        }
    }
}

#[derive(Debug)]
pub struct Test {
    pub operation: Op,
    pub if_true: u64,
    pub if_false: u64,
}

#[derive(Debug)]
pub enum OpValue {
    Int(u64),
    Old,
}

impl OpValue {
    pub fn get_value(&self, num: u64) -> u64 {
        match *self {
            OpValue::Int(x) => x,
            OpValue::Old => num,
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Multiply(OpValue),
    Add(OpValue),
    Divide(OpValue),
    Subtract(OpValue),
}

pub fn operation(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, operator_raw) = take(1usize)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, matched_op_value) = alt((tag("old"), digit1))(input)?;

    if matched_op_value == "old" {
        let op = match operator_raw {
            "+" => Op::Add(OpValue::Old),
            "-" => Op::Subtract(OpValue::Old),
            "*" => Op::Multiply(OpValue::Old),
            "/" => Op::Divide(OpValue::Old),
            _ => panic!(),
        };
        let (input, _) = newline(input)?;
        return Ok((input, op));
    }

    let num = matched_op_value.parse::<u64>().unwrap();
    let (input, _) = newline(input)?;
    let op = match operator_raw {
        "+" => Op::Add(OpValue::Int(num)),
        "-" => Op::Subtract(OpValue::Int(num)),
        "*" => Op::Multiply(OpValue::Int(num)),
        "/" => Op::Divide(OpValue::Int(num)),
        _ => panic!(),
    };

    Ok((input, op))
}

pub fn test(input: &str) -> IResult<&str, Test> {
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, divisible_by) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, if_true) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = take_till(|c: char| c.is_numeric())(input)?;
    let (input, if_false) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;

    Ok((
        input,
        Test {
            if_true,
            if_false,
            operation: Op::Divide(OpValue::Int(divisible_by)),
        },
    ))
}

fn items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, items) = separated_list0(tag(", "), digit0)(input)?;
    let items = items
        .iter()
        .filter_map(|&s| s.parse::<u64>().ok())
        .collect();
    let (input, _) = multispace1(input)?;

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
        ..Default::default()
    };

    Ok((input, monkey))
}

pub fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (rest, monkeys) = separated_list1(tag("\n\n"), monkey)(input)?;

    Ok((rest, monkeys))
}
