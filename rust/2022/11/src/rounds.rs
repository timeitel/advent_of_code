use crate::parsers::{Monkey, Op, OpValue};

pub fn get_monkey_business(mut monkeys: Vec<Monkey>) -> u64 {
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let activity = monkeys[0].inspections * monkeys[1].inspections;
    activity
}

pub fn compute_rounds(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            if monkey.items.is_empty() {
                continue;
            }

            monkey.inspections += 1;
            let old = monkey.items.pop().unwrap();
            let new = match &monkey.operation {
                Op::Multiply(x) => old * x.get_value(old),
                Op::Add(x) => old + x.get_value(old),
                Op::Divide(x) => old / x.get_value(old),
                Op::Subtract(x) => old - x.get_value(old),
            };

            let mut pass_to_monkey: Option<usize> = None;
            let new = (new as f64 / 3.).floor() as u64;
            if let Op::Divide(OpValue::Int(n)) = monkey.test.operation {
                if new % n == 0 {
                    pass_to_monkey = Some(monkey.test.if_true as usize);
                } else {
                    pass_to_monkey = Some(monkey.test.if_false as usize);
                }
            }

            monkeys[pass_to_monkey.unwrap()].items.push(new);
        }
    }

    monkeys
}
