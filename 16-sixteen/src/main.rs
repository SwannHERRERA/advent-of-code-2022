use std::collections::{HashMap, HashSet};

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
    flow_rate: usize,
    name: String,
}

struct NodeInRec {
    id: usize,
    time_to_be_reach: usize,
    release_potentiel: usize,
}

fn main() {

}

fn pondere_graph(adjacency: &mut Vec<Vec<Option<usize>>>, nodes: &Vec<Node>, start_point: usize, state: &VolcanoState) {
    for (i, x) in adjacency[start_point].iter().enumerate().filter(|(i, x)| x.is_some()) {
        
    }
}

fn pondere_adjacency(adjacency: &mut Vec<Vec<Option<usize>>>, nodes: &Vec<Node>, visited: Vec<usize>, start: usize, state: &VolcanoState,) {
    let count = adjacency[start].iter().enumerate().filter(|(i, x)| x.is_some() && !visited.contains(i)).count();
    // 
}


fn parse_line(line: &str) -> (&str, usize, &str) {
    let (_, x) = line.split_once(' ').unwrap();
    let (name, x) = x.split_once(' ').unwrap();
    let (_, x) = x.split_once('=').unwrap();
    let (flow_rate, x) = x.split_once(';').unwrap();
    let mut option = x.split_once("valves ");
    if option.is_none() {
        option = x.split_once("valve ");
    }
    let (_, connections) = option.unwrap();
    let flow_rate: usize = flow_rate.parse().unwrap();
    (name, flow_rate, connections)
}

fn create_nodes(input: &str) -> Vec<Node> {
    input.lines().enumerate().map(|(id, line)| {
        let (name, flow_rate, _connections) = parse_line(line);
        Node { name: name.to_string(), id, flow_rate }
    }).collect()
}

fn create_matrix(input: &str, nodes: &Vec<Node>) -> Vec<Vec<Option<usize>>> {
    let mut vec: Vec<Vec<Option<usize>>> = Vec::with_capacity(nodes.len());
    for _ in 0..nodes.len() {
        vec.push(vec![None; nodes.len()]);
    }
    input.lines().enumerate().for_each(|(id, line)| {
        let (_, _, connections) = parse_line(line);
        for connection in connections.split(", ") {
            let connection: usize = nodes.iter().find(|node| node.name == connection).unwrap().id;
            vec[id][connection] = Some(0);
        }
    });
    vec
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
    fn test_parsing_into_adjacence_matrix() {
        let expected = vec![
            vec![None, Some(0), None, Some(0), None, None, None, None, Some(0), None],
            vec![Some(0), None, Some(0), None, None, None, None, None, None, None],
            vec![None, Some(0), None, Some(0), None, None, None, None, None, None],
            vec![Some(0), None, Some(0), None, Some(0), None, None, None, None, None],
            vec![None, None, None, Some(0), None, Some(0), None, None, None, None],
            vec![None, None, None, None, Some(0), None, Some(0), None, None, None],
            vec![None, None, None, None, None, Some(0), None, Some(0), None, None],
            vec![None, None, None, None, None, None, Some(0), None, None, None],
            vec![Some(0), None, None, None, None, None, None, None, None, Some(0)],
            vec![None, None, None, None, None, None, None, None, Some(0), None],
        ];
        let nodes = create_nodes(INPUT);
        let res = create_matrix(INPUT, &nodes);
        assert_eq!(expected, res);
    }

}
