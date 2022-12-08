use std::fs;

mod error;
mod prelude;
use crate::prelude::*;

mod resolver;
mod utils;
use resolver::*;

fn main() -> Result<()> {
    let input = fs::read_to_string("06-six/input.txt")?;
    let res_one = part_one(&input);
    println!("part one : {res_one}");
    let res_two = part_two(&input);
    println!("part two : {res_two}");
    Ok(())
}
