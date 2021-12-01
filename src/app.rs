//! Application logic
use aoc21::Day;
use std::io::{stdout, Write};
use std::process::exit;
use std::time::Instant;

#[macro_export]
macro_rules! all_the_days {
    ($module:ident -> $input:expr) => {{
        let day = Box::new($module::Today);
        let input = include_str!($input);

        (day, input)
    }};
    ($($module:ident: $input:expr),+) => {{
        let days: Vec<(Box<dyn ::aoc21::Day>, _)> = vec![
            $(all_the_days!($module -> $input)),+
        ];

        days
    }};
    ($($module:ident: $input:expr),+,) => {{
        all_the_days!($($module: $input),+)
    }};
}

/// Run the two parts of a day, printing their output and execution time
///
/// Note that the printed execution time does *NOT* have benchmark-quality.
fn run_day(index: usize, day: &dyn ::aoc21::Day, input: &str) {
    timed(index, 1, || day.part1(input));
    timed(index, 2, || day.part2(input));
    println!();
}

/// Execute a closure and print its output and execution time
///
/// Note that the printed execution time does *NOT* have benchmark-quality
fn timed<F>(day: usize, part: usize, func: F)
where
    F: Fn() -> String,
{
    let start = Instant::now();
    let result = func();
    let elapsed = start.elapsed();

    println!(
        "day{:0>2}-part{} {:>9} us {:>12}",
        day,
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
usage: aoc-2021 [-d DAY | --single-day DAY]

Run advent of code 2021

Options:
    -h, --help                  Print this help message
    -d, --single-day    DAY     Only run the specified day
    -l, --latest-only           Only run the latest day
"
    )
    .unwrap();
}

/// A description of the applications command line arguments
#[derive(Debug)]
struct Args {
    /// Option to run only a single day
    single_day: Option<usize>,
    /// Option to run only the latest available day
    latest_only: bool,
}

impl Args {
    /// Generates [`Args`] with the help of the pico-args crate
    fn try_from_pico_args() -> Result<Args, anyhow::Error> {
        let mut args = pico_args::Arguments::from_env();

        if args.contains(["-h", "--help"]) {
            help(stdout());
            exit(0);
        }

        Ok(Self {
            single_day: args.opt_value_from_str(["-d", "--single-day"])?,
            latest_only: args.contains(["-l", "--latest-only"]),
        })
    }
}

/// Runs the app
pub fn app(days: &[(Box<dyn Day>, &str)]) -> Result<(), anyhow::Error> {
    let args = Args::try_from_pico_args()?;

    let single_day = if args.latest_only {
        days.last().map(|d| (days.len(), d))
    } else if let Some(single_day) = args.single_day {
        Some((
            single_day,
            days.get(single_day - 1)
                .ok_or_else(|| anyhow::anyhow!("invalid day: {}", single_day))?,
        ))
    } else {
        None
    };

    if let Some((day_nr, (day, input))) = single_day {
        run_day(day_nr, day.as_ref(), input);
    } else {
        for (index, (day, input)) in days.iter().enumerate() {
            run_day(index + 1, day.as_ref(), input);
        }
    }

    Ok(())
}
