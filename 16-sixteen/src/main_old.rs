use std::{rc::Rc, cell::RefCell, fs, collections::{HashMap, HashSet}};

#[derive(Debug, Clone)]
struct Valve<'a> {
    is_lock: bool,
    flow_rate: usize,
    tunnels: Vec<&'a str>
}

#[derive(Debug, Clone)]
struct VolcanoState {
    total_pressure_release: usize,
    minutes_remaining: u8,
    current_flow_rate: usize,
    valve_open: HashSet<String>,
}

impl VolcanoState {
    fn new() -> Self {
        VolcanoState {
            total_pressure_release: 0,
            minutes_remaining: 30,
            current_flow_rate: 0,
            valve_open: HashSet::new(),
        }
    }
}

struct Node {
    id: usize,
    time_to_be_reach: usize,
    release_potentiel: usize,
}

// Je vais faire un tableau avec la ligne a laquel je pars 

fn test(input: &str) {
    let node_count = input.lines().count();
    let state = VolcanoState::new();
    let adjacence: Vec<Vec<Option<usize>>> = Vec::with_capacity(node_count);
}

impl<'a> Valve<'a> {
    fn new(flow_rate: usize) -> Self {
        Valve { flow_rate, tunnels: Vec::new(), is_lock: true }
    }
}

fn main() {
    let input = fs::read_to_string("16-sixteen/input.txt").unwrap();
    let part_one = part_one(&input);
    println!("part one : {}", part_one);
}

fn part_one(input:&str) -> usize {
    let valves = parse_input(input);
    let valves = Rc::new(RefCell::new(valves));
    let result: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(Vec::new()));
    part_one_recu(result.clone(), valves, "AA", 0, 0);
    let res = result.take();
    println!("{:?}", res);
    *res.iter().max().unwrap()
}

fn part_one_recu<'a>(result: Rc<RefCell<Vec<usize>>>, valves: Rc<RefCell<HashMap<&'a str, Valve<'a>>>>, current_position: &str, minutes: u8, mut current_sum: usize) {
    eprintln!("{minutes}");
    if minutes >= 30 {
        compute_pressure_release(valves.clone(), result.clone(), current_sum);
        return;
    }
    let valve = find_valve(current_position, valves.clone());
    for tunnel in valve.tunnels {
        let new_valve = find_valve(tunnel, valves.clone());
        if new_valve.is_lock {
            current_sum += compute_pressure_release(valves.clone(), result.clone(), current_sum);
            unlock(tunnel, valves.clone());
            current_sum += compute_pressure_release(valves.clone(), result.clone(), current_sum);
            part_one_recu(result.clone(), valves.clone(), tunnel, minutes + 3, current_sum);
        } else {
            current_sum += compute_pressure_release(valves.clone(), result.clone(), current_sum);
            part_one_recu(result.clone(), valves.clone(), tunnel, minutes + 2, current_sum);
        }
    }
    if minutes < 30 {
        current_sum += compute_pressure_release(valves.clone(), result.clone(), current_sum);
        part_one_recu(result.clone(), valves.clone(), current_position, minutes + 1, current_sum);
    }
}

fn compute_pressure_release<'a>(valves: Rc<RefCell<HashMap<&str ,Valve<'a>>>>, result: Rc<RefCell<Vec<usize>>>, current_sum: usize) -> usize {
    let valves = valves.borrow();
    let sum = valves.values().filter(|v| !v.is_lock).map(|v| v.flow_rate).sum();
    let mut result = result.borrow_mut();
    result.push(sum + current_sum);
    sum
}

fn find_valve<'a>(name: &str, valves: Rc<RefCell<HashMap<&'a str, Valve<'a>>>>) -> Valve<'a> {
    let valves = valves.borrow();
    valves.get(name).unwrap().clone()
}

fn unlock<'a>(name: &str, valves: Rc<RefCell<HashMap<&str ,Valve<'a>>>>)  {
    let mut valves = valves.borrow_mut();
    let valve = valves.get_mut(name).unwrap();
    valve.is_lock = false;
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Valve<'a>> {
    input.lines().map(|line| {
        let (_, x) = line.split_once(' ').unwrap();
        let (name, x) = x.split_once(' ').unwrap();
        let (_, x) = x.split_once('=').unwrap();
        let (flow_rate, x) = x.split_once(';').unwrap();
        println!("{x}");
        let mut option = x.split_once("valves ");
        if option.is_none() {
            option = x.split_once("valve ");
        }
        let (_, connections) = option.unwrap();
        let flow_rate: usize = flow_rate.parse().unwrap();
        let mut valve = Valve::new(flow_rate);
        for connection in connections.split(", ") {
            valve.tunnels.push(connection);
        }
        (name, valve)
    }).collect()
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
    fn test_part_one() {
        let res = part_one(INPUT);
        assert_eq!(1651, res);
    }
}
