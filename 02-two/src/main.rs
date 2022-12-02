use std::fs;

mod constant;
mod types;
mod utils;

use constant::*;
use types::{OutcomeOfRound, Shape, Strategy};

use utils::{convert_letter_to_shape, convert_letter_to_shape_second_strategy};
use Shape::*;

use crate::utils::parse_input;

fn main() {
    star1();
    star2();
}

fn star1() {
    let input = fs::read_to_string("02-two/data.txt").unwrap();
    let strategy = parse_input(input.as_str());
    let result = compute_total_score(strategy);
    println!("{result}");
}

fn star2() {
    let input = fs::read_to_string("02-two/data.txt").unwrap();
    let strategy = parse_input(input.as_str());
    let result = compute_second_strategy(strategy);
    println!("{result}");
}

fn compute_total_score(strategy: Strategy) -> u32 {
    let games = convert_letter_to_shape(strategy);

    let win_lose_score: u32 = games.iter().map(play_round).sum();
    let shape_usage_score: u32 = games
        .iter()
        .map(|(_, player_shape)| *player_shape as u32)
        .sum();
    win_lose_score + shape_usage_score
}

fn compute_second_strategy(strategy: Strategy) -> u32 {
    let games = convert_letter_to_shape_second_strategy(strategy);

    let win_lose_score: u32 = games
        .iter()
        .map(|(_, game_resolution)| game_resolution)
        .sum();
    let shape_usage_score: u32 = games
        .iter()
        .map(find_best_move)
        .map(|shape_played| shape_played as u32)
        .sum();
    win_lose_score + shape_usage_score
}

fn play_round((opponent_shape, player_shape): &(Shape, Shape)) -> OutcomeOfRound {
    match (player_shape, opponent_shape) {
        (ROCK, SCISSORS) | (PAPER, ROCK) | (SCISSORS, PAPER) => WIN,
        (ROCK, ROCK) | (PAPER, PAPER) | (SCISSORS, SCISSORS) => DRAW,
        (_, _) => LOOSE,
    }
}

fn find_best_move((opponent_shape, result_expected): &(Shape, u32)) -> Shape {
    match (opponent_shape, *result_expected) {
        (ROCK, DRAW) | (PAPER, LOOSE) | (SCISSORS, WIN) => ROCK,
        (ROCK, WIN) | (PAPER, DRAW) | (SCISSORS, LOOSE) => PAPER,
        (ROCK, LOOSE) | (PAPER, WIN) | (SCISSORS, DRAW) => SCISSORS,
        (_, _) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpponentShape::*;
    use crate::types::PlayerShape::*;

    #[test]
    fn compute_total_score_test() {
        const EXPECTED_SCORE: u32 = 15;

        let strategy: Strategy = vec![(A, Y), (B, X), (C, Z)];
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

    #[test]
    fn use_second_strategy() {
        const EXPECTED_SCORE: u32 = 12;
        const INPUT: &str = "A Y
B X
C Z";

        let strategy = parse_input(INPUT);
        let result = compute_second_strategy(strategy);
        assert_eq!(result, EXPECTED_SCORE);
    }
}
