//! Application logic
use aoc21::Quizzer;
use std::io::{stdout, Write};
use std::time::Instant;

/// Compose all [`aoc21::Quizzer`]s and their respective inputs into a [`Vec`]
#[macro_export]
macro_rules! all_the_quizzes {
    ($module:ident -> $input:expr) => {{
        let quiz = Box::new($module::Quiz);
        let input = include_str!($input);

        (quiz, input)
    }};
    ($($module:ident: $input:expr),+) => {{
        let quizzes: Vec<(Box<dyn ::aoc21::Quizzer>, _)> = vec![
            $(all_the_quizzes!($module -> $input)),+
        ];

        quizzes
    }};
    ($($module:ident: $input:expr),+,) => {{
        all_the_quizzes!($($module: $input),+)
    }};
}

/// Run the two parts of a quiz, printing their output and execution time
///
/// Note that the printed execution time does *NOT* have benchmark-quality.
fn run_quiz(index: usize, quiz: &dyn ::aoc21::Quizzer, input: &str) {
    timed(index, 1, || quiz.part1(input));
    timed(index, 2, || quiz.part2(input));
    println!();
}

/// Execute a closure and print its output and execution time
///
/// Note that the printed execution time does *NOT* have benchmark-quality
fn timed<F>(quiz: usize, part: usize, func: F)
where
    F: Fn() -> String,
{
    let start = Instant::now();
    let result = func();
    let elapsed = start.elapsed();

    println!(
        "quiz{:0>2}-part{} {:>9} us {:>16}",
        quiz,
        part,
        elapsed.as_micros(),
        result
    );
}

/// Write the help message to the given [`Write`]r
pub fn help<W: Write>(mut w: W) {
    write!(
        w,
        "
usage: aoc-2021 [-q QUIZ | --single-quiz QUIZ] [-l | --latest-only]

Run advent of code 2021

Options:
    -h, --help                  Print this help message
    -q, --single-quiz   QUIZ    Only run the specified quiz
    -l, --latest-only           Only run the latest quiz
"
    )
    .unwrap();
}

/// A description of the applications command line arguments
#[derive(Debug)]
struct Args {
    /// Option to print the help output
    help: bool,
    /// Option to run only a single quiz
    single_quiz: Option<usize>,
    /// Option to run only the latest available quiz
    latest_only: bool,
}

impl Args {
    /// Generates [`Args`] with the help of the pico-args crate
    fn try_from_pico_args() -> Result<Args, anyhow::Error> {
        let mut args = pico_args::Arguments::from_env();

        Ok(Self {
            help: args.contains(["-h", "--help"]),
            single_quiz: args.opt_value_from_str(["-q", "--single-quiz"])?,
            latest_only: args.contains(["-l", "--latest-only"]),
        })
    }
}

/// Runs the app
pub fn app(quizzes: &[(Box<dyn Quizzer>, &str)]) -> Result<(), anyhow::Error> {
    let args = Args::try_from_pico_args()?;

    if args.help {
        help(stdout());
        return Ok(());
    }

    let single_quiz = if args.latest_only {
        quizzes.last().map(|d| (quizzes.len(), d))
    } else if let Some(single_quiz) = args.single_quiz {
        Some((
            single_quiz,
            quizzes
                .get(single_quiz - 1)
                .ok_or_else(|| anyhow::anyhow!("invalid quiz: {}", single_quiz))?,
        ))
    } else {
        None
    };

    if let Some((quiz_nr, (quiz, input))) = single_quiz {
        run_quiz(quiz_nr, quiz.as_ref(), input);
    } else {
        for (index, (quiz, input)) in quizzes.iter().enumerate() {
            run_quiz(index + 1, quiz.as_ref(), input);
        }
    }

    Ok(())
}
