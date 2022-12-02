pub type OutcomeOfRound = u32;

#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerShape {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpponentShape {
    A,
    B,
    C,
}

pub type Strategy = Vec<(OpponentShape, PlayerShape)>;
pub type Games = Vec<(Shape, Shape)>;
pub type SecondStrategy = Vec<(Shape, OutcomeOfRound)>;
