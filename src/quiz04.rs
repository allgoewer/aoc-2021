//! Day 4: Giant Squid
use aoc21::{util::*, Quizzer};
use std::{convert::Infallible, str::FromStr};

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        let mut bingo: Bingo = input.parse().unwrap();
        bingo.play_winning_score().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut bingo: Bingo = input.parse().unwrap();
        bingo.play_last_winning_score().to_string()
    }
}

/// A type alias for a single bingo board
type Board = Grid<Option<u8>>;

/// A game of bingo
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Bingo {
    /// All drawn numbers in order of occurrence
    draws: Vec<u8>,
    /// All boards participating in the game
    boards: Vec<Board>,
}

/// Calculate the board score assuming the board is a winner
fn calc_score(draw: u8, board: &Board) -> u64 {
    draw as u64
        * board
            .iter()
            .map(|v| v.unwrap_or_default() as u64)
            .sum::<u64>()
}

/// Find and mark the drawn value on all boards
fn mark_draws(boards: &mut [Board], draw: u8) {
    for board in boards {
        for val in board.iter_mut() {
            if Some(draw) == *val {
                val.take();
            }
        }
    }
}

/// Get the boards which have a completely marked row or column
fn get_winners(boards: &mut Vec<Board>) -> Option<Vec<Board>> {
    let mut winners = Vec::new();

    while let Some((index, _)) = boards.iter().enumerate().find(|(_, b)| {
        let (width, height) = b.dim();

        for col in 0..width {
            let all_drawn = b.col_iter(col).all(|v| v.is_none());
            if all_drawn {
                return true;
            }
        }

        for row in 0..height {
            let all_drawn = b.row_iter(row).all(|v| v.is_none());
            if all_drawn {
                return true;
            }
        }

        false
    }) {
        winners.push(boards.remove(index));
    }

    if winners.is_empty() {
        None
    } else {
        Some(winners)
    }
}

impl Bingo {
    /// Play bingo and calculate the score according to part 1
    fn play_winning_score(&mut self) -> u64 {
        for draw in &self.draws {
            mark_draws(&mut self.boards, *draw);

            if let Some(winners) = get_winners(&mut self.boards) {
                return calc_score(*draw, &winners[0]);
            }
        }

        0
    }

    /// Play bingo and calculate the score according to part 2
    fn play_last_winning_score(&mut self) -> u64 {
        let mut winner_list = Vec::new();

        for draw in &self.draws {
            mark_draws(&mut self.boards, *draw);

            if let Some(winners) = get_winners(&mut self.boards) {
                winner_list.extend(std::iter::repeat(*draw).zip(winners.into_iter()));
            }
        }

        winner_list
            .last()
            .map(|(draw, last_winner)| calc_score(*draw, last_winner))
            .unwrap_or_default()
    }
}

impl FromStr for Bingo {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().map(str::trim);

        let draws: Vec<_> = lines
            .next()
            .expect("no draws in input")
            .trim()
            .split(',')
            .map(|v| v.parse().expect("parsing draw failed"))
            .collect();

        let mut boards = Vec::new();
        let mut next_board = Vec::new();

        for line in lines {
            let line = line.trim();

            next_board.extend(
                line.split_ascii_whitespace()
                    .map(|v| Some(v.parse::<u8>().expect("parsing board value failed"))),
            );

            if next_board.len() == 25 {
                boards.push((next_board.clone(), 5).try_into().expect("invalid board"));
                next_board.clear();
            }
        }

        Ok(Bingo { draws, boards })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7";

    #[test]
    fn part1_examples() {
        let mut bingo: Bingo = EXAMPLE.parse().unwrap();
        assert_eq!(bingo.play_winning_score(), 4512);
    }

    #[test]
    fn part2_examples() {
        let mut bingo: Bingo = EXAMPLE.parse().unwrap();
        assert_eq!(bingo.play_last_winning_score(), 1924);
    }
}
