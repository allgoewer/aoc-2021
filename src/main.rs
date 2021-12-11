//! Advent of Code 2021

//#![deny(dead_code, unused_imports, unused_mut)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

use app::{app, help};

mod app;
mod quiz01;
mod quiz02;
mod quiz03;
mod quiz04;
mod quiz06;
mod quiz07;
mod quiz08;
mod quiz09;
mod quiz10;
mod quiz11;

/// The applications main entry point
fn main() {
    let quizzes = all_the_quizzes![
        quiz01: "inputs/1",
        quiz02: "inputs/2",
        quiz03: "inputs/3",
        quiz04: "inputs/4",
        quiz06: "inputs/6",
        quiz07: "inputs/7",
        quiz08: "inputs/8",
        quiz09: "inputs/9",
        quiz10: "inputs/10",
        quiz11: "inputs/11",
    ];

    if let Err(e) = app(&quizzes) {
        eprintln!("{}", e);
        help(std::io::stderr());
        std::process::exit(1);
    }
}
