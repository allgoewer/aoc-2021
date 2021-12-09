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

/// A set of utility functions and types useful for handling Advent of Code quizzes
pub mod util {
    use std::fmt::Debug;
    use std::ops::{Deref, DerefMut, Index, IndexMut};
    use std::str::FromStr;

    mod private {
        /// Used to seal the supers modules
        pub trait Sealed {}
    }

    /// GridPos is used to index a grid and to find out whether a point lies on the grid
    pub trait GridPos: private::Sealed {
        /// The position the GridPos represents
        fn pos<T>(&self, grid: &Grid<T>) -> (usize, usize);

        /// Whether the position is on the grid or not
        #[inline]
        fn is_on_grid<T>(&self, grid: &Grid<T>) -> bool {
            let (width, height) = grid.dim();
            let (x, y) = self.pos(grid);

            x < width && y < height
        }
    }

    impl private::Sealed for (usize, usize) {}

    impl GridPos for (usize, usize) {
        #[inline]
        fn pos<T>(&self, _grid: &Grid<T>) -> (usize, usize) {
            (self.0, self.1)
        }
    }

    /// A position on a [`Grid<T>`] which wraps around at the borders
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct WrappingPos(pub usize, pub usize);

    impl private::Sealed for WrappingPos {}

    impl GridPos for WrappingPos {
        #[inline]
        fn pos<T>(&self, grid: &Grid<T>) -> (usize, usize) {
            let (width, height) = grid.dim();
            (self.0 % width, self.1 % height)
        }

        #[inline]
        fn is_on_grid<T>(&self, _grid: &Grid<T>) -> bool {
            true
        }
    }

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
        /// The dimensions of the grid
        ///
        /// Returns (width, height) of the grid
        pub fn dim(&self) -> (usize, usize) {
            (self.width, self.height)
        }

        /// Whether a point lies within the grid
        pub fn contains<P: GridPos>(&self, pos: P) -> bool {
            pos.is_on_grid(self)
        }

        /// An iterator over a grid row
        pub fn row_iter(&self, row: usize) -> impl Iterator<Item = &T> {
            self.values.iter().skip(row * self.width).take(self.width)
        }

        /// An iterator over a grid column
        pub fn col_iter(&self, col: usize) -> impl Iterator<Item = &T> {
            self.values.iter().skip(col).step_by(self.width)
        }

        /// A mutable iterator over a grid row
        pub fn row_iter_mut(&mut self, row: usize) -> impl Iterator<Item = &mut T> {
            self.values.iter_mut().skip(row).step_by(self.width)
        }

        /// A mutable iterator over a grid column
        pub fn col_iter_mut(&mut self, col: usize) -> impl Iterator<Item = &mut T> {
            self.values.iter_mut().skip(col).step_by(self.width)
        }

        /// An iterator over all positions of the grid
        pub fn index_iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
            (0..self.height).flat_map(|y| (0..self.width).zip(std::iter::repeat(y)))
        }
    }

    impl<T> AsMut<[T]> for Grid<T> {
        #[inline]
        fn as_mut(&mut self) -> &mut [T] {
            &mut self.values
        }
    }

    impl<T> AsRef<[T]> for Grid<T> {
        #[inline]
        fn as_ref(&self) -> &[T] {
            &self.values
        }
    }

    impl<T> AsMut<Grid<T>> for Grid<T> {
        #[inline]
        fn as_mut(&mut self) -> &mut Grid<T> {
            self
        }
    }

    impl<T> AsRef<Grid<T>> for Grid<T> {
        #[inline]
        fn as_ref(&self) -> &Grid<T> {
            self
        }
    }

    impl<T> Deref for Grid<T> {
        type Target = [T];

        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.values
        }
    }

    impl<T> DerefMut for Grid<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.values
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

    impl<T> Index<WrappingPos> for Grid<T> {
        type Output = T;

        #[inline]
        fn index(&self, index: WrappingPos) -> &Self::Output {
            let (x, y) = index.pos(self);
            &self.values[x + y * self.width]
        }
    }

    impl<T> IndexMut<WrappingPos> for Grid<T> {
        #[inline]
        fn index_mut(&mut self, index: WrappingPos) -> &mut Self::Output {
            let (x, y) = index.pos(self);
            &mut self.values[x + y * self.width]
        }
    }

    impl<T> Index<(usize, usize)> for Grid<T> {
        type Output = T;

        #[inline]
        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            &self.values[x + y * self.width]
        }
    }

    impl<T> IndexMut<(usize, usize)> for Grid<T> {
        #[inline]
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
