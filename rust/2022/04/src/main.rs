use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::{character::complete, IResult};
use std::fs;

fn assignment_range(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, lower) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, upper) = complete::u32(input)?;

    Ok((input, (lower, upper)))
}

fn line_to_assignments(line: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    separated_pair(assignment_range, tag(","), assignment_range)(line)
}

fn process(file: &str) -> usize {
    let result = file
        .lines()
        .filter(|line| {
            let (_, ((one_lower, one_upper), (two_lower, two_upper))) =
                line_to_assignments(line).unwrap();

            (one_lower..=one_upper).contains(&two_lower)
                || (one_lower..=one_upper).contains(&two_upper)
                || (two_lower..=two_upper).contains(&one_lower)
                || (two_lower..=two_upper).contains(&one_upper)
        })
        .count();

    println!("{result}");
    result
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    process(&file);
}

#[test]
fn passes() {
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(process(INPUT), 4);
}
