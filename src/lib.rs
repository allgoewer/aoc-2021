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
    use std::ops::{Index, IndexMut};
    use std::str::FromStr;

    /// A two-dimensional grid of values
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Grid<T> {
        /// The underlying values
        values: Vec<T>,
        /// The width of the grid
        width: usize,
        /// The height of the grid
        height: usize,
    }

    impl<T> Grid<T> {
        /// Get the number of values in the grid
        pub fn len(&self) -> usize {
            self.values.len()
        }

        /// Whether the grid contains no values
        ///
        /// Equivalent to `grid.len() == 0`
        pub fn is_empty(&self) -> bool {
            self.values.len() == 0
        }

        /// The dimensions of the grid
        ///
        /// Returns (width, height) of the grid
        pub fn dim(&self) -> (usize, usize) {
            (self.width, self.height)
        }

        /// Whether a point lies within the grid
        pub fn contains(&self, (x, y): (usize, usize)) -> bool {
            x < self.width && y < self.height
        }

        /// An iterator over all positions of the grid
        pub fn index_iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
            (0..self.height).flat_map(|y| (0..self.width).zip(std::iter::repeat(y)))
        }
    }

    impl<T> TryFrom<(Vec<T>, usize)> for Grid<T> {
        type Error = anyhow::Error;

        fn try_from((values, width): (Vec<T>, usize)) -> Result<Self, Self::Error> {
            if width == 0 {
                return Err(anyhow::anyhow!("width can not be zero"));
            }

            let height = values.len() / width;

            if width * height == values.len() {
                Ok(Self {
                    values,
                    width,
                    height,
                })
            } else {
                Err(anyhow::anyhow!("grid is not rectangular"))
            }
        }
    }

    impl<T> Index<(usize, usize)> for Grid<T> {
        type Output = T;

        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            &self.values[x + y * self.width]
        }
    }

    impl<T> IndexMut<(usize, usize)> for Grid<T> {
        fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
            &mut self.values[x + y * self.width]
        }
    }

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
