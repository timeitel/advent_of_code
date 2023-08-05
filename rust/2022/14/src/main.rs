use std::collections::BTreeSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let one = process_part1(include_str!("input.txt"));
    let two = process_part2(include_str!("input.txt"));
    println!("{one}");
    println!("{two}");
}

fn line(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, complete::char(','), complete::u32),
    )(input)?;

    let iter = pairs
        .into_iter()
        .tuple_windows()
        .flat_map(|((a_x, a_y), (b_x, b_y))| {
            let x_min = a_x.min(b_x);
            let x_max = a_x.max(b_x);
            let x_range = x_min..=x_max;

            let y_min = a_y.min(b_y);
            let y_max = a_y.max(b_y);
            let y_range = y_min..=y_max;
            x_range.cartesian_product(y_range)
        });

    Ok((input, iter))
}

fn rocks(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, pairs) = separated_list1(line_ending, line)(input)?;
    let map: BTreeSet<(u32, u32)> = pairs.into_iter().flatten().collect();
    Ok((input, map))
}

fn process_part1(input: &str) -> String {
    let (_, mut material) = rocks(input).unwrap();
    let rocks_without_sand = material.len();

    let material_clone = material.clone();
    let mut material_vec = material_clone.iter().collect::<Vec<&(u32, u32)>>();
    material_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let lowest_rock = material_vec.last().unwrap();

    let mut curr_sand = (500, 0);
    while curr_sand.1 < lowest_rock.1 {
        let down = (curr_sand.0, curr_sand.1 + 1);
        let down_left = (curr_sand.0 - 1, curr_sand.1 + 1);
        let down_right = (curr_sand.0 + 1, curr_sand.1 + 1);

        match (
            material.get(&down),
            material.get(&down_left),
            material.get(&down_right),
        ) {
            (Some(_), Some(_), Some(_)) => {
                material.insert(curr_sand);
                curr_sand = (500, 0);
            }
            (None, _, _) => curr_sand = down,
            (_, None, _) => curr_sand = down_left,
            (_, _, None) => curr_sand = down_right,
        };
    }

    (material.len() - rocks_without_sand).to_string()
}

fn process_part2(input: &str) -> String {
    let (_, mut material) = rocks(input).unwrap();
    let rocks_without_sand = material.len();

    let material_clone = material.clone();
    let mut material_vec = material_clone.iter().collect::<Vec<&(u32, u32)>>();
    material_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let lowest_rock = material_vec.last().unwrap();

    let mut curr_sand = (500, 0);
    while let None = material.get(&(500, 0)) {
        let down = (curr_sand.0, curr_sand.1 + 1);
        let down_left = (curr_sand.0 - 1, curr_sand.1 + 1);
        let down_right = (curr_sand.0 + 1, curr_sand.1 + 1);

        match (
            material.get(&down).or_else(|| {
                if down.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            material.get(&down_left).or_else(|| {
                if down_left.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            material.get(&down_right).or_else(|| {
                if down_right.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
        ) {
            (Some(_), Some(_), Some(_)) => {
                material.insert(curr_sand);
                curr_sand = (500, 0);
            }
            (None, _, _) => curr_sand = down,
            (_, None, _) => curr_sand = down_left,
            (_, _, None) => curr_sand = down_right,
        };
    }

    (material.len() - rocks_without_sand).to_string()
}

#[test]
fn p1() {
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(process_part1(INPUT), "24");
}
