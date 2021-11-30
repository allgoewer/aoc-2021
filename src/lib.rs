/// Trait to be implemented by all 24 for quizzes of the advent of code
///
/// Implementations of this trait should return the days result (which is probably always numeric)
/// as a String.
pub trait Day {
    /// The first part of a days quiz
    fn part1(&self, _input: &str) -> String {
        String::new()
    }

    /// The second part of a days quiz
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
    /// # Panic
    /// This functions panics if the lines cannot be parsed.
    /// For Advent of Code, we assume that the input is properly formatted.
    pub fn parsed<T>(input: &str) -> impl Clone + Iterator<Item = T> + '_
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        input
            .lines()
            .map(|l| l.trim().parse::<T>().expect("Unable to parse AOC input"))
    }

    /// Parses a newline separated input into a [`Vec<T>`]
    ///
    /// # Panic
    /// This functions panics if the lines cannot be parsed.
    /// For Advent of Code, we assume that the input is properly formatted.
    pub fn collected<T>(input: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        parsed(input).collect()
    }
}
