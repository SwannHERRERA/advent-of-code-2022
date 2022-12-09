use std::{fs, collections::HashSet};

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

type Position = (i32, i32);

type Quantity = usize;
type Instruction = (Direction, Quantity);
type Insctructions = Vec<Instruction>;

fn parse_instruction(instruction: (&str, &str)) -> Instruction {
    let direction = instruction.0.chars().last().unwrap();
    let quantity: Quantity = instruction.1.parse().unwrap();
    let direction = match direction {
        'U' => Direction::Up,
        'R' => Direction::Right,
        'L' => Direction::Left,
        'D' => Direction::Down,
        _ => unreachable!(),
    };
    return (direction, quantity)
}

fn main() {
    let input = fs::read_to_string("09-nine/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {part_one}");
    let part_two = part_two(&input);
    println!("part two : {part_two}");
}

fn part_one(input: &str) -> usize {
    let mut position_discovery: HashSet<Position> = HashSet::new();
    let instructions: Insctructions = input.lines().map(|line| {
        let instruction = line.split_once(' ').unwrap();
        parse_instruction(instruction)
    }).collect();
    let mut head: Position = (0, 0);
    let mut tail: Position = (0, 0);
    position_discovery.insert(tail);

    for (direction, quantity) in instructions {
        for _ in 0..quantity {
            move_head(&direction, &mut head);
            if let Some(new_tail_position) = compute_future_position_of_the_tail(head, tail) {
                tail = new_tail_position;
                position_discovery.insert(tail);
            }
        }
    }
    position_discovery.len()
}

fn move_head(direction: &Direction, head: &mut Position) {
    match direction {
        Direction::Up => head.0 += 1,
        Direction::Right => head.1 += 1,
        Direction::Left => head.1 -= 1,
        Direction::Down => head.0 -= 1,
    }
}

fn compute_future_position_of_the_tail(head: Position, tail: Position) -> Option<Position> {
    if !is_tail_far_away(tail, head) {
        return None;
    }
    let mut diff_on_y = head.0 - tail.0;
    let mut diff_on_x = head.1 - tail.1;
    if diff_on_y > 1 {
        diff_on_y -= 1;
    }
    if diff_on_y < -1 {
        diff_on_y += 1;
    }
    if diff_on_x > 1 {
        diff_on_x -= 1;
    }
    if diff_on_x < -1 {
        diff_on_x += 1;
    }
    Some((tail.0 + diff_on_y, tail.1 + diff_on_x))
}

fn part_two(input: &str) -> usize {
    const TAIL_INDEX: usize = 9;
    let mut position_discovery: HashSet<Position> = HashSet::new();
    let instructions: Insctructions = input.lines().map(|line| {
        let instruction = line.split_once(' ').unwrap();
        parse_instruction(instruction)
    }).collect();

    let mut roots = [(0, 0); 10];
    position_discovery.insert((0, 0));

    for (direction, quantity) in instructions {
        for _ in 0..quantity {
            move_head(&direction, &mut roots[0]);
            for idx  in 1..10 {
                let predecesor = roots[idx-1];
                let element = &mut roots[idx];
                if let Some(new_root_position) = compute_future_position_of_the_tail(predecesor, *element) {
                    *element = new_root_position;
                    if idx == TAIL_INDEX {
                        position_discovery.insert(*element);
                    }
                }
            }
        }
    }
    position_discovery.len()
}

fn is_tail_far_away(tail: Position, head: Position) -> bool {
    let diff_on_y = head.0 - tail.0;
    let diff_on_x = head.1 - tail.1;
    diff_on_x > 1 || diff_on_x < -1 || diff_on_y > 1 || diff_on_y < -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_two() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let res = part_two(INPUT);
        assert_eq!(36, res);
    }
    
    #[test]
    fn test_part_one() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let res = part_one(INPUT);
        assert_eq!(13, res);
    }

    #[test]
    fn test_is_tail_far_away() {
        let head = (0,0);
        let tail = (0,0);
        assert!(!is_tail_far_away(tail, head));

        let head = (1,0);
        let tail = (0,0);
        assert!(!is_tail_far_away(tail, head));

        let head = (1,0);
        let tail = (0,1);
        assert!(!is_tail_far_away(tail, head));

        let head = (2,0);
        let tail = (0,1);
        assert!(is_tail_far_away(tail, head));

        let head = (0,0);
        let tail = (0,2);
        assert!(is_tail_far_away(tail, head));
    }
}
