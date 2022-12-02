use crate::constant::*;
use crate::types::{Strategy, Games, SecondStrategy};
use crate::types::Shape::*;
use crate::types::PlayerShape::*;
use crate::types::OpponentShape::*;

pub fn parse_input(input: &str) -> Strategy {
    input
        .lines()
        .map(|line| {
            let mut line = line.chars();
            let opponent = line.nth(0).unwrap();
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

pub fn convert_letter_to_shape(strategy: Strategy) -> Games {
    strategy.into_iter().map(|(opponent_play, player_play)| {
        let opponent_shape = match opponent_play {
            A => ROCK,
            B => PAPER,
            C => SCISSORS,
        };
        let player_shape = match player_play {
            X => ROCK,
            Y => PAPER,
            Z => SCISSORS,
        };
        return (opponent_shape, player_shape);
    }).collect()
}

pub fn convert_letter_to_shape_second_strategy(strategy: Strategy) -> SecondStrategy {
    strategy.into_iter().map(|(opponent_play, outcome_code)| {
        let opponent_shape = match opponent_play {
            A => ROCK,
            B => PAPER,
            C => SCISSORS,
        };
        let outcome = match outcome_code {
            X => LOST,
            Y => DRAW,
            Z => WIN,
        };
        return (opponent_shape, outcome);
    }).collect()
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
        let expected = vec![
            (A, Y),
            (B, X),
            (C, Z),
        ];

        let result = parse_input(INPUT);
        assert_eq!(result, expected);
    }

    #[test]
    fn convert_letter_to_shape_test() {
        let strategy = vec![
            (A, Y),
            (B, X),
            (C, Z),
        ];
        let expected_result = vec![
            (ROCK, PAPER),
            (PAPER, ROCK),
            (SCISSORS, SCISSORS),
        ];
        let result = convert_letter_to_shape(strategy);
        assert!(vec_eq(result, expected_result));
    }

    #[test]
    fn test_convert_letter_to_shape_second_strategy() {
        let strategy = vec![
            (A, Y),
            (B, X),
            (C, Z),
        ];
        let expected_result = vec![
            (ROCK, DRAW),
            (PAPER, LOST),
            (SCISSORS, WIN),
        ];
        let result = convert_letter_to_shape_second_strategy(strategy);
        assert!(vec_eq(result, expected_result));
        
    }
}

