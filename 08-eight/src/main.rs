use std::{fs, ops::Add};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tree {
    height: u8,
    visible: bool,
    viewving_distance: usize,
}

impl Tree {
    fn new(height: char) -> Self {
        Tree {
            visible: true,
            height: height.to_digit(10).unwrap() as u8,
            viewving_distance: 0,
        }
    }
}

type Forest = Vec<Vec<Tree>>;

fn create_forest(input: &str) -> Forest {
    input
        .lines()
        .map(|line| line.chars().map(Tree::new).collect())
        .collect()
}

fn flag_trees(input: &str) -> Forest {
    let mut forest = create_forest(input);
    let (row, col) = (forest.len(), forest[0].len());
    for i in 1..row {
        for j in 1..col {
            calc_viewing_distance(i, j, row, col, &mut forest);
            if is_hidden_tree(i, j, row, col, &forest) {
                let mut tree = &mut forest[i][j];
                tree.visible = false;
            }
        }
    }
    forest
}

fn calc_viewing_distance(i: usize, j: usize, row: usize, col: usize, forest: &mut Forest) {
    let tree = &forest[i][j];
    let mut scores = [0, 0, 0, 0];
    // scores[0] = (0..i).rev().take_while(|k| tree.height > forest[*k][j].height).count().add(1);
    // scores[1] = ((i+1)..row).take_while(|k| tree.height > forest[*k][j].height).count().add(1);
    // scores[2] = (0..j).rev().take_while(|k| tree.height > forest[*k][j].height).count().add(1);
    // scores[3] = (j..col).take_while(|k| tree.height > forest[*k][j].height).count().add(1);
    for k in (0..(i)).rev() {
         if tree.height > forest[k][j].height {
            scores[0] += 1;
        } else {
            scores[0] += 1;
            break;
        }
    }
    for k in (i+1)..row {
         if tree.height > forest[k][j].height {
            scores[1] += 1;
        } else {
            scores[1] += 1;
            break;
        }
    }
    for k in (0..(j)).rev() {
         if tree.height > forest[i][k].height {
            scores[2] += 1;
        } else {
            scores[2] += 1;
            break;
        }
    }
    for k in (j+1)..col {
         if tree.height > forest[i][k].height {
            scores[3] += 1;
        } else {
            scores[3] += 1;
            break;
        }
    }

    let mut tree = &mut forest[i][j];
    tree.viewving_distance = scores.iter().filter(|x| **x > 0).product();
}

fn is_visible_from_on_row(
    start: usize,
    col: usize,
    end: usize,
    forest: &Forest,
    tree: &Tree,
) -> bool {
    (start..end).all(|i| forest[i][col].height < tree.height)
}

fn is_visible_from_on_line(
    row: usize,
    start: usize,
    end: usize,
    forest: &Forest,
    tree: &Tree,
) -> bool {
    (start..end).all(|i| forest[row][i].height < tree.height)
}

fn is_hidden_tree(
    row: usize,
    col: usize,
    row_count: usize,
    col_count: usize,
    forest: &Forest,
) -> bool {
    let tree = &forest[row][col];
    let bottom = !is_visible_from_on_row(row + 1, col, row_count, forest, tree);
    let top = !is_visible_from_on_row(0, col, row, forest, tree);
    let left = !is_visible_from_on_line(row, 0, col, forest, tree);
    let right = !is_visible_from_on_line(row, col + 1, col_count, forest, tree);
    right && left && top && bottom
}

fn count_tree_visible(forest: &Forest) -> usize {
    forest.iter().flatten().filter(|tree| tree.visible).count()
}

fn main() {
    let input = fs::read_to_string("08-eight/input.txt").unwrap();
    let forest = flag_trees(&input);
    let count_visible = count_tree_visible(&forest);
    println!("part one : {}", count_visible);
    let max_render_distance = forest.iter().flatten().map(|tree| tree.viewving_distance).max();
    println!("part two : {}", max_render_distance.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::vec_eq;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_one() {
        let forest = flag_trees(INPUT);
        let number_of_tree_visible = count_tree_visible(&forest);
        assert_eq!(21, number_of_tree_visible);
    }

    #[test]
    fn test_tree_viewing_distance() {
        let mut tree_plan: Forest = create_forest(INPUT);
        calc_viewing_distance(3, 2, 5, 5, &mut tree_plan);
        let center_bottom = tree_plan[3][2].clone();
        let expected = Tree {
            height: 5,
            visible: true,
            viewving_distance: 8,
        };
        assert_eq!(expected, center_bottom);

        calc_viewing_distance(1, 2, 5, 5, &mut tree_plan);
        let top_center = tree_plan[1][2].clone();
        let expected = Tree {
            height: 5,
            visible: true,
            viewving_distance: 4,
        };
        assert_eq!(expected, top_center);
    }

    #[test]
    fn test_forest_creation() {
        let expected = vec![
            "30373".chars().map(Tree::new).collect::<Vec<Tree>>(),
            "25512".chars().map(Tree::new).collect::<Vec<Tree>>(),
            "65332".chars().map(Tree::new).collect::<Vec<Tree>>(),
            "33549".chars().map(Tree::new).collect::<Vec<Tree>>(),
            "35390".chars().map(Tree::new).collect::<Vec<Tree>>(),
        ];
        let res = create_forest(INPUT);
        assert!(vec_eq(expected, res));
    }

    #[test]
    fn test_is_hidden_tree() {
        let forest = create_forest(INPUT);
        assert!(is_hidden_tree(3, 1, 5, 5, &forest));
        assert!(is_hidden_tree(3, 3, 5, 5, &forest));
        assert!(is_hidden_tree(1, 3, 5, 5, &forest));
        assert!(is_hidden_tree(2, 2, 5, 5, &forest));
        assert!(is_hidden_tree(3, 2, 5, 5, &forest) == false);
    }

    #[allow(unused)]
    fn print_forest(forest: &Forest) {
        for i in 1..(forest.len() - 1) {
            for j in 1..(forest[i].len() - 1) {
                println!(
                    "{} {}: {} {}",
                    i,
                    j,
                    forest[i][j].height,
                    is_hidden_tree(i, j, 4, 4, &forest)
                );
            }
        }
    }
}
