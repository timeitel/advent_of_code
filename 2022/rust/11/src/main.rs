mod parsers;

use crate::parsers::{operation, Op};

use self::parsers::monkeys;
use std::fs;

fn process(str: &str) -> u32 {
    let (_, monkeys) = monkeys(str).unwrap();
    32
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    print!("{result}");
}

#[test]
fn passes() {
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    assert_eq!(process(INPUT), 4);
}

#[test]
fn operation_works() {
    const INPUT: &str = "Operation: new = old * 19
    If false: throw to monkey 1";
    let (input, op) = operation(INPUT).unwrap();

    matches!(op, Op::Multiply(19));
}
