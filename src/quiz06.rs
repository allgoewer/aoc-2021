//! Day 1: Sonar Sweep
use aoc21::Quizzer;

pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        predict_fishies(&collect(input), 80).to_string()
    }

    fn part2(&self, input: &str) -> String {
        predict_fishies(&collect(input), 256).to_string()
    }
}

fn collect(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|v| v.parse().expect("parsing failed"))
        .collect()
}

fn predict_fishies(fishies: &[u8], n_days: usize) -> usize {
    let mut days = [0usize; 9];

    for fish in fishies {
        days[*fish as usize] += 1;
    }

    for i in 0..n_days {
        let today = i % days.len();
        days[(today + 7) % days.len()] += days[today];
    }

    days.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn part1_examples() {
        let fishies = collect(EXAMPLE);

        assert_eq!(predict_fishies(&fishies, 18), 26);
        assert_eq!(predict_fishies(&fishies, 80), 5934);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(predict_fishies(&collect(EXAMPLE), 256), 26984457539);
    }
}
