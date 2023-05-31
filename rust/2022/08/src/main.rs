use std::fs;

fn get_trees(file: &str) -> Vec<Vec<u32>> {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    for row in file.lines() {
        let mut new_row: Vec<u32> = Vec::new();
        for tree in row.chars() {
            new_row.push(tree.to_digit(10).unwrap());
        }
        trees.push(new_row);
    }
    trees
}

fn get_score_for_row(tree: u32, row: &[u32], reverse_iterate: bool) -> u32 {
    let mut score: u32 = 0;
    if reverse_iterate {
        for i in (0..row.len()).rev() {
            let next_tree = row[i];
            if next_tree <= tree {
                score += 1;
            }

            if next_tree >= tree {
                break;
            }
        }
    } else {
        for i in 0..row.len() {
            let next_tree = row[i];
            score += 1;

            if next_tree >= tree {
                break;
            }
        }
    }

    score
}

fn get_score(row: usize, column: usize, trees: &Vec<Vec<u32>>) -> u32 {
    let tree = trees[row][column];
    let left_row = &trees[row][..column];
    let right_row = &trees[row][column + 1..];
    let mut upper_col: Vec<u32> = vec![];
    for i in 0..row {
        upper_col.push(trees[i][column])
    }
    let mut lower_col: Vec<u32> = vec![];
    for i in row + 1..trees.len() {
        lower_col.push(trees[i][column])
    }

    let left_score = get_score_for_row(tree, left_row, true);
    let right_score = get_score_for_row(tree, right_row, false);
    let upper_score = get_score_for_row(tree, &upper_col, true);
    let lower_score = get_score_for_row(tree, &lower_col, false);

    left_score * right_score * upper_score * lower_score
}

fn process(file: &str) -> u32 {
    let trees = get_trees(file);
    let mut scores = vec![];

    for (row, row_trees) in trees.iter().enumerate() {
        for (column, _) in row_trees.iter().enumerate() {
            scores.push(get_score(row, column, &trees));
        }
    }

    scores.into_iter().max().unwrap()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = process(&file);
    println!("{result}")
}

#[test]
fn passes() {
    const INPUT: &str = "30373
25512
65332
33549
35390";

    let size = process(INPUT);
    assert_eq!(size, 8);
}
