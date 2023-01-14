use std::ops::Add;

pub trait Map<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_y_x(&self, y: usize, x: usize) -> Option<&T>;
}

impl<T: Copy> Map<T> for Vec<Vec<T>> {
    fn get_y_x(&self, y: usize, x: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }

    fn width(&self) -> usize {
        self.get(0).map_or(0, |x| x.len())
    }

    fn height(&self) -> usize {
        self.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<Pos> for Direction {
    fn into(self) -> Pos {
        match self {
            Direction::Right => Pos(1, 0),
            Direction::Down => Pos(0, 1),
            Direction::Left => Pos(-1, 0),
            Direction::Up => Pos(0, -1),
        }
    }
}

pub type Valley = Vec<Vec<Tile>>;
pub type Blizzard = Vec<Vec<Option<Direction>>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}
