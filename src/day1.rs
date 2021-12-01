//! Day 1: Sonar Sweep
use aoc21::{util::*, Day};

pub struct Today;

impl Day for Today {
    fn part1(&self, input: &str) -> String {
        count_increases(&collected(input)).to_string()
    }

    fn part2(&self, input: &str) -> String {
        count_sliding_increases(&collected(input)).to_string()
    }
}

fn count_increases(values: &[i32]) -> usize {
    values.windows(2).filter(|w| w[0] < w[1]).count()
}

fn count_sliding_increases(values: &[i32]) -> usize {
    values
        .windows(4)
        .filter(|w| w[0] + w[1] + w[2] < w[1] + w[2] + w[3])
        .count()
}

#[test]
fn part1_examples() {
    assert_eq!(
        count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}

#[test]
fn part2_examples() {
    assert_eq!(
        count_sliding_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5
    );
}
