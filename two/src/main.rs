use std::fs;

mod types;
mod utils;

use types::{Shape, Strategy, OutcomeOfRound};

use Shape::*;
use utils::convert_letter_to_shape;

use crate::utils::parse_input;


const LOST: OutcomeOfRound = 0;
const DRAW: OutcomeOfRound = 3;
const WIN: OutcomeOfRound = 6;

fn play_round((opponent_shape, player_shape): &(Shape, Shape)) -> OutcomeOfRound {
    match (player_shape, opponent_shape) {
        (ROCK, SCISSORS) | (PAPER, ROCK) | (SCISSORS, PAPER) => WIN,
        (ROCK, ROCK) | (PAPER, PAPER) | (SCISSORS, SCISSORS) => DRAW,
        (_, _) => LOST,
    }
}

fn compute_total_score(strategy: Strategy) -> u32 {
    let games = convert_letter_to_shape(strategy);
    
    let win_lose_score: u32 = games.iter().map(|game| play_round(game)).sum();
    let shape_usage_score: u32 = games.iter().map(|(_, player_shape)| *player_shape as u32).sum(); 
    return win_lose_score + shape_usage_score;
}

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let strategy = parse_input(input.as_str());
    let result = compute_total_score(strategy);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpponentShape::*;
    use crate::types::PlayerShape::*;

    #[test]
    fn compute_total_score_test() {
        const EXPECTED_SCORE: u32 = 15;

        let strategy: Strategy = vec![
            (A, Y),
            (B, X),
            (C, Z),
        ];
        let score = compute_total_score(strategy);
        assert_eq!(score, EXPECTED_SCORE);
    }

    #[test]
    fn resolve_example() {
        const EXPECTED_SCORE: u32 = 15;
        const INPUT: &str = "A Y
B X
C Z";

        let strategy = parse_input(INPUT);

        let total_score = compute_total_score(strategy);
        assert_eq!(total_score, EXPECTED_SCORE);
    }
}
