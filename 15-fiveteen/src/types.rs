use std::{collections::BTreeMap, ops::RangeInclusive};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    pub fn manhattan_dist(&self, other: &Point) -> isize {
        self.x.abs_diff(other.x) as isize + self.y.abs_diff(other.y) as isize
    }
}

pub type MergedRange = (RangeInclusive<isize>, Option<isize>);
pub type IndexedRanges = BTreeMap<isize, Vec<RangeInclusive<isize>>>;
