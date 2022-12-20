use std::{collections::{HashMap, VecDeque}, str::FromStr};

#[derive(Debug, Clone)]
pub struct Valve {
    pub name: String,
    pub flow_rate: isize,
    pub tunnels: Vec<String>,
}

impl FromStr for Valves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut valves = HashMap::new();
        for line in s.replace("valves","valve").lines() {
            let mut parts = line.split(" has flow rate=");
            let name = parts.next().unwrap().split(' ').last().unwrap().to_string();
            let mut parts = parts.next().unwrap().split(';');
            let flow_rate = parts.next().unwrap().parse().unwrap();
            let parts = parts.next().unwrap().split("valve ");
            let tunnels = parts.last().unwrap().split(", ").map(|s| s.to_string()).collect();
            valves.insert(name.clone(), Valve { name, flow_rate, tunnels });
        }
        Ok(Valves { valves })
    }
}

#[derive(Debug, Clone)]
pub struct Valves {
    pub valves: HashMap<String, Valve>,
}

impl Valves {
    pub fn find_distance(&self, start: String, end: String) -> isize {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back((start.clone(), 0));
        visited.insert(start, true);
        while let Some((valve_name, depth)) = queue.pop_front() {
            if *valve_name == end {
                return depth;
            }
            visited.insert(valve_name.clone(), true);
            for tunnel in self.valves.get(&valve_name).unwrap().tunnels.iter() {
                if !visited.contains_key(tunnel) {
                    queue.push_back((tunnel.clone(), depth+1));
                }
            }
        }
        unreachable!()
    }
}
