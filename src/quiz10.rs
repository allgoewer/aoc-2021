//! Day 10: Syntax Scoring
use aoc21::Quizzer;
use State::*;

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        lines_score(input.lines()).to_string()
    }

    fn part2(&self, input: &str) -> String {
        lines_completion_score(input.lines()).to_string()
    }
}

/// The states in which a line can be in
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum State {
    /// The line is missing closing delimiters
    Incomplete,
    /// The line contains at least one unmatched closing delimiter
    Corrupted(char),
}

/// Get the state and missing delimiters of a single line
fn line_state(line: &str) -> (State, Vec<char>) {
    let mut stack = Vec::new();

    for c in line.chars() {
        match (c, stack.last()) {
            ('(', _) => stack.push(')'),
            ('[', _) => stack.push(']'),
            ('{', _) => stack.push('}'),
            ('<', _) => stack.push('>'),
            (')', Some(')')) | (']', Some(']')) | ('}', Some('}')) | ('>', Some('>')) => {
                stack.pop();
            }
            (c, Some(_)) => return (Corrupted(c), stack),
            _ => unreachable!(),
        }
    }

    (Incomplete, stack)
}

/// Score a line according to part 1
fn lines_score<'input>(lines: impl Iterator<Item = &'input str>) -> u64 {
    lines
        .map(|l| {
            let (state, _) = line_state(l.trim());
            match state {
                Incomplete => 0,
                Corrupted(')') => 3,
                Corrupted(']') => 57,
                Corrupted('}') => 1197,
                Corrupted('>') => 25137,
                Corrupted(_) => unreachable!(),
            }
        })
        .sum()
}

/// Score a line according to part 2
fn line_completion(line: &str) -> u64 {
    let (state, stack) = line_state(line);

    if let Corrupted(_) = state {
        return 0;
    }

    stack.into_iter().rev().fold(0, |score, c| {
        score * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
    })
}

/// Get the middle score according to part 2
fn lines_completion_score<'input>(lines: impl Iterator<Item = &'input str>) -> u64 {
    let mut scores: Vec<_> = lines
        .map(str::trim)
        .map(line_completion)
        .filter(|&score| score > 0)
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_examples() {
        assert_eq!(lines_score(EXAMPLE.lines()), 26397);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(lines_completion_score(EXAMPLE.lines()), 288957);
    }
}
