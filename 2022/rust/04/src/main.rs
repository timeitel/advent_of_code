use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let result = file
        .lines()
        .filter(|pair| {
            let (one, two) = pair.split_once(",").unwrap();
            let (one_lower_str, one_upper_str) = one.split_once("-").unwrap();
            let (two_lower_str, two_upper_str) = two.split_once("-").unwrap();

            let one_lower = one_lower_str.parse::<u32>().unwrap();
            let one_upper = one_upper_str.parse::<u32>().unwrap();
            let two_lower = two_lower_str.parse::<u32>().unwrap();
            let two_upper = two_upper_str.parse::<u32>().unwrap();

            (one_lower..=one_upper).contains(&two_lower)
                || (one_lower..=one_upper).contains(&two_upper)
                || (two_lower..=two_upper).contains(&one_lower)
                || (two_lower..=two_upper).contains(&one_upper)
        })
        .count();

    println!("{result}");
}
