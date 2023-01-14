use std::fs;
use std::collections::{HashSet, VecDeque};

mod types;

use types::{Direction, Pos, Map, Valley, Blizzard};
use types::Direction::*;
use types::Tile::*;

fn print_grid(valley: &Valley, blizzard: &Blizzard) {
    for i in 0..valley.len() {
        for j in 0..valley[0].len() {
            if is_border(i, j, valley) {
                print!("{:?}", valley[i][j]);
            } else {
                if blizzard[i][j].is_some() {
                    print!("{:?}", blizzard[i][j].unwrap());
                }
            }
        }
        println!("");
    }
}

fn is_border(i: usize, j: usize, valley: &Valley) -> bool {
    if i == 0 || j == 0 {
        return true;
    }
    if i == valley.len() - 1 || j == valley[0].len() - 1 {
        return true;
    }
    false
}

fn main() {
    let input = fs::read_to_string("24-twenty-four/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one: {}", part_one);
    let part_two = part_two(&input);
    println!("part two: {}", part_two);
}

fn part_one(input: &str) -> usize {
   let (valley, blizzard) = parse_valley(input);
   let start = Pos(1, 0);
   let end = Pos((valley.width() - 2) as i32, (valley.height() - 1) as i32);
   search(&valley, &blizzard, start, end, 0).unwrap()
}

fn part_two(input: &str) -> usize {
   let (valley, blizzard) = parse_valley(input);
   let start = Pos(1, 0);
   let end = Pos((valley.width() - 2) as i32, (valley.height() - 1) as i32);
   let first_stop = search(&valley, &blizzard, start, end, 0).unwrap();
   println!("step {}", first_stop);
   let second_stop = search(&valley, &blizzard, end, start, first_stop).unwrap();
   println!("step {}", second_stop);
   search(&valley, &blizzard, start, end, second_stop).unwrap()
}

fn search(
    valley: &Valley,
    blizzard: &Blizzard,
    start: Pos,
    end: Pos,
    tick: usize,
) -> Option<usize> {
    const DIRECTIONS: [Direction; 4] = [
        Right,
        Down,
        Left,
        Up,
    ];

    let mut queue: VecDeque<(usize, Pos)> = VecDeque::new();
    queue.push_back((tick, start));

    let mut visited: HashSet<(usize, Pos)> = HashSet::new();
    visited.insert((tick, start));

    while let Some((tick, pos)) = queue.pop_front() {
        if pos == end {
            return Some(tick);
        }

        for delta in DIRECTIONS {
            let new_pos = pos + delta.into();
            if let Some(tile) = valley.get_y_x(new_pos.1 as usize, new_pos.0 as usize) {
                if *tile == Empty
                    && !is_blizzard_blocking(new_pos, blizzard, tick + 1)
                    && !visited.contains(&(tick + 1, new_pos))
                {
                    visited.insert((tick + 1, new_pos));
                    queue.push_back((tick + 1, new_pos));
                }
            }
        }

        if !is_blizzard_blocking(pos, blizzard, tick + 1) && !visited.contains(&(tick + 1, pos)) {
            visited.insert((tick + 1, pos));
            queue.push_back((tick + 1, pos));
        }
    }
    None
}


fn is_blizzard_blocking(pos: Pos, blizzard: &Blizzard, tick: usize) -> bool {
    if pos.1 < 1 || pos.1 > blizzard.height() as i32 {
        return false;
    }
    let (x, y) = (pos.0 - 1, pos.1 - 1);

    let blizzard_possible = [
        ((x, y - tick as i32), Down),
        ((x, y + tick as i32), Up),
        ((x - tick as i32, y), Right),
        ((x + tick as i32, y), Left),
    ];

    blizzard_possible
    .iter()
    .map(|((x, y), direction)| {
        let blizzard_x = (*x as usize) % blizzard.width();
        let blizzard_y = (*y as usize) % blizzard.height();
        (
            blizzard[blizzard_y][blizzard_x],
            direction,
        )
    })
    .any(|(blizzard_direction, expected)| blizzard_direction.is_some() && blizzard_direction.unwrap() == *expected)
}

fn parse_valley(input: &str) -> (Valley, Blizzard) {
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let mut valley: Valley = vec![vec![Empty; lines[0].len()]; lines.len()];
    let mut blizzard: Blizzard = vec![vec![None; lines[0].len() - 2]; lines.len() - 2];

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '.' => valley[y][x] = Empty,
                '#' => valley[y][x] = Wall,
                '^' => blizzard[y - 1][x - 1] = Some(Up),
                'v' => blizzard[y - 1][x - 1] = Some(Down),
                '<' => blizzard[y - 1][x - 1] = Some(Left),
                '>' => blizzard[y - 1][x - 1] = Some(Right),
                _ => unreachable!(),
            };
        }
    }
    (valley, blizzard)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const INPUT: &str = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(18, res);
    }

    #[test]
    fn test_part_two() {
        let res = part_two(INPUT);
        assert_eq!(54, res);
    }
}
