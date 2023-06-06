use crate::parsers::{Monkey, Op, OpValue};

pub fn get_monkey_business(mut monkeys: Vec<Monkey>) -> u64 {
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let activity = monkeys[0].inspections * monkeys[1].inspections;
    activity
}

pub fn compute_rounds(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    let lcm = monkeys
        .iter()
        .map(|monkey| {
            if let Op::Divide(n) = &monkey.test.operation {
                n.get_value(0)
            } else {
                0
            }
        })
        .product::<u64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let monkey = &mut monkeys[i];
                monkey.inspections += 1;
                let old = monkey.items.remove(0);
                let new = match &monkey.operation {
                    Op::Multiply(x) => old * x.get_value(old),
                    Op::Add(x) => old + x.get_value(old),
                    Op::Divide(x) => old / x.get_value(old),
                    Op::Subtract(x) => old - x.get_value(old),
                };

                let mut pass_to_monkey: Option<usize> = None;
                let new = new % lcm;
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
    }

    monkeys
}
