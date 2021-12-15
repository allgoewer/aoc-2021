//! Day 13: Transparent Origami
use aoc21::Quizzer;
use std::collections::BTreeSet;
use std::fmt::{self, Write};

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        let (mut paper, folds) = parse(input);
        paper.fold_and_count(&folds[..1]).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (mut paper, folds) = parse(input);
        paper.fold_and_count(&folds);
        paper.to_string()
    }
}

/// Where to fold the paper
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Folding {
    /// Fold along the x axis
    X(usize),
    /// Fold along the y axis
    Y(usize),
}

/// Parse the puzzle input
fn parse(input: &str) -> (Paper, Vec<Folding>) {
    let dots: Result<_, anyhow::Error> = input
        .lines()
        .map(str::trim)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l
                .split_once(',')
                .ok_or_else(|| anyhow::anyhow!("can't split dot location"))?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect();

    let folds: Result<Vec<_>, anyhow::Error> = input
        .lines()
        .rev()
        .map(str::trim)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let l = l
                .strip_prefix("fold along ")
                .ok_or_else(|| anyhow::anyhow!("can't find fold instruction"))?;
            let (direction, axis) = l
                .split_once('=')
                .ok_or_else(|| anyhow::anyhow!("can't split fold instruction"))?;

            Ok(if direction == "x" {
                Folding::X(axis.parse()?)
            } else {
                Folding::Y(axis.parse()?)
            })
        })
        .collect();

    let mut folds = folds.expect("parsing folding instructions failed");
    folds.reverse(); // We read the folding instructions in reverse order, so reverse them again

    (Paper(dots.expect("parsing dots failed")), folds)
}

/// The transparent paper
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Paper(BTreeSet<(usize, usize)>);

impl Paper {
    /// Fold the paper according to the provided folding instructions
    fn fold(&mut self, folds: &[Folding]) {
        let mut folded = BTreeSet::new();

        let do_fold = |pos, &line| {
            if pos > line {
                let pos = 2 * line - pos;
                if pos < line {
                    return Some(pos);
                }
            }
            None
        };

        for fold in folds {
            match fold {
                Folding::X(col) => self
                    .0
                    .retain(|&(x, y)| do_fold(x, col).map(|x| folded.insert((x, y))).is_none()),
                Folding::Y(row) => self
                    .0
                    .retain(|&(x, y)| do_fold(y, row).map(|y| folded.insert((x, y))).is_none()),
            }

            self.0.extend(&folded);
            folded.clear();
        }
    }

    /// Fold the paper according to the provided folding instructions and count the number of dots
    fn fold_and_count(&mut self, folds: &[Folding]) -> usize {
        self.fold(folds);
        self.0.len()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(&(width, height)) = self.0.iter().last() {
            f.write_str("\n\t")?;

            for y in 0..=height {
                for x in 0..=width {
                    if self.0.contains(&(x, y)) {
                        f.write_char('#')?;
                    } else {
                        f.write_char(' ')?;
                    }
                }

                f.write_str("\n\t")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5";

    #[test]
    fn part1_examples() {
        let (mut paper, folds) = parse(EXAMPLE);
        assert_eq!(paper.fold_and_count(&folds), 16);
    }

    #[test]
    fn part2_examples() {
        let (mut paper, folds) = parse(EXAMPLE);
        paper.fold_and_count(&folds);
        assert_eq!(
            paper.to_string(),
            "
\t#####
\t#   #
\t#   #
\t#   #
\t#####
\t"
        );
    }
}
