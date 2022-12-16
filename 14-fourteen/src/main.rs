use std::fs;

type Position = (i32, i32);
type Structure = Vec<Position>;
type Structures = Vec<Structure>;
type Map = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string("14-fourteen/input.txt").unwrap();
    part_one(&input);
}

fn print_map(map: &Map) {
   for line in map {
    let current: String = line.iter().collect();
     println!("{current}"); 
  } 
}

fn part_one(input: &str) -> usize {
    let (structs, source) = parse_structs(input);
    let map = create_map(&structs);
    let map = place_rocks(&structs, map);
    let map = place_sand_source(map, source);
    print_map(&map);
    let map = fall_sand(map, source);

    todo!()
}

fn fall_sand(map: Map, source: usize) -> Map {
    let mut y = map[0].len();
    loop {
        if map[source][y] == 'o' || map[source][y] == '#' {
            // try lower left and  lower right
            // if cannot
            break;
        }
    };
    map
}

fn place_sand_source(mut map: Map, source: usize) -> Map {
    map[0][source] = '+';
    map
}

fn place_rocks(structs: &Structures, mut map: Map) -> Map {
    for command in structs {
        for i in 0..(command.len() - 1) {
            let point = command[i];
            let next_point = command[i+1];
            let mut point = (point.0 as usize, point.1 as usize);
            let next_point = (next_point.0 as usize, next_point.1 as usize);
            while point != next_point {
                map[point.0][point.1] = '#';
                if point.0 < next_point.0 {
                    point.0 += 1;
                }
                if point.0 > next_point.0 {
                    point.0 -= 1;
                }
                if point.1 < next_point.1 {
                    point.1 += 1;
                }
                if point.1 > next_point.1 {
                    point.1 -= 1;
                }
            }
        }
    }
    map
}

fn create_map(structures: &Structures) -> Map {
    let (max_x, max_y) = find_max(&structures);
    // println!("{}. {}", max_x, max_y);
    let mut lines: Vec<Vec<char>> = Vec::with_capacity(max_x as usize);
    for _ in 0..max_x {
        lines.push(vec![' '; max_y]);
    }
    lines
}

fn parse_structs(input: &str) -> (Structures, usize) {
    let structs: Structures = parse_inputs(input);
    let (min_x, _) = find_min(&structs);
    let structures = structs
        .iter()
        .map(|line| line
             .iter()
             .map(|(x, y)| (x - min_x, *y))
             .collect()
        )
        .collect();
    (structures, 500-min_x as usize)
}

fn parse_inputs(input: &str) -> Structures {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Structure {
    line
        .split(" -> ")
        .map(|segment| segment.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

fn find_min(structs: &Structures) -> Position {
    let min_x = structs.iter().map(|line| line.iter().map(|(x, _)| x).min().unwrap()).min().unwrap_or(&0);
    let min_y = structs.iter().map(|line| line.iter().map(|(_, y)| y).min().unwrap()).min().unwrap_or(&0);
    (*min_x, *min_y)
}

fn find_max(structs: &Structures) -> (usize, usize) {
    let max_x: usize = structs.iter().map(|line| line.iter().map(|(x, _)| *x as usize).max().unwrap()).max().unwrap_or(0);
    let max_y: usize = structs.iter().map(|line| line.iter().map(|(_, y)| *y as usize).max().unwrap()).max().unwrap_or(0);
    (max_x + 1, max_y + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::vec_eq;

    #[test]
    fn test_parse_line() {
        const LINE: &str = "498,4 -> 498,6 -> 496,6";
        let res = parse_line(LINE);
        let expected = vec![(498, 4), (498, 6), (496, 6)];
        assert!(vec_eq(expected, res));
    }
}
