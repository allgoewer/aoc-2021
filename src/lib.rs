/// Trait to be implemented by all 25 quizzes of the Advent of Code
///
/// Implementations of this trait should return the quizzes result (which is probably always numeric) as a String.
pub trait Quizzer {
    /// The first part of a quiz
    fn part1(&self, input: &str) -> String {
        let _ = input;
        String::new()
    }

    /// The second part of a quiz
    fn part2(&self, input: &str) -> String {
        let _ = input;
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

    /// Parses a newline separated input into an [`Iterator`] over T's
    ///
    /// # Panics
    /// Panics if the lines cannot be parsed.
    pub fn parsed_with<'input, F, T>(
        input: &'input str,
        f: F,
    ) -> impl Clone + Iterator<Item = T> + 'input
    where
        F: 'input + Clone + Fn(&'input str) -> T,
    {
        input.lines().map(str::trim).map(f)
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

    /// Parses a newline separated input into a [`Vec<T>`]
    ///
    /// # Errors
    /// If f can error, this may return the first occurence of the error, if any.
    pub fn collected_with<E, F, T>(input: &str, f: F) -> Result<Vec<T>, E>
    where
        F: Clone + Fn(&str) -> Result<T, E>,
    {
        parsed_with(input, f).collect()
    }
}
