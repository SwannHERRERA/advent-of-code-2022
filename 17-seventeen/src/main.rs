use std::collections::{BTreeMap, HashSet};
use std::iter::{Cycle, Enumerate};
use std::slice::Iter;
use std::{cmp, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Action {
    Left,
    Right,
}

impl From<char> for Action {
    fn from(value: char) -> Self {
        match value {
            '<' => Action::Left,
            '>' => Action::Right,
            _ => panic!("Unexpected movement!"),
        }
    }
}

type Rock = Vec<(i64, i64)>;
type Ground = Vec<(i64, i64)>;

const NUMBER_OF_COLS: i64 = 7;
const CACHE_SIZE: usize = 20;

fn main() {
    let input = fs::read_to_string("17-seventeen/input.txt").unwrap();
    let input = input.trim();
    let rocks: Vec<Rock> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let actions: Vec<Action> = input.chars().map(|x| x.into()).collect();
    let part_one = resolve(2022, &actions, &rocks);
    let part_two = resolve(1_000_000_000_000, &actions, &rocks);
    println!("part one = {}", part_one);
    println!("part two = {}", part_two);
}

fn resolve(num_rocks: i64, actions: &Vec<Action>, rocks: &Vec<Rock>) -> i64 {
    let mut filled_cells: HashSet<(i64, i64)> = HashSet::new();
    let mut cache: BTreeMap<(i64, usize, Ground), (i64, i64)> = BTreeMap::new();
    let mut max_height = 0;
    let mut height_gain_by_cache = 0;
    let mut count = num_rocks;
    let mut actions_cycle = actions.iter().enumerate().cycle();
    let mut rocks_cycle = rocks.iter().enumerate().cycle();
    let mut action_index = 0;

    while count > 0 {
        let (rock_index, next_rock) = rocks_cycle.next().unwrap();
        (action_index, max_height) = place_rock(
            &mut filled_cells,
            action_index,
            max_height,
            &mut actions_cycle,
            next_rock,
        );
        count -= 1;
        let Some(ground) = find_ground(&filled_cells, max_height) else {
            continue;
        };
        if let Some((old_max, old_count)) = cache.get(&(action_index, rock_index, ground.clone())) {
            height_gain_by_cache += (max_height - old_max) * (count / (old_count - count));
            count %= old_count - count;
        }
        cache.insert((action_index, rock_index, ground), (max_height, count));
    }
    return max_height + height_gain_by_cache;
}

fn place_rock(
    filled_cells: &mut HashSet<(i64, i64)>,
    action_index: i64,
    max_y: i64,
    actions: &mut Cycle<Enumerate<Iter<Action>>>,
    rock: &Rock,
) -> (i64, i64) {
    let mut x = 2;
    let mut y = max_y + 5;
    let mut new_action_index = action_index;
    while rock_is_movable(filled_cells, x, y - 1, rock) {
        y -= 1;
        let (idx, action) = actions.next().unwrap();
        match action {
            Action::Left => {
                if rock_is_movable(filled_cells, x - 1, y, rock) {
                    x -= 1;
                }
            }
            Action::Right => {
                if rock_is_movable(filled_cells, x + 1, y, rock) {
                    x += 1;
                }
            }
        }
        new_action_index = idx as i64;
    }
    let final_rock_position: Rock = rock.iter().map(|(dx, dy)| (x + dx, y + dy)).collect();
    final_rock_position.iter().for_each(|cell| {
        filled_cells.insert(*cell);
    });

    let top_of_the_rock = final_rock_position.iter().map(|(_, y)| *y).max().unwrap();
    (new_action_index, cmp::max(max_y, top_of_the_rock))
}

fn find_ground(placed: &HashSet<(i64, i64)>, max_y: i64) -> Option<Ground> {
    let mut state: HashSet<(i64, i64)> = HashSet::new();
    for x in 0..NUMBER_OF_COLS {
        search_border(x, 0, &mut state, max_y, placed);
    }
    if state.len() <= CACHE_SIZE {
        return Some(state.into_iter().collect());
    }
    None
}

fn search_border(
    x: i64,
    y: i64,
    visited: &mut HashSet<(i64, i64)>,
    max_y: i64,
    filled_cells: &HashSet<(i64, i64)>,
) {
    if (!is_empty(filled_cells, x, max_y + y))
        || visited.contains(&(x, y))
        || visited.len() > CACHE_SIZE
    {
        return;
    }
    visited.insert((x, y));
    vec![(x - 1, y), (x + 1, y), (x, y - 1)]
        .iter()
        .for_each(|(nx, ny)| {
            search_border(*nx, *ny, visited, max_y, filled_cells);
        });
}

fn rock_is_movable(placed: &HashSet<(i64, i64)>, x: i64, y: i64, rock: &Rock) -> bool {
    rock.iter().all(|(dx, dy)| is_empty(placed, x + dx, y + dy))
}

fn is_empty(placed: &HashSet<(i64, i64)>, x: i64, y: i64) -> bool {
    (y > 0) && (x >= 0) && (x < NUMBER_OF_COLS) && !placed.contains(&(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> (Vec<Action>, Vec<Rock>) {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let actions: Vec<Action> = input.chars().map(|c| c.into()).collect();
        let rocks: Vec<Rock> = vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ];
        (actions, rocks)
    }

    #[test]
    fn test_with_2022() {
        let (actions, rocks) = sample_data();
        let res = resolve(2022, &actions, &rocks);
        assert_eq!(3068, res);
    }

    #[test]
    #[ignore = "to long"]
    fn test_with_1_000_000_000_000() {
        let expected: i64 = 1514285714288;
        let (actions, rocks) = sample_data();
        let res = resolve(1_000_000_000_000, &actions, &rocks);
        assert_eq!(expected, res);
    }
}
