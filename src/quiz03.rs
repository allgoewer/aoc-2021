//! Day 3: Binary Diagnostic
use aoc21::{util::*, Quizzer};

pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        part1(&collect(input), 12).to_string()
    }

    fn part2(&self, input: &str) -> String {
        part2(collect(input), 12).to_string()
    }
}

fn collect(input: &str) -> Vec<u16> {
    collected_with(input, |v| u16::from_str_radix(v, 2)).expect("radix parsing failed")
}

/// Count the number of 1s for all bit positions in the input and returns the length of the iterator
fn count_ones<'a>(values: impl IntoIterator<Item = &'a u16>, counter: &mut [usize]) -> usize {
    counter.iter_mut().for_each(|v| *v = 0);

    values
        .into_iter()
        .copied()
        .inspect(|v| {
            for (i, one_count) in counter.iter_mut().enumerate() {
                if (v >> i) & 1 != 0 {
                    *one_count += 1;
                }
            }
        })
        .count()
}

fn part1(values: &[u16], bits: usize) -> usize {
    let mut one_counts = vec![0; bits];
    let n_entries = count_ones(values, &mut one_counts);

    let (gamma, epsilon) =
        one_counts
            .iter()
            .enumerate()
            .fold((0, 0), |(gamma, epsilon), (i, &one_count)| {
                if one_count > n_entries - one_count {
                    (gamma | 1 << i, epsilon)
                } else {
                    (gamma, epsilon | 1 << i)
                }
            });

    gamma * epsilon
}

fn part2(values: Vec<u16>, n_bits: usize) -> usize {
    let mut one_counts = vec![0; n_bits];
    let mut ogr_values = values.clone();
    let mut csr_values = values;

    for b in (0..n_bits).rev() {
        let bit = 1 << b;

        // update bit counts for the oxygen generator rating
        let n_entries = count_ones(&ogr_values, &mut one_counts);
        let ones = one_counts[b];
        let zeros = n_entries - ones;

        if ogr_values.len() > 1 {
            ogr_values.retain(|v| {
                if ones >= zeros {
                    v & bit != 0
                } else {
                    v & bit == 0
                }
            });
        }

        // update bit counts for the co2 scrubber rating
        let n_entries = count_ones(&csr_values, &mut one_counts);
        let ones = one_counts[b];
        let zeros = n_entries - ones;

        if csr_values.len() > 1 {
            csr_values.retain(|v| {
                if zeros <= ones {
                    v & bit == 0
                } else {
                    v & bit != 0
                }
            });
        }
    }

    ogr_values[0] as usize * csr_values[0] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&collect(EXAMPLE), 5), 198);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(collect(EXAMPLE), 5), 230);
    }
}
