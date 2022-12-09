#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'L' => Direction::Left,
            'D' => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    #[allow(unused)]
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

pub type Quantity = usize;
pub type Instruction = (Direction, Quantity);
pub type Insctructions = Vec<Instruction>;
