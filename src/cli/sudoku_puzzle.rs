use std::io::stdin;
use std::process;

use clap::Args;

use puzzle_solver::PuzzleSolver;

#[derive(Args)]
pub struct SudokuArgs {
    /// Board size
    #[arg(long, value_parser = parse_size)]
    size: Option<(usize, usize)>,
    /// Alphabet table
    #[arg(long)]
    alphabet: Option<String>,
    /// Show all solutions
    #[arg(short, long)]
    all: bool,
    /// Count Solutions
    #[arg(long)]
    count: bool,
}

fn parse_size(s: &str) -> Result<(usize, usize), String> {
    match s {
        "4" => Ok((2, 2)),
        "9" => Ok((3, 3)),
        "16" => Ok((4, 4)),
        "25" => Ok((5, 5)),
        _ => {
            let l: Vec<&str> = s.split('x').collect();
            if let (Ok(row), Ok(column)) = (l[0].parse(), l[1].parse()) {
                Ok((row, column))
            } else {
                Err(format!("Invalid box size: {s}"))
            }
        }
    }
}

pub fn solve_sudoku_puzzle(args: &SudokuArgs) {
    let (box_width, box_height) = args.size.unwrap_or((3,3));
    let mut input = String::new();
    let board_string = match stdin().read_line(&mut input) {
        Ok(_) => input.trim_end(),
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    };
    let show_all_solutions = args.all;
    let count_solutions = args.count;

    let board_size = box_width * box_height;
    let alphabet = match &args.alphabet {
        Some(string) => string.clone(),
        None => {
            if board_size > 9 {
                "ABDCEFGHIJKLMNOPQRSTUVWXYZ".to_owned()
            } else {
                "123456789".to_owned()
            }
        }
    };
    let alphabet_bytes = alphabet.as_bytes();

    if board_string.len() != board_size * board_size {
        eprintln!("Invalid board string");
        return;
    }
    let mut board = vec![vec![0; board_size]; board_size];
    for (index, c) in board_string.char_indices() {
        if let Some(offset) = alphabet.find(c) {
            if offset < board_size {
                board[index / board_size][index % board_size] = offset + 1;
            }
        }
    }

    let mut solver = PuzzleSolver::new();
    for _ in 0..board_size * board_size {
        solver.add_rows(board_size);
    }
    for row in 0..board_size {
        for number in 0..board_size {
            solver.add_column(
                (0..board_size).map(|column| (row * board_size + column) * board_size + number),
            );
        }
    }
    for column in 0..board_size {
        for number in 0..board_size {
            solver.add_column(
                (0..board_size).map(|row| (row * board_size + column) * board_size + number),
            );
        }
    }
    for sudoku_box in 0..board_size {
        for number in 0..board_size {
            let mut list = vec![];
            for r in 0..box_width {
                for c in 0..box_height {
                    let row = sudoku_box / box_width * box_width + r;
                    let column = sudoku_box % box_width * box_height + c;
                    list.push((row * board_size + column) * board_size + number);
                }
            }
            eprintln!("{:?}", list);
            solver.add_column(list.into_iter());
        }
    }
    for (row, board_row) in board.iter().enumerate() {
        for (column, &board_cell) in board_row.iter().enumerate() {
            if board_cell != 0 {
                solver.select_row((row * board_size + column) * board_size + (board_cell - 1));
            }
        }
    }

    let print_solution = |solution: Vec<usize>| {
        let mut solution_board = vec![vec![0; board_size]; board_size];
        for row in solution {
            solution_board[row / board_size / board_size][row / board_size % board_size] =
                row % board_size + 1;
        }

        print!("┌─");
        for column in 0..board_size {
            if column > 0 {
                print!("─");
                if column % box_height == 0 {
                    print!("┬─");
                }
            }
            print!("─");
        }
        println!("─┐");

        for row in 0..board_size {
            if row > 0 && row % box_width == 0 {
                print!("├─");
                for column in 0..board_size {
                    if column > 0 {
                        print!("─");
                        if column % box_height == 0 {
                            print!("┼─");
                        }
                    }
                    print!("─");
                }
                println!("─┤");
            }
            print!("│ ");
            for column in 0..board_size {
                if column > 0 {
                    print!(" ");
                    if column % box_height == 0 {
                        print!("│ ");
                    }
                }
                print!(
                    "{}",
                    alphabet_bytes[solution_board[row][column] - 1] as char
                );
            }
            println!(" │");
        }

        print!("└─");
        for column in 0..board_size {
            if column > 0 {
                print!("─");
                if column % box_height == 0 {
                    print!("┴─");
                }
            }
            print!("─");
        }
        println!("─┘");
    };

    let mut solution_count = 0usize;
    if show_all_solutions {
        for (solution_index, solution) in solver.solve().enumerate() {
            if solution_index != 0 {
                println!();
            }
            println!("{}: ", solution_index + 1);
            print_solution(solution);
            solution_count += 1;
        }
    } else {
        for (solution_index, solution) in solver.solve().enumerate() {
            if solution_index == 0 {
                print_solution(solution);
            }
            solution_count += 1;
            if !count_solutions {
                break;
            }
        }
    }
    if solution_count == 0 {
        println!("No solution found!");
    } else if count_solutions {
        println!();
        println!(
            "Total {solution_count} solution{}.",
            if solution_count == 1 { "" } else { "s" }
        );
    }
}
