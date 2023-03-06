use std::fs;

fn get_common_char_value(first: &str, second: &str, third: &str) -> u32 {
    let alpha_lower = ('a'..='z').into_iter().collect::<Vec<char>>();
    let alpha_upper = ('A'..='Z').into_iter().collect::<Vec<char>>();
    let mut value: u32 = 0;

    first.chars().for_each(|char| {
        if second.contains(char) && third.contains(char) {
            if char.is_lowercase() {
                value = u32::try_from(
                    alpha_lower
                        .iter()
                        .position(|&b| b == char)
                        .expect("Lower not working?"),
                )
                .unwrap()
                    + 1;
                println!("value is :{value} for {char}")
            } else {
                value = u32::try_from(
                    alpha_upper
                        .iter()
                        .position(|&b| b == char)
                        .expect("Upper not working?"),
                )
                .unwrap()
                    + 27;
                println!("value is :{value} for {char}")
            }
        }
    });

    value
}

struct Group<'a> {
    bags: Vec<&'a str>,
}

impl<'a> Group<'a> {
    fn add_bag(&mut self, bag: &'a str) {
        self.bags.push(bag);
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut elf_groups: Vec<Group> = vec![];

    for (i, bag) in file.lines().enumerate() {
        if i % 3 != 0 {
            let len = elf_groups.len();
            elf_groups[len - 1].add_bag(bag);
            continue;
        }

        elf_groups.push(Group { bags: vec![bag] });
    }

    let result = elf_groups
        .into_iter()
        .map(|group| get_common_char_value(group.bags[0], group.bags[1], group.bags[2]))
        .sum::<u32>();

    println!("{}", result);
}
