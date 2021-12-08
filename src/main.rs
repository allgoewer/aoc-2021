use app::{app, help};

mod app;
mod quiz01;
mod quiz02;
mod quiz03;
mod quiz07;
mod quiz08;

/// The applications main entry point
fn main() {
    let quizzes = all_the_quizzes![
        quiz01: "inputs/1",
        quiz02: "inputs/2",
        quiz03: "inputs/3",
        quiz07: "inputs/7",
        quiz08: "inputs/8",
    ];

    if let Err(e) = app(&quizzes) {
        eprintln!("{}", e);
        help(std::io::stderr());
        std::process::exit(1);
    }
}
