use std::{cell::RefCell, fs, rc::Rc};

mod error;
mod prelude;
use parser::Parser;
use prelude::*;
use types::ElveFile;
mod builder;
mod parser;
mod types;

const LIMIT_SIZE_PART_ONE: usize = 100_000;
const FS_TOTAL_SIZE: usize = 70_000_000;
const FREE_SPACE_NEEDED: usize = 30_000_000;
const LIMIT_ACCEPTABLE: usize = 28_000_000;

fn main() -> Result<()> {
    let input = fs::read_to_string("07-seven/input.txt")?;
    // let res_1 = part_one(&input)?;
    // println!("part one : {}", res_1);
    let res_2 = part_two(&input)?;
    println!("part two : {}", res_2);
    Ok(())
}

#[allow(unused)]
fn part_one(input: &str) -> Result<usize> {
    let mut parser = Parser::new();
    let root = parser.parse_commands(input)?;
    let sizes: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(Vec::with_capacity(40)));
    get_sizes(root, sizes.clone());
    let sizes = sizes.take();
    let sizes: Vec<usize> = sizes
        .iter()
        .filter(|item| **item < LIMIT_SIZE_PART_ONE)
        .copied()
        .collect();
    Ok(sizes.iter().sum())
}

fn part_two(input: &str) -> Result<usize> {
    let mut parser = Parser::new();
    let root = parser.parse_commands(input)?;
    let used_space: usize = match &root {
        ElveFile::File(_) => unreachable!(),
        ElveFile::Folder(folder) => folder.borrow().size.unwrap(),
    };
    let sizes: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(Vec::with_capacity(40)));
    get_sizes(root, sizes.clone());
    let mut sizes = sizes.take();
    sizes.sort();
    let current_free_space = FS_TOTAL_SIZE - used_space;
    let sizes: Vec<usize> = sizes
        .iter()
        .filter(|item| current_free_space + **item >= FREE_SPACE_NEEDED)
        .copied()
        .collect();
    let min = sizes.iter().min().unwrap();
    println!("current_free_space: {}", current_free_space);
    println!("used_space: {}", used_space);
    println!("min: {}", min);
    println!("{:?}", sizes);
    println!("size: {}", sizes.len());

    // let added_sizes: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(Vec::with_capacity(40)));
    // // let max_under_limit = sizes.iter().filter(|size| **size < LIMIT_SIZE).max().unwrap();
    // let sizes = Rc::new(sizes);
    // subset_sum(added_sizes.clone(), sizes, 0, 0);
    // // println!("{:?}", added_sizes);
    // let sizes = added_sizes.take();
    // println!("{:?}", sizes);
    // let max = sizes.iter().max();
    Ok(*min)
}

#[allow(unused)]
fn subset_sum(
    tab: Rc<RefCell<Vec<usize>>>,
    source: Rc<Vec<usize>>,
    current_sum: usize,
    iteration: usize,
) {
    if iteration > source.len() || current_sum + source[iteration] > FREE_SPACE_NEEDED {
        return;
    }
    for i in iteration..source.len() {
        if current_sum + source[i] > FREE_SPACE_NEEDED {
            continue;
        }
        if current_sum + source[i] > LIMIT_ACCEPTABLE {
            let mut tab_copy = tab.borrow_mut();
            tab_copy.push(current_sum + source[i]);
        }
        subset_sum(
            tab.clone(),
            source.clone(),
            current_sum + source[i],
            iteration + 1,
        );
    }
}

fn get_sizes(file: ElveFile, sizes: Rc<RefCell<Vec<usize>>>) {
    match file {
        ElveFile::File(_file) => {
            // let mut sizes = sizes.borrow_mut();
            // sizes.push(file.size);
        }
        ElveFile::Folder(folder) => {
            let folder = folder.borrow();
            let mut sizes_copy = sizes.borrow_mut();
            sizes_copy.push(folder.size.expect("folder doesn't have size"));
            drop(sizes_copy);
            for file in &folder.files {
                get_sizes(file.clone(), sizes.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_max_addition_under_limit() {
        let input: Vec<usize> = vec![584, 94853, 24933642, 48381165];
        let max: usize = input.iter().filter(|x| **x < LIMIT_SIZE_PART_ONE).sum();
        assert_eq!(95437, max);
    }

    #[test]
    fn test_find_max_part2() {
        let input: Vec<usize> = vec![584, 94853, 24933642, 48381165];
        let sizes: Vec<usize> = input
            .iter()
            .filter(|x| **x < FREE_SPACE_NEEDED)
            .copied()
            .collect();
        let max = sizes.iter().max().unwrap();
        assert_eq!(24933642, *max);
    }
}
