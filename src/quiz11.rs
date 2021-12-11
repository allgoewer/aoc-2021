//! Day 11: Dumbo Octopus
use aoc21::{util::*, Quizzer};

/// Todays quiz implementation
pub struct Quiz;

/// A field of octopuses
type Octopuses = Grid<(u8, bool)>;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        steps(&mut parse(input), 100).to_string()
    }

    fn part2(&self, input: &str) -> String {
        first_synchronized_flash(&mut parse(input)).to_string()
    }
}

/// Parse the puzzle input
fn parse(input: &str) -> Octopuses {
    let mut width = 0;
    let values: Vec<_> = input
        .lines()
        .map(|l| {
            let bytes = l.trim().as_bytes();
            width = bytes.len();
            bytes.iter()
        })
        .flatten()
        .map(|v| (v - b'0', false))
        .collect();

    (values, width).try_into().expect("creating grid failed")
}

/// Increase the energy of all octopuses by 1
fn increase(octopuses: &mut Octopuses) {
    for (energy, flashed) in octopuses.iter_mut() {
        *flashed = false;
        *energy += 1;
    }
}

/// Flash all the octopuses
fn flash(octopuses: &mut Octopuses) -> u64 {
    let mut flashes = 0;

    for (x, y) in octopuses.index_iter() {
        let neighbours = [
            (x.wrapping_add(1), y.wrapping_add(1)),
            (x.wrapping_add(1), y),
            (x.wrapping_add(1), y.wrapping_sub(1)),
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
            (x.wrapping_sub(1), y.wrapping_add(1)),
            (x.wrapping_sub(1), y),
            (x.wrapping_sub(1), y.wrapping_sub(1)),
        ];

        if let (10.., false) = octopuses[(x, y)] {
            octopuses[(x, y)].1 = true;
            flashes += 1;

            for neigh in neighbours {
                if octopuses.contains(neigh) {
                    octopuses[neigh].0 += 1;
                }
            }
        }
    }

    flashes
}

/// Reset all previously flashing octopuses to 0 energy
fn reset(octopuses: &mut Octopuses) {
    for (energy, flashed) in octopuses.iter_mut() {
        if *flashed {
            *energy = 0;
        }
    }
}

/// Step the octopuses by one day
fn light_step(octopuses: &mut Octopuses) -> u64 {
    increase(octopuses);

    let mut sum_flashes = 0;
    loop {
        let flashes = flash(octopuses);
        sum_flashes += flashes;

        if flashes == 0 {
            break;
        }
    }

    reset(octopuses);
    sum_flashes
}

/// Step the octopuses by n_days
fn steps(octopuses: &mut Octopuses, n_days: usize) -> u64 {
    (1..=n_days).map(|_| light_step(octopuses)).sum()
}

/// Find the first day where ALL octopuses flash
fn first_synchronized_flash(octopuses: &mut Octopuses) -> usize {
    for day in 1.. {
        if light_step(octopuses) as usize == octopuses.len() {
            return day;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[&str] = &[
        "\
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526",
        "\
        6594254334
        3856965822
        6375667284
        7252447257
        7468496589
        5278635756
        3287952832
        7993992245
        5957959665
        6394862637",
        "\
        8807476555
        5089087054
        8597889608
        8485769600
        8700908800
        6600088989
        6800005943
        0000007456
        9000000876
        8700006848",
    ];

    #[test]
    fn part1_examples() {
        let mut octopuses = parse(EXAMPLE[0]);

        let flashes = light_step(&mut octopuses);
        assert_eq!(
            (
                octopuses
                    .iter()
                    .map(|(energy, _)| energy)
                    .collect::<Vec<_>>(),
                flashes
            ),
            (
                parse(EXAMPLE[1]).iter().map(|(energy, _)| energy).collect(),
                0
            )
        );

        let flashes = light_step(&mut octopuses);
        assert_eq!(
            (
                octopuses
                    .iter()
                    .map(|(energy, _)| energy)
                    .collect::<Vec<_>>(),
                flashes
            ),
            (
                parse(EXAMPLE[2]).iter().map(|(energy, _)| energy).collect(),
                35
            )
        );
    }

    #[test]
    fn part2_examples() {
        assert_eq!(first_synchronized_flash(&mut parse(EXAMPLE[0])), 195);
    }
}
