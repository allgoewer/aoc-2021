//! The first day of the advent of code quiz.
//!
//! This is a dummy :)
use aoc21::Day;

pub struct Today;

impl Day for Today {
    fn part1(&self, input: &str) -> String {
        input.trim().to_owned()
    }

    fn part2(&self, input: &str) -> String {
        input.trim().to_owned()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {
        assert!(true);
    }
}
