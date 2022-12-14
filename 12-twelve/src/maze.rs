use std::collections::VecDeque;

pub struct Maze<'a> {
    pub line_length: usize,
    pub bytes: &'a [u8],
    pub queue: VecDeque<(usize, usize)>,
    pub backlinks: Vec<Option<usize>>,
}

impl<'a> Maze<'a> {
    pub fn new(bytes: &'a Vec<u8>, line_length: usize) -> Self {
        Maze {
            bytes: &bytes,
            queue: VecDeque::new(),
            backlinks: vec![None; bytes.len()],
            line_length,
        }
    }
   
    pub fn visit_neighbours(&mut self, pos: usize) {
        if pos % self.line_length > 0 {
            self.visit_neighbour(pos, pos - 1);
        }
        if pos % self.line_length < self.line_length - 1 {
            self.visit_neighbour(pos, pos + 1);
        }
        if pos > self.line_length {
            self.visit_neighbour(pos, pos - self.line_length);
        }
        if pos < self.bytes.len() - self.line_length {
            self.visit_neighbour(pos, pos + self.line_length);
        }
    }

    pub fn visit_neighbour(&mut self, from: usize, pos: usize) {
        if self.backlinks[pos].is_none() && self.bytes[pos] <= self.bytes[from] + 1 {
            self.queue.push_back((from, pos))
        }
    }

    pub fn find_path(&mut self, end_pos: usize) {
        while let Some((prev, pos)) = self.queue.pop_front() {
            if self.backlinks[pos].is_none() {
                self.backlinks[pos] = Some(prev);
                if pos == end_pos {
                    return;
                } else {
                    self.visit_neighbours(pos);
                }
            }
        }
    }

    pub fn count_steps(&mut self, target: usize) -> usize {
        let mut steps = 0;
        let mut pos = target;
        loop {
            let prev = self.backlinks[pos].unwrap();
            if prev == pos {
                return steps;
            }
            pos = prev;
            steps += 1;
        }
    }
}
