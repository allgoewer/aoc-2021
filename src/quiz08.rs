//! Day 8: Seven Segment Search
use aoc21::{util::*, Quizzer};
use std::collections::{BTreeSet, HashMap};
use std::str::SplitAsciiWhitespace;

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        count_unique_digits(parsed_with(input, parse)).to_string()
    }

    fn part2(&self, input: &str) -> String {
        sum_output_values(parsed_with(input, parse)).to_string()
    }
}

/// Alias around iterators for each line of the input
type Io<'input> = (SplitAsciiWhitespace<'input>, SplitAsciiWhitespace<'input>);

/// Alias for a [`HashMap`] of input lines and their respective 7-segment value
type Signals<'input> = HashMap<BTreeSet<u8>, u8>;

/// Parse the puzzle input
fn parse(line: &str) -> Io<'_> {
    let (patterns, outputs) = line.split_once('|').expect("parsing failed");

    (
        patterns.split_ascii_whitespace(),
        outputs.split_ascii_whitespace(),
    )
}

/// Count the number of digits in the outputs which have a unique number of signal lines
fn count_unique_digits<'input>(ios: impl Iterator<Item = Io<'input>>) -> usize {
    ios.map(|(_, o)| {
        o.filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
            .count()
    })
    .sum()
}

/// Descramble signal lines, decode the outputs and calculate the sum
fn sum_output_values<'input>(ios: impl Iterator<Item = Io<'input>>) -> usize {
    ios.map(|(p, o)| decode_output(o, decode_patterns(p))).sum()
}

/// Generates a [`BTreeSet`] from a &[`str`]
///
/// It is assumed that the input is ASCII.
fn to_set(input: &str) -> BTreeSet<u8> {
    input.as_bytes().iter().copied().collect()
}

/// Tries to decode a scrambled input line pattern with the help of the already decoded signal lines
fn match_pattern(
    pat: &BTreeSet<u8>,
    decided: &HashMap<u8, BTreeSet<u8>>,
) -> Option<(u8, BTreeSet<u8>)> {
    match pat.len() {
        // can be 2, 3 or 5
        5 => {
            if let Some(one) = decided.get(&1) {
                if pat.is_superset(one) {
                    return Some((3, pat.clone()));
                } else if let Some(four) = decided.get(&4) {
                    let unique = four - one;
                    if pat.is_superset(&unique) {
                        return Some((5, pat.clone()));
                    } else {
                        return Some((2, pat.clone()));
                    }
                }
            }
        }
        // can be 0, 6 or 9
        6 => {
            if let Some(four) = decided.get(&4) {
                if pat.is_superset(four) {
                    return Some((9, pat.clone()));
                } else if let Some(one) = decided.get(&1) {
                    if pat.is_superset(one) {
                        return Some((0, pat.clone()));
                    } else {
                        return Some((6, pat.clone()));
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    None
}

/// Tries to decode all scrambled input signals
fn decode_patterns<'input>(patterns: impl Iterator<Item = &'input str>) -> Signals<'input> {
    let mut decided = HashMap::new();
    let mut undecided: Vec<_> = patterns.map(to_set).collect();

    while !undecided.is_empty() {
        undecided.retain(|v| match v.len() {
            n @ (2 | 3 | 4 | 7) => {
                decided
                    .entry(CANDIDATES_BY_NUM[n][0])
                    .or_insert_with(|| v.clone());
                false
            }
            5 | 6 => {
                if let Some((val, pat)) = match_pattern(v, &decided) {
                    decided.entry(val).or_insert(pat);
                    false
                } else {
                    true
                }
            }
            _ => unreachable!(),
        });
    }

    decided.into_iter().map(|(k, v)| (v, k)).collect()
}

/// Decode the output signals with the decoded input signals
fn decode_output<'input>(
    outputs: impl Iterator<Item = &'input str>,
    signals: Signals<'input>,
) -> usize {
    outputs
        .zip([1000, 100, 10, 1])
        .map(|(out, mul)| signals[&to_set(out)] as usize * mul)
        .sum()
}

/// List of candidates per number of active signal lines
///
/// ```txt
/// 0: none
/// 1: none
/// 2: 1
/// 3: 7
/// 4: 4
/// 5: 2, 3, 5
/// 6: 0, 6, 9
/// 7: 8
/// ```
const CANDIDATES_BY_NUM: [&[u8]; 8] = [&[], &[], &[1], &[7], &[4], &[2, 3, 5], &[0, 6, 9], &[8]];

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1_examples() {
        assert_eq!(count_unique_digits(parsed_with(EXAMPLE, parse)), 26);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(sum_output_values(parsed_with(EXAMPLE, parse)), 61229);
    }
}
