fn main() {
    let input = fs::read_to_string("23-twenty-three/input.txd").unwrap();
    let part_one = part_one(&input);
    println!("part one {}", part_one);
    let part_two = part_two(&input);
    println!("part two {}", part_two);
}

fn part_one(input: &str) -> i64 {
    todo!()
}

fn part_two(input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(110, res);
    }
}
