use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{valves::{Valves, Valve}, TIME_TO_TEACH_ELEPHANT, START_VALVE};

#[derive(Debug)]
pub struct Graph {
    adj_matrix : Array2<isize>,
    flow_rates : Array1<isize>,
}

impl Graph {
    pub fn new(valves: &Valves) -> Self {
        let mut non_0_valves: Vec<(&String, &Valve)> = valves.valves
            .iter()
            .filter(|(n,v)| *n == START_VALVE || v.flow_rate > 0)
            .collect();
        non_0_valves.sort_by_key(|(name,_)| *name);

        let mut adj_matrix = Array2::zeros((non_0_valves.len(), non_0_valves.len()));
        for i in 0..non_0_valves.len() {
            for j in 0..non_0_valves.len() {
                let name_start = non_0_valves[i].1.name.clone();
                let name_end = non_0_valves[j].1.name.clone();
                adj_matrix[[i,j]] = valves.find_distance(name_start, name_end);
            }
            
        }
        let flow_rates: Array1<isize> = non_0_valves.iter().map(|(_,v)| v.flow_rate).collect();
        Graph { adj_matrix, flow_rates }
    }

    pub fn find_best_path(&self, minutes: isize) -> isize {
        let visited = 0;
        self.best_flow_part(visited, minutes)
    }

    pub fn find_best_path_with_elephant(&self, minutes_allowed: isize) -> isize {
        (0..u16::MAX/2).into_par_iter()
            .step_by(2)
            .filter(|v| v.count_ones() == 7)
            .map(|visited| {
                self.best_flow_part(visited, minutes_allowed - TIME_TO_TEACH_ELEPHANT)
                + self.best_flow_part(!visited ^ 1, minutes_allowed - TIME_TO_TEACH_ELEPHANT)
            })
            .max()
            .unwrap()
    }

    fn best_flow_part(&self, visited: u16, minutes: isize) -> isize {
        const INITAL_STACK_CAPACITY: usize = 100;
        let mut best_flow = 0;
        let mut stack = Vec::with_capacity(INITAL_STACK_CAPACITY);
        stack.push((visited, 0, minutes, 0));
        while let Some((visited, node, time, flow)) = stack.pop() {
            for i in 0..self.adj_matrix.nrows() {
                if  visited & (1<<i) == 0 { 
                    let mut new_visited = visited;
                    new_visited |= 1<<i;
                    let new_time = time - 1 - self.adj_matrix[[node, i]];
                    if new_time > 0 {
                        stack.push((new_visited, i, new_time, flow + self.flow_rates[i]*new_time));
                    }
                    best_flow = best_flow.max(flow);
                }
            }
        }
        best_flow
    }

}

