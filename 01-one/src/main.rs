use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("01-one/data.txt")?;
    let mut buffer = BufReader::new(file);
    let mut content = String::new();
    let _read_to_stirng_result = buffer.read_to_string(&mut content)?;
    let elve_inventories: Vec<&str> = content.split("\n\n").collect();

    let mut sums: Vec<u32> = elve_inventories
        .into_iter()
        .map(|inventory| {
            inventory
                .lines()
                .map(|s| s.parse::<u32>().expect("line is number"))
                .sum()
        })
        .collect();
    sums.sort_by(|a, b| b.cmp(a));

    let three_best_player: u32 = sums.iter().take(3).sum();
    println!("{}", three_best_player);

    Ok(())
}
