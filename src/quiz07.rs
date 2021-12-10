//! Day 7: The Treachery of Whales
use aoc21::Quizzer;

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        calc_simplified_fuel_consumption(collect(input)).to_string()
    }

    fn part2(&self, input: &str) -> String {
        calc_fuel_consumption(collect(input)).to_string()
    }
}

/// Collect the quiz input
fn collect(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|v| v.parse().expect("parsing failed"))
        .collect()
}

/// Calculate the optimal fuel consumption according to the simplified formula (part 1)
fn calc_simplified_fuel_consumption(mut crabs: Vec<i64>) -> i64 {
    crabs.sort_unstable();

    let median_pos = crabs[crabs.len() / 2];
    crabs.into_iter().map(|pos| (median_pos - pos).abs()).sum()
}

/// Calculate the amount of fuel necessary to move all crabs to pos with the extended formula (part 2)
fn calc_fuel_for_pos(crabs: &[i64], pos: i64) -> i64 {
    crabs
        .iter()
        .map(|current_pos| {
            let dist = (pos - current_pos).abs();
            // little gauss (sum of 1 + 2 + 3 .. + dist)
            (dist * dist + dist) / 2
        })
        .sum()
}

/// Calculate the optimal fuel consumption according to the extended formula (part 2)
fn calc_fuel_consumption(mut crabs: Vec<i64>) -> i64 {
    crabs.sort_unstable();
    let min = crabs[0];
    let max = crabs[crabs.len() - 1];

    // brute-force the solution :(
    (min..=max)
        .map(|pos| calc_fuel_for_pos(&crabs, pos))
        .min()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_examples() {
        assert_eq!(calc_simplified_fuel_consumption(collect(EXAMPLE)), 37);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(calc_fuel_consumption(collect(EXAMPLE)), 168);
    }
}
