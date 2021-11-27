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
