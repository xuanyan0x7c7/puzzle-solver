mod cli;

use chrono::NaiveDate;
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
                    Arg::with_name("board")
                        .required(true)
                        .index(1)
                        .help("Board string"),
                )
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
            let input = subcommand.value_of("date").unwrap();
            match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
                Ok(date) => {
                    solve_calendar_puzzle(
                        date,
                        subcommand.is_present("all"),
                        subcommand.is_present("count"),
                    );
                }
                Err(_) => {
                    eprintln!("Invalid date: {}", input);
                }
            }
        }
        ("sudoku", Some(subcommand)) => {
            let size_string = subcommand.value_of("size").unwrap();
            let board_string = subcommand.value_of("board").unwrap();
            solve_sudoku_puzzle(
                if size_string == "16" { (4, 4) } else { (3, 3) },
                board_string,
                subcommand.value_of("alphabet"),
                subcommand.is_present("all"),
                subcommand.is_present("count"),
            );
        }
        _ => {}
    }
}
