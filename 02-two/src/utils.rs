use crate::types::OpponentLetter;
use crate::types::OpponentLetter::*;
use crate::types::Play;
use crate::types::PlayerLetter::*;
use crate::types::Play::*;
use crate::types::Outcome::*;
use crate::types::{Moves, MoveForOutcome, Strategy};

pub fn parse_input(input: &str) -> Strategy {
    input
        .lines()
        .map(|line| {
            let mut line = line.chars();
            let opponent = line.next().unwrap();
            let player = line.nth(1).unwrap();
            let opponent_shape = match opponent {
                'A' => A,
                'B' => B,
                'C' => C,
                _ => unreachable!(),
            };

            let player_shape = match player {
                'X' => X,
                'Y' => Y,
                'Z' => Z,
                _ => unreachable!(),
            };
            (opponent_shape, player_shape)
        })
        .collect()
}

pub fn convert_letters_to_moves(strategy: Strategy) -> Moves {
    strategy
        .into_iter()
        .map(|(opponent_play, player_play)| {
            let opponent_shape = convert_opponent_letter(opponent_play);
            let player_shape = match player_play {
                X => Rock,
                Y => Paper,
                Z => Scissors,
            };
            (opponent_shape, player_shape)
        })
        .collect()
}

pub fn convert_letters_to_move_for_outcome(strategy: Strategy) -> MoveForOutcome {
    strategy
        .into_iter()
        .map(|(opponent_play, outcome_code)| {
            let opponent_shape = convert_opponent_letter(opponent_play);
            let outcome = match outcome_code {
                X => Loose,
                Y => Draw,
                Z => Win,
            };
            (opponent_shape, outcome)
        })
        .collect()
}

fn convert_opponent_letter(letter: OpponentLetter) -> Play {
    match letter {
        A => Rock,
        B => Paper,
        C => Scissors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::vec_eq;

    #[test]
    fn parse_input_test() {
        const INPUT: &str = "A Y
B X
C Z";
        let expected = vec![(A, Y), (B, X), (C, Z)];

        let result = parse_input(INPUT);
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_letter_to_shape_test() {
        let strategy = vec![(A, Y), (B, X), (C, Z)];
        let expected_result = vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)];
        let result = convert_letters_to_moves(strategy);
        assert!(vec_eq(result, expected_result));
    }

    #[test]
    fn test_convert_letter_to_shape_second_strategy() {
        let strategy = vec![(A, Y), (B, X), (C, Z)];
        let expected_result = vec![(Rock, Draw), (Paper, Loose), (Scissors, Win)];
        let result = convert_letters_to_move_for_outcome(strategy);
        assert!(vec_eq(result, expected_result));
    }
}
