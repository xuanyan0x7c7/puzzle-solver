mod cli;

use clap::{App, Arg, SubCommand};

use crate::cli::{solve_calendar_puzzle, solve_sudoku_puzzle};

fn main() {
    let matches = App::new("Puzzle Solver")
        .version("0.1.0")
        .author("xuanyan <xuanyan@xuanyan.ws>")
        .subcommand(
            SubCommand::with_name("calendar")
                .about("Solve Calendar Puzzle")
                .arg(
                    Arg::with_name("date")
                        .required(true)
                        .index(1)
                        .help("Date to solve"),
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Show all solutions"),
                )
                .arg(
                    Arg::with_name("count")
                        .long("count")
                        .help("Count Solutions"),
                ),
        )
        .subcommand(
            SubCommand::with_name("sudoku")
                .about("Solve Sudoku Puzzle")
                .arg(
                    Arg::with_name("size")
                        .long("size")
                        .default_value("9")
                        .help("Board size"),
                )
                .arg(
                    Arg::with_name("alphabet")
                        .long("alphabet")
                        .help("Alphabet table"),
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Show all solutions"),
                )
                .arg(
                    Arg::with_name("count")
                        .long("count")
                        .help("Count Solutions"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("calendar", Some(subcommand)) => {
            solve_calendar_puzzle(subcommand);
        }
        ("sudoku", Some(subcommand)) => {
            solve_sudoku_puzzle(subcommand);
        }
        _ => {}
    }
}
