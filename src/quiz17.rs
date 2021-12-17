//! Day 17: Trick Shot
use std::{fmt::Display, str::FromStr};

use aoc21::Quizzer;

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        let area: BoundingBox = input.parse().unwrap();
        let launcher = ProbeLauncher::default();

        launcher
            .find_highest_hit(100, 1000, &area)
            .unwrap()
            .1
            .to_string()
    }
}

/// A position on a x/y grid
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Pos {
    /// The x coordinate
    x: i64,
    /// The y coordinate
    y: i64,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// A deep-sea probe
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Probe {
    /// The probes current trajectory
    trajectory: (i64, i64),
    /// The probes current position
    pos: Pos,
}

impl Probe {
    /// Whether a probe does hit the target area
    fn hit(&mut self, area: &BoundingBox) -> Option<(Pos, i64)> {
        let mut max_y = 0;

        for pos in self {
            max_y = std::cmp::max(max_y, pos.y);

            if area.contains(&pos) {
                return Some((pos, max_y));
            } else if area.overshot(&pos) {
                return None;
            }
        }

        None
    }
}

impl Iterator for Probe {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;

        // update the probes position
        self.pos.x += self.trajectory.0;
        self.pos.y += self.trajectory.1;

        // update the probes trajectory
        let tx = {
            let x = self.trajectory.0;
            match x {
                0 => 0,
                1.. => x - 1,
                _ => x + 1,
            }
        };

        let ty = self.trajectory.1 - 1;

        self.trajectory = (tx, ty);

        Some(pos)
    }
}

/// A launcher for a deep-sea probe
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct ProbeLauncher {
    /// The launchers location
    pos: Pos,
}

impl ProbeLauncher {
    /// Launches a probe into the given direction
    fn launch(&self, direction: (i64, i64)) -> Probe {
        Probe {
            trajectory: direction,
            pos: self.pos,
        }
    }

    /// Tries to find the direction which hits the target area and gives the highest launch
    fn find_highest_hit(
        &self,
        max_x: i64,
        max_y: i64,
        area: &BoundingBox,
    ) -> Option<(Pos, i64, usize)> {
        // To find the highest hit, we need to shoot to the right

        let mut highscore = None;
        let mut highpos = None;
        let mut hitcount = 0;

        for x in 0..=max_x {
            for y in -max_y..=max_y {
                if let Some((_, max)) = self.launch((x, y)).hit(area) {
                    hitcount += 1;

                    if Some(max) > highscore {
                        highscore = Some(max);
                        highpos = Some(Pos { x, y });
                    }
                }
            }
        }

        highpos.map(|pos| (pos, highscore.unwrap(), hitcount))
    }
}

/// A bounding box representing a probe target area
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BoundingBox {
    /// The range of x values
    x: (i64, i64),
    /// The range of y values
    y: (i64, i64),
}

impl BoundingBox {
    /// Whether a position lies within the BoundingBox
    fn contains(&self, pos: &Pos) -> bool {
        (self.x.0..=self.x.1).contains(&pos.x) && (self.y.0..=self.y.1).contains(&pos.y)
    }

    /// Whether the BoundingBox was overshot
    fn overshot(&self, pos: &Pos) -> bool {
        pos.x > self.x.1 || pos.y < self.y.1
    }
}

impl FromStr for BoundingBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parser::bounding_box(s) {
            Ok((_, bb)) => Ok(bb),
            Err(e) => Err(anyhow::anyhow!("parsing failed: {}", e)),
        }
    }
}

mod parser {
    //! A parser for this quizzes bounding box
    use super::BoundingBox;
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::digit1;
    use nom::combinator::{map, map_res, recognize};
    use nom::sequence::{pair, preceded, separated_pair};

    #[allow(clippy::missing_docs_in_private_items)]
    #[derive(Debug)]
    enum Axis {
        X(i64, i64),
        Y(i64, i64),
    }

    /// Type alias for noms IResult
    type IResult<'input, T> = nom::IResult<&'input str, T>;

    /// Parse an integer number
    fn number<'input>(input: &'input str) -> IResult<'input, i64> {
        let sign = |input: &'input str| alt((tag("+"), tag("-"), tag("")))(input);
        map_res(recognize(pair(sign, digit1)), |number| number.parse())(input)
    }

    /// Parse an axis
    fn axis(input: &str) -> IResult<Axis> {
        let label = |i| alt((tag("x"), tag("y")))(i);
        let range = |i| separated_pair(number, tag(".."), number)(i);
        map(
            separated_pair(label, tag("="), range),
            |(label, (start, end))| {
                if label == "x" {
                    Axis::X(start, end)
                } else {
                    Axis::Y(start, end)
                }
            },
        )(input)
    }

    /// Parse a bounding box
    pub(super) fn bounding_box(input: &str) -> IResult<BoundingBox> {
        let prefix = |i| {
            pair(
                tag("target area:"),
                take_while(|c: char| c.is_ascii_whitespace()),
            )(i)
        };
        let separator = |i| pair(tag(","), take_while(|c: char| c.is_ascii_whitespace()))(i);

        map_res(
            preceded(prefix, separated_pair(axis, separator, axis)),
            |axes| match &axes {
                (Axis::X(x0, x1), Axis::Y(y0, y1)) | (Axis::Y(y0, y1), Axis::X(x0, x1)) => {
                    Ok(BoundingBox {
                        x: (*x0, *x1),
                        y: (*y0, *y1),
                    })
                }
                _ => Err(anyhow::anyhow!("missing axis")),
            },
        )(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn number_ok() {
            assert!(matches!(number("1000"), Ok((_, 1000))));
            assert!(matches!(number("+1000"), Ok((_, 1000))));
            assert!(matches!(number("-1000"), Ok((_, -1000))));
        }

        #[test]
        fn axis_ok() {
            assert!(matches!(axis("x=10..20"), Ok((_, Axis::X(10, 20)))));
            assert!(matches!(axis("x=10..20"), Ok((_, Axis::X(10, 20)))));
            assert!(matches!(axis("y=10..20"), Ok((_, Axis::Y(10, 20)))));
            assert!(matches!(axis("y=-10..20"), Ok((_, Axis::Y(-10, 20)))));
        }

        #[test]
        fn bounding_box_ok() {
            assert!(matches!(
                dbg!(bounding_box("target area: x=20..30, y=-10..-5")),
                Ok((
                    _,
                    BoundingBox {
                        x: (20, 30),
                        y: (-10, -5),
                    }
                ))
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part1_examples() {
        let bb: BoundingBox = EXAMPLE.parse().unwrap();
        let launcher = ProbeLauncher::default();

        assert_eq!(
            launcher.launch((6, 9)).hit(&bb),
            Some((Pos { x: 21, y: -10 }, 45))
        );
        assert_eq!(launcher.launch((17, -4)).hit(&bb), None);
        assert_eq!(
            launcher.find_highest_hit(100, 100, &bb),
            Some((Pos { x: 6, y: 9 }, 45, 112))
        );
    }

    #[test]
    fn part2_examples() {}
}
