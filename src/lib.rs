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
    /// Parses a newline separated input into an [`Iterator`] over T's
    ///
    /// # Panic
    /// This functions panics if the lines cannot be parsed.
    /// For Advent of Code, we assume that the input is properly formatted.
    pub fn parsed<T>(input: &str) -> impl Clone + Iterator<Item = T> + '_
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        input
            .lines()
            .map(|l| l.trim().parse::<T>().expect("Unable to parse AOC input"))
    }
}
