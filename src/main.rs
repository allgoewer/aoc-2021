mod app;
mod day1;

use app::{app, help};

fn main() {
    let days = all_the_days![
        day1: "inputs/1",
    ];

    if let Err(e) = app(days) {
        eprintln!("{}", e);
        help(std::io::stderr());
        std::process::exit(1);
    }
}
