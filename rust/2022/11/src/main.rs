mod parsers;
mod rounds;

use self::parsers::monkeys;
use self::rounds::compute_rounds;
use self::rounds::get_monkey_business;
use std::fs;

fn process(str: &str) -> u64 {
    let (_, monkeys) = monkeys(str).unwrap();
    let computed_monkeys = compute_rounds(monkeys);
    let activity = get_monkey_business(computed_monkeys);
    activity
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}");
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
fn test_works() {
    use crate::parsers::test;
    const INPUT: &str = "Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
    let result = test(INPUT).unwrap();
    dbg!(result);
}

#[test]
fn two_monkeys() {
    const INPUT: &str = "Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 20 
    If false: throw to monkey 8

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3";

    let (_, monkeys) = monkeys(INPUT).unwrap();
    dbg!(monkeys);
}

#[test]
fn one_monkey() {
    const INPUT: &str = "Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3";

    let (_, monkeys) = monkeys(INPUT).unwrap();
    dbg!(monkeys);
}
