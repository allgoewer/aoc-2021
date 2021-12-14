//! Day 14: Extended Polymerization
use aoc21::Quizzer;
use std::collections::{BTreeSet, HashMap};

/// Todays quiz implementation
pub struct Quiz;

/// A counter for all pairs in a polymer
type Polymer = HashMap<(u8, u8), i64>;

/// A mapping of pairs to the respective inserted value
type Mapping = HashMap<(u8, u8), u8>;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        let (mut poly, map) = parse(input).unwrap();
        step_polymer(&mut poly, &map, 10).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (mut poly, map) = parse(input).unwrap();
        step_polymer(&mut poly, &map, 40).to_string()
    }
}

/// Parse the puzzle input
fn parse(input: &str) -> Result<(Polymer, Mapping), anyhow::Error> {
    let mut input = input.lines();

    let polymer = input
        .next()
        .ok_or_else(|| anyhow::anyhow!("no first line in input"))?;

    let mut poly_count = HashMap::new();
    for pair in polymer.as_bytes().windows(2) {
        *poly_count.entry((pair[0], pair[1])).or_default() += 1;
    }

    let mapping: Mapping = input
        .skip(1)
        .filter_map(|l| l.split_once("->"))
        .map(|(l, r)| {
            let l = l.trim().as_bytes();
            let r = r.trim().as_bytes();

            ((l[0], l[1]), r[0])
        })
        .collect();

    Ok((poly_count, mapping))
}

/// Expand the polymer once
fn expand_polymer(poly: &mut Polymer, map: &Mapping) {
    let mut additional: Polymer = HashMap::new();

    for (pair, count) in poly.iter() {
        if let Some(o) = map.get(pair) {
            *additional.entry(*pair).or_default() -= count;
            *additional.entry((pair.0, *o)).or_default() += count;
            *additional.entry((*o, pair.1)).or_default() += count;
        }
    }

    for (k, v) in additional {
        *poly.entry(k).or_default() += v;
    }
}

/// Expand the polymer n_steps times and return the difference in quantity between the most frequent and the least frequent character
fn step_polymer(poly: &mut Polymer, map: &Mapping, n_steps: usize) -> i64 {
    for _ in 0..n_steps {
        expand_polymer(poly, map);
    }

    let mut letter_count: HashMap<_, i64> = HashMap::new();
    for (k, &mut v) in poly {
        *letter_count.entry(k.0).or_default() += v;
        *letter_count.entry(k.1).or_default() += v;
    }

    let letter_count: BTreeSet<_> = letter_count.into_iter().map(|(_, v)| (v + 1) / 2).collect();

    let min = letter_count.iter().copied().next().unwrap_or_default();
    let max = letter_count.iter().copied().next_back().unwrap_or_default();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

    #[test]
    fn part1_examples() {
        let (mut polymer, map) = parse(EXAMPLE).unwrap();
        assert_eq!(step_polymer(&mut polymer, &map, 10), 1588);
    }

    #[test]
    fn part2_examples() {
        let (mut polymer, map) = parse(EXAMPLE).unwrap();
        assert_eq!(step_polymer(&mut polymer, &map, 40), 2188189693529);
    }
}
