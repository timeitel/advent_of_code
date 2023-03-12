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

fn is_tree_visible(row: usize, column: usize, trees: &Vec<Vec<u32>>) -> bool {
    let tree = trees[row][column];

    if row == 0 || row == trees.len() - 1 || column == 0 || column == trees[row].len() - 1 {
        return true;
    }

    let left_row = &trees[row][..column];
    if left_row.iter().all(|x| *x < tree) {
        return true;
    }

    let right_row = &trees[row][column + 1..];
    if right_row.iter().all(|x| *x < tree) {
        return true;
    }

    let mut upper_col: Vec<u32> = vec![];
    for i in 0..row {
        upper_col.push(trees[i][column])
    }
    if upper_col.iter().all(|x| *x < tree) {
        return true;
    }

    let mut lower_col: Vec<u32> = vec![];
    for i in row + 1..trees.len() {
        lower_col.push(trees[i][column])
    }
    if lower_col.iter().all(|x| *x < tree) {
        return true;
    }

    false
}

fn process(file: &str) -> u32 {
    let trees = get_trees(file);
    let mut visible_trees: u32 = 0;

    for (row, row_trees) in trees.iter().enumerate() {
        for (column, _) in row_trees.iter().enumerate() {
            if is_tree_visible(row, column, &trees) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
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
    assert_eq!(size, 21);
}
