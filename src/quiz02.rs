//! Day 2: Dive!
use aoc21::{util::*, Quizzer};
use std::str::FromStr;

pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        calc_depth_result(parsed(input)).to_string()
    }

    fn part2(&self, input: &str) -> String {
        calc_complicated_depth_result(parsed(input)).to_string()
    }
}

#[derive(Debug)]
struct Command(i64, i64);

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, delta) = s
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("can't split once '{}'", s))?;
        let delta = delta.parse()?;

        match (cmd, delta) {
            ("forward", x) => Ok(Self(x, 0)),
            ("down", y) => Ok(Self(0, y)),
            ("up", y) => Ok(Self(0, -y)),
            _ => Err(anyhow::anyhow!("unknown command '{}'", cmd)),
        }
    }
}

fn calc_depth_result(cmds: impl Iterator<Item = Command>) -> i64 {
    let position = cmds.fold((0, 0), |(x, y), cmd| (x + cmd.0, y + cmd.1));
    position.0 * position.1
}

fn calc_complicated_depth_result(cmds: impl Iterator<Item = Command>) -> i64 {
    let position = cmds.fold((0, 0, 0), |(aim, x, y), cmd| {
        (aim + cmd.1, x + cmd.0, y + aim * cmd.0)
    });
    position.1 * position.2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";

    #[test]
    fn part1_examples() {
        assert_eq!(calc_depth_result(parsed(EXAMPLE)), 150);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(calc_complicated_depth_result(parsed(EXAMPLE)), 900);
    }
}
