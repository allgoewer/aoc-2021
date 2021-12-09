//! Day 9: Smoke Basin
use aoc21::{util::*, Quizzer};
use std::collections::HashSet;

pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        let values = collect(input);
        risk_low_points(&values).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let values = collect(input);
        three_largest_basins(&values).to_string()
    }
}

/// Collect the input into a [`Grid<u8>`]
fn collect(input: &str) -> Grid<u8> {
    let mut width = 0;

    let bytes: Vec<_> = input
        .lines()
        .map(|l| {
            let bytes = l.trim().as_bytes();
            width = bytes.len();

            bytes.iter()
        })
        .flatten()
        .map(|b| *b - b'0')
        .collect();

    (bytes, width).try_into().expect("parsing failed")
}

/// Checks whether a given point is lower than its neighbours
fn is_lowpoint(values: &Grid<u8>, x: usize, y: usize) -> bool {
    let neighbours = [
        (x.wrapping_sub(1), y),
        (x.wrapping_add(1), y),
        (x, y.wrapping_sub(1)),
        (x, y.wrapping_add(1)),
    ];

    for pos in neighbours {
        if values.contains(pos) && values[pos] <= values[(x, y)] {
            return false;
        }
    }

    true
}

/// Generates an iterator over all lowpoints in the given [`Grid<u8>`]
fn lowpoint_positions(values: &Grid<u8>) -> impl Iterator<Item = (usize, usize)> + '_ {
    values
        .index_iter()
        .filter(|(x, y)| is_lowpoint(values, *x, *y))
}

/// Calculates the risk of all the lowpoints
fn risk_low_points(values: &Grid<u8>) -> u64 {
    lowpoint_positions(values)
        .map(|p| values[p] as u64 + 1)
        .sum()
}

/// Calculates the size of a basin centered around the lowpoint at (x, y)
fn basin_size(
    values: &Grid<u8>,
    (x, y): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> u64 {
    visited.insert((x, y));

    if values[(x, y)] == 9 {
        return 0;
    }

    let neighbours = [
        (x.wrapping_sub(1), y),
        (x.wrapping_add(1), y),
        (x, y.wrapping_sub(1)),
        (x, y.wrapping_add(1)),
    ];

    let size: u64 = neighbours
        .iter()
        .filter_map(|p| {
            if !visited.contains(p) && values.contains(*p) {
                Some(basin_size(values, *p, visited))
            } else {
                None
            }
        })
        .sum();

    size + 1
}

/// Finds the three largest basins and calculates the product of their sizes
fn three_largest_basins(values: &Grid<u8>) -> u64 {
    let mut visited = HashSet::new();
    let mut basins: Vec<_> = lowpoint_positions(values)
        .map(|p| basin_size(values, p, &mut visited))
        .collect();

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678";

    #[test]
    fn part1_examples() {
        let values = collect(EXAMPLE);
        assert_eq!(risk_low_points(&values), 15);
    }

    #[test]
    fn part2_examples() {
        let values = collect(EXAMPLE);
        assert_eq!(three_largest_basins(&values), 1134);
    }
}
