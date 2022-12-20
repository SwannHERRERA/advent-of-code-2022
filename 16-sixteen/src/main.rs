use std::fs;
use graph::Graph;
use valves::Valves;

pub const TIME_TO_TEACH_ELEPHANT: isize = 4;
pub const START_VALVE: &str = "AA";

mod valves;
mod graph;

fn main() {
    let input = fs::read_to_string("16-sixteen/input.txt").unwrap();
    let time = std::time::Instant::now();
    println!("Part two: {}", part_one(&input, 30));
    println!("Time: {}ms", time.elapsed().as_millis());
    let time = std::time::Instant::now();
    println!("Part two: {}", part_two(&input, 30));
    println!("Time: {}ms", time.elapsed().as_millis());
}

fn part_one(input: &str, minutes: isize) -> isize {
    let valves: Valves = input.parse().unwrap();
    let graph = Graph::new(&valves);
    // println!("{:?}", graph);
    graph.find_best_path(minutes)
}

fn part_two(input: &str, minutes: isize) -> isize {
    let valves: Valves = input.parse().unwrap();
    let graph = Graph::new(&valves);
    graph.find_best_path_with_elephant(minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1() {
        const EXPECTED: isize = 1651;
        assert_eq!(EXPECTED, part_one(INPUT, 30));
        assert!(false);
    }
    #[test]
    fn test_part2() {
        const EXPECTED: isize = 1707;
        assert_eq!(EXPECTED, part_two(INPUT, 30));
    }
}
