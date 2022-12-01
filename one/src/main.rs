use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("one/data.txt")?;
    let mut buffer = BufReader::new(file);
    let mut content = String::new();
    let _read_to_stirng_result = buffer.read_to_string(&mut content)?;
    let elve_inventories: Vec<&str> = content.split("\n\n").collect();

    let sums: Vec<u32> = elve_inventories.into_iter().map(|inventory| {
        inventory.lines().map(|s| s.parse::<u32>().expect("line is number")).sum()
    }).collect();
    let max = sums.into_iter().max_by(|a, b| a.cmp(b));

    println!("{}", max.unwrap());

    Ok(())
}
