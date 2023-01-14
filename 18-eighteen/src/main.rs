use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("18-eighteen/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one: {}", part_one);
    let part_two = part_two(&input);
    println!("part two: {}", part_two);
}

type Position = (i64, i64, i64);

fn parse_input(input: &str) -> HashSet<Position> {
    input
        .lines()
        .map(|line| {
            let axies: Vec<i64> = line.split(',').map(|num| num.parse().unwrap()).collect();
            (axies[0], axies[1], axies[2])
        })
        .collect()
}

fn compute_neighbour((x, y, z): (i64, i64, i64)) -> Vec<Position> {
    let mut vec = Vec::with_capacity(6);
    vec.push((x + 1, y, z));
    vec.push((x - 1, y, z));
    vec.push((x, y + 1, z));
    vec.push((x, y - 1, z));
    vec.push((x, y, z + 1));
    vec.push((x, y, z - 1));
    vec
}

fn part_one(input: &str) -> usize {
    let set = parse_input(input);
    set.iter()
        .map(|axies| {
            let neighbours = compute_neighbour(*axies);
            neighbours
                .iter()
                .filter(|neighbour| !set.contains(&neighbour))
                .count()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let set = parse_input(input);
    set.iter()
        .map(|axies| {
            let neighbours = compute_neighbour(*axies);
            let exterior_count = neighbours
                .iter()
                .filter(|neighbour| !set.contains(&neighbour))
                .filter(|position| !is_interior_block(position, &set))
                .count();
            let interior_count = neighbours
                .iter()
                .filter(|neighbour| !set.contains(&neighbour))
                .filter(|position| is_interior_block(position, &set))
                .filter(|(x, y, z)| {
                    [
                        (*x - 1, *y, *z),
                        (*x + 1, *y, *z),
                        (*x, *y - 1, *z),
                        (*x, *y + 1, *z),
                        (*x, *y, *z - 1),
                        (*x, *y, *z + 1),
                    ]
                    .iter()
                    .filter(|position| set.get(position).is_none())
                    .any(|position| check_if_block_is_visible(*position, &set) >= 1)
                })
                .count();
            exterior_count + interior_count
        })
        .sum()
}

fn is_interior_block(&(x, y, z): &Position, set: &HashSet<Position>) -> bool {
    let upper = set
        .iter()
        .any(|position| position.0 > x && position.1 == y && position.2 == z);
    let lower = set
        .iter()
        .any(|position| position.0 < x && position.1 == y && position.2 == z);
    let left = set
        .iter()
        .any(|position| position.0 == x && position.1 > y && position.2 == z);
    let right = set
        .iter()
        .any(|position| position.0 == x && position.1 < y && position.2 == z);
    let comming = set
        .iter()
        .any(|position| position.0 == x && position.1 == y && position.2 > z);
    let outgoing = set
        .iter()
        .any(|position| position.0 == x && position.1 == y && position.2 < z);
    [upper, lower, left, right, comming, outgoing]
        .iter()
        .all(|v| *v)
}

fn check_if_block_is_visible((x, y, z): Position, points: &HashSet<Position>) -> usize {
    let x_low = (x - 1, y, z);
    let x_high = (x + 1, y, z);
    let y_low = (x, y - 1, z);
    let y_high = (x, y + 1, z);
    let z_low = (x, y, z - 1);
    let z_high = (x, y, z + 1);
    [x_low, x_high, y_low, y_high, z_low, z_high]
        .iter()
        .filter(|ivec| points.get(ivec).is_none())
        .filter(|ivec| !is_interior_block(&ivec, &points))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part_one() {
        let side_exposed = part_one(INPUT);
        assert_eq!(64, side_exposed);
    }

    #[test]
    fn test_part_two() {
        let side_exposed = part_two(INPUT);
        assert_eq!(58, side_exposed);
    }

    #[test]
    fn test_adjacent_cube() {
        const INPUT: &str = "1,1,1
2,1,1";
        let side_exposed = part_one(INPUT);
        assert_eq!(10, side_exposed);
    }
}
