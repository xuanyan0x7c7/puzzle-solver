use clap::{Parser, Subcommand};
use cli::{CalendarArgs, SudokuArgs};

mod cli;

use crate::cli::{solve_calendar_puzzle, solve_sudoku_puzzle};

#[derive(Parser)]
#[command(name = "Puzzle Solver")]
#[command(version = "0.1.0")]
#[command(author = "xuanyan <xuanyan@xuanyan.ws>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve Calendar Puzzle
    Calendar(CalendarArgs),
    /// Solve Sudoku Puzzle
    Sudoku(SudokuArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Calendar(subcommand) => {
            solve_calendar_puzzle(subcommand);
        }
        Commands::Sudoku(subcommand) => {
            solve_sudoku_puzzle(subcommand);
        }
    }
}
