#[derive(Debug, PartialEq, Eq)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerLetter {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpponentLetter {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Outcome {
    Win = 6,
    Draw = 3,
    Loose = 0,
}

pub type Strategy = Vec<(OpponentLetter, PlayerLetter)>;
pub type Moves = Vec<(Play, Play)>;
pub type MoveForOutcome = Vec<(Play, Outcome)>;
