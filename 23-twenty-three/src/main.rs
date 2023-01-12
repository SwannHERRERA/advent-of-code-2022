use std::{fs, collections::{HashSet, HashMap}};

fn main() {
    let input = fs::read_to_string("23-twenty-three/input.tx").unwrap();
    let part_one = part_one(&input);
    println!("part one {}", part_one);
    // let part_two = part_two(&input);
    // println!("part two {}", part_two);
}

fn part_one(input: &str) -> i64 {
    const ROUNDS: usize = 10;
    let mut elves = parse_input(input);
    for _ in 0..ROUNDS {
        let next_move: HashMap<(i64, i64), (i64, i64)> = elves
            .iter()
            .filter(|&&pos| elve_should_move(pos, &elves))
            .map(|&pos| compute_next_position(pos, &elves))
            .collect();
        elves = next_move
            .iter()
            .filter(|(key, value)| !next_move
                .iter()
                .any(|(k, v)| {
                    value.0 == v.0 && value.1 == v.1 
                    && !(k.0 == key.0 && k.1 == key.1)
                })
            )
            .map(|(_k , v)| v)
            .copied()
            .collect();
    }
    // il y a peut être des movement que j'ai pas
    // il faut trouver le min en x le max en x same pour y
    // une fois que j'ai ça il faut faire for x in x..xmax for y in y..ymax filter out ceux
    // qui sont dedans et count
    todo!()
}

fn part_two(input: &str) -> i64 {
    todo!()
}

fn parse_input(input: &str) -> HashSet<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| line
            .chars()
            .enumerate()
            .filter(|(_j, c)| *c == '#')
            .map(move |(j, _c)| (i as i64, j as i64))
        )
        .flatten()
        .collect()
}

fn elve_should_move((x, y): (i64, i64), set: &HashSet<(i64, i64)>) -> bool {
    const NEIGHBOUR_POS: [(i64, i64); 8] = [(1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1)];
    NEIGHBOUR_POS.iter().all(|(ax, ay)| !set.contains(&(ax + x, ay + y))) 
}

fn compute_next_position((x, y): (i64, i64), set: &HashSet<(i64, i64)>) -> ((i64, i64), (i64, i64)) {
    // TODO operate for finding the good direction and then operate a calcul for find the next pos
    ((x, y), (x + 1, y + 1))
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
