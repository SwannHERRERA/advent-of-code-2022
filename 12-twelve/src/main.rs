use std::{fs, cell::RefCell, rc::Rc, collections::HashMap};

fn main() {
    let input = fs::read_to_string("12-twelve/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one: {}", part_one);
}

type Map = Vec<Vec<char>>;
type Solutions = Rc<RefCell<HashMap<(usize, usize) ,usize>>>;

fn part_one(input: &str) -> usize {
    let solutions: Solutions = Rc::new(RefCell::new(HashMap::new()));
    let map: Map = parse_input(input);
    let start = get_start_position(&map);
    let end = get_end_position(&map);
    let map: Map = remove_start_and_end(&map, start, end);
    println!("{:?}", map);
    find_path(solutions.clone(), &map, start, end, 0);
    let solutions = solutions.take();
    println!("{:?}", solutions);
    *solutions.get(&end).unwrap()
}

fn remove_start_and_end(map: &Map, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<char>> {
    let mut map = map.clone();
    map[start.0][start.1] = 'a';
    map[end.0][end.1] = 'z';
    return map;
}

fn find_path(solutions: Solutions, map: &Map, start: (usize, usize), end: (usize, usize), current_length: usize) {
    if start == end {
        let mut borrow_mut = solutions.borrow_mut();   
        let previous_response = borrow_mut.get(&end);
        if previous_response == None || *previous_response.unwrap() > current_length {
            borrow_mut.insert(end, current_length);
        }
        return;
    }

    let solution_getter = solutions.borrow();
    if let Some(previous_length) = solution_getter.get(&start) {
        if *previous_length < current_length {
            return;
        }
    }
    drop(solution_getter);
    let current_height = map[start.0][start.1];
    let len = map.len() as i32;
    let line_len = map[0].len() as i32;
    let neighbour = create_valid_neighbour(start, len, line_len);

    neighbour.iter().filter(|(x, y)| {
        let height = map[*x][*y];
        let height_diff = current_height as i16 - height as i16;
        is_clamped(height_diff, -1, 1)
    }).for_each(|new_start| {
        let mut solution_setter = solutions.borrow_mut();
        solution_setter.insert(start, current_length);
        drop(solution_setter);
        find_path(solutions.clone(), map, *new_start, end, current_length + 1);
    });
}

fn create_valid_neighbour(start: (usize, usize), x_len: i32, y_len: i32) -> Vec<(usize, usize)> {
    let (x, y): (i32, i32) = (start.0 as i32, start.1 as i32);
    let neighbour: [(i32, i32); 4] = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    neighbour
        .iter()
        .filter(|(x, y)| {
            *x >= 0 && *y >= 0 && *x < x_len && *y < y_len
        })
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
}

fn get_end_position(map: &Map) -> (usize, usize) {
    for (i, line) in map.iter().enumerate(){
        if let Some(j) = line.iter().position(|c| *c == 'E') {
            return (i, j)
        }
    } 
    unreachable!()
}

fn get_start_position(map: &Map) -> (usize, usize) {
    for (i, line) in map.iter().enumerate() {
        if let Some(j) = line.iter().position(|c| *c == 'S') {
            return (i, j)
        }
    } 
    unreachable!()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

pub fn is_clamped<T: PartialOrd>(num: T, min: T, max: T) -> bool {
    min <= num && num <= max
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(31, res);
    }
}
