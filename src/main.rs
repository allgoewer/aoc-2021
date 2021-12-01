use app::{app, help};

mod app;
mod quiz01;

/// The applications main entry point
fn main() {
    let quizzes = all_the_quizzes![
        quiz01: "inputs/1",
    ];

    if let Err(e) = app(&quizzes) {
        eprintln!("{}", e);
        help(std::io::stderr());
        std::process::exit(1);
    }
}
