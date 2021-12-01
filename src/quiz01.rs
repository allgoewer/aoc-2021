//! Day 1: Sonar Sweep
use aoc21::{util::*, Quizzer};

pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        count_increases(&collected(input), 2).to_string()
    }

    fn part2(&self, input: &str) -> String {
        count_increases(&collected(input), 4).to_string()
    }
}

fn count_increases(values: &[i32], len: usize) -> usize {
    // w[1] + .. + w[len - 2] cancels out on both sides of the equation
    values.windows(len).filter(|w| w.first() < w.last()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_examples() {
        assert_eq!(count_increases(EXAMPLE, 2), 7);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(count_increases(EXAMPLE, 4), 5);
    }
}
