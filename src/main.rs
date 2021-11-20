mod cli;

use chrono::NaiveDate;
use clap::{App, Arg, SubCommand};

use crate::cli::solve_calendar_puzzle;

fn main() {
    let matches = App::new("Puzzle Solver")
        .version("0.1.0")
        .author("xuanyan <xuanyan@xuanyan.ws>")
        .subcommand(
            SubCommand::with_name("calendar")
                .about("Solve Calendar Puzzle")
                .arg(
                    Arg::with_name("date")
                        .help("Date to solve")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("show all solutions"),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("calendar", Some(subcommand)) => {
            let input = subcommand.value_of("date").unwrap();
            match NaiveDate::parse_from_str(input, "%Y-%m-%d") {
                Ok(date) => {
                    let show_all_solutions = subcommand.is_present("all");
                    solve_calendar_puzzle(date, show_all_solutions);
                }
                Err(_) => {
                    eprintln!("Invalid date: {}", input);
                }
            }
        }
        _ => {}
    }
}
