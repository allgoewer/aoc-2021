/// Trait to be implemented by all 25 quizzes of the Advent of Code
///
/// Implementations of this trait should return the quizzes result (which is probably always numeric) as a String.
pub trait Quizzer {
    /// The first part of a quiz
    fn part1(&self, _input: &str) -> String {
        String::new()
    }

    /// The second part of a quiz
    fn part2(&self, _input: &str) -> String {
        String::new()
    }
}

/// A set of utility functions useful for handling Advent of Code quizzes
pub mod util {
    use std::fmt::Debug;
    use std::str::FromStr;

    /// Parses a newline separated input into an [`Iterator`] over T's
    ///
    /// # Panics
    /// Panics if the lines cannot be parsed.
    pub fn parsed<T>(input: &str) -> impl Clone + Iterator<Item = T> + '_
    where
        T: FromStr,
        T::Err: Debug,
    {
        input
            .lines()
            .map(|l| l.trim().parse::<T>().expect("unable to parse AOC input"))
    }

    /// Parses a newline separated input into a [`Vec<T>`]
    ///
    /// # Panics
    /// Panics if the lines cannot be parsed.
    pub fn collected<T>(input: &str) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        parsed(input).collect()
    }
}
