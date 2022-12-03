use std::fs;

mod types;
mod utils;

use types::{
    Outcome::{self, *},
    Play, Strategy,
};

use utils::{convert_letters_to_move_for_outcome, convert_letters_to_moves};
use Play::*;

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
    let games = convert_letters_to_moves(strategy);

    let win_lose_score: u32 = games.iter().map(play_round).map(|res| res as u32).sum();
    let shape_usage_score: u32 = games
        .iter()
        .map(|(_, player_shape)| *player_shape as u32)
        .sum();
    win_lose_score + shape_usage_score
}

fn compute_second_strategy(strategy: Strategy) -> u32 {
    let games = convert_letters_to_move_for_outcome(strategy);

    let win_lose_score: u32 = games
        .iter()
        .map(|(_, game_resolution)| *game_resolution as u32)
        .sum();
    let shape_usage_score: u32 = games
        .iter()
        .map(find_best_move)
        .map(|shape_played| shape_played as u32)
        .sum();
    win_lose_score + shape_usage_score
}

fn play_round((opponent_shape, player_shape): &(Play, Play)) -> Outcome {
    match (player_shape, opponent_shape) {
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Loose,
    }
}

fn find_best_move((opponent_shape, result_expected): &(Play, Outcome)) -> Play {
    match (opponent_shape, *result_expected) {
        (Rock, Draw) | (Paper, Loose) | (Scissors, Win) => Rock,
        (Rock, Win) | (Paper, Draw) | (Scissors, Loose) => Paper,
        (Rock, Loose) | (Paper, Win) | (Scissors, Draw) => Scissors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpponentLetter::*;
    use crate::types::PlayerLetter::*;
    const INPUT: &str = "A Y
B X
C Z";

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

        let strategy = parse_input(INPUT);
        let total_score = compute_total_score(strategy);
        assert_eq!(total_score, EXPECTED_SCORE);
    }

    #[test]
    fn use_second_strategy() {
        const EXPECTED_SCORE: u32 = 12;

        let strategy = parse_input(INPUT);
        let result = compute_second_strategy(strategy);
        assert_eq!(result, EXPECTED_SCORE);
    }
}
