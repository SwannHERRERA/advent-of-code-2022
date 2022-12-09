use std::collections::HashSet;

use num::clamp;

use crate::{
    types::{Direction, Insctructions, Position},
    utils::is_not_clamped,
};

pub struct Rope {
    size: usize,
    position_tail_discovered: HashSet<Position>,
}

impl Rope {
    pub fn new(size: usize) -> Self {
        let mut rope = Rope {
            size,
            position_tail_discovered: HashSet::new(),
        };
        let inital_position = Position::default();
        rope.position_tail_discovered.insert(inital_position);
        rope
    }

    pub fn process(&mut self, instructions: &Insctructions) {
        let mut segments = vec![Position::default(); self.size];

        for (direction, quantity) in instructions {
            for _ in 0..*quantity {
                Self::apply_move(
                    *direction,
                    &mut segments,
                    &mut self.position_tail_discovered,
                );
            }
        }
    }

    pub fn cell_discovered_by_the_tail(&self) -> usize {
        self.position_tail_discovered.len()
    }

    fn apply_move(
        direction: Direction,
        segments: &mut Vec<Position>,
        position_discovery: &mut HashSet<Position>,
    ) {
        Self::move_head(direction, &mut segments[0]);
        let length = segments.len();
        for idx in 1..segments.len() {
            let predecesor = segments[idx - 1];
            let element = &mut segments[idx];
            if let Some(new_root_position) =
                Self::compute_future_position_of_the_tail(predecesor, *element)
            {
                *element = new_root_position;
                if idx == length - 1 {
                    position_discovery.insert(*element);
                }
            }
        }
    }

    fn move_head(direction: Direction, head: &mut Position) {
        match direction {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Left => head.x -= 1,
            Direction::Down => head.y -= 1,
        }
    }

    fn compute_future_position_of_the_tail(head: Position, tail: Position) -> Option<Position> {
        if !Self::is_tail_far_away(tail, head) {
            return None;
        }
        let diff_y = clamp(head.y - tail.y, -1, 1);
        let diff_x = clamp(head.x - tail.x, -1, 1);

        let position = Position {
            y: tail.y + diff_y,
            x: tail.x + diff_x,
        };
        Some(position)
    }

    fn is_tail_far_away(tail: Position, head: Position) -> bool {
        let diff_y = head.y - tail.y;
        let diff_x = head.x - tail.x;
        is_not_clamped(diff_y, -1, 1) || is_not_clamped(diff_x, -1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tail_far_away() {
        let head = Position::default();
        let tail = Position::default();
        assert!(!Rope::is_tail_far_away(tail, head));

        let head = Position::new(1, 0);
        let tail = Position::default();
        assert!(!Rope::is_tail_far_away(tail, head));

        let head = Position::new(1, 0);
        let tail = Position::new(0, 1);
        assert!(!Rope::is_tail_far_away(tail, head));

        let head = Position::new(2, 0);
        let tail = Position::new(0, 1);
        assert!(Rope::is_tail_far_away(tail, head));

        let head = Position::default();
        let tail = Position::new(0, 2);
        assert!(Rope::is_tail_far_away(tail, head));
    }
}
