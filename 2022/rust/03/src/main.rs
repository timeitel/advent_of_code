use std::fs;

fn get_common_char_value(first: &str, second: &str) -> u32 {
    let alpha_lower = ('a'..='z').into_iter().collect::<Vec<char>>();
    let alpha_upper = ('A'..='Z').into_iter().collect::<Vec<char>>();
    let mut value: u32 = 0;

    first.chars().for_each(|char| {
        if second.contains(char) {
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

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let result = file
        .lines()
        .map(|bag| {
            let itemslen = bag.len() / 2;
            let first = &bag[..itemslen];
            let second = &bag[itemslen..];
            get_common_char_value(first, second)
        })
        .sum::<u32>();

    println!("{}", result);
}
