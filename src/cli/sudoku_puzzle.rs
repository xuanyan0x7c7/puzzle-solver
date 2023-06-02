use clap::ArgMatches;
use puzzle_solver::PuzzleSolver;
use std::io::stdin;
use std::process;

pub fn solve_sudoku_puzzle(subcommand: &ArgMatches) {
    let size_string = subcommand.value_of("size").unwrap();
    let box_size = if size_string == "4" {
        (2, 2)
    } else if size_string == "9" {
        (3, 3)
    } else if size_string == "16" {
        (4, 4)
    } else if size_string == "25" {
        (5, 5)
    } else {
        let l: Vec<&str> = size_string.split('x').collect();
        let box_row = l[0].parse();
        let box_column = l[1].parse();
        if let (Ok(row), Ok(column)) = (box_row, box_column) {
            (row, column)
        } else {
            (3, 3)
        }
    };
    let mut input = String::new();
    let board_string = match stdin().read_line(&mut input) {
        Ok(_) => input.trim_end(),
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };
    let alphabet = subcommand.value_of("alphabet");
    let show_all_solutions = subcommand.is_present("all");
    let count_solutions = subcommand.is_present("count");

    let board_size = box_size.0 * box_size.1;
    let alphabet = match alphabet {
        Some(string) => string,
        None => {
            if board_size > 9 {
                "ABDCEFGHIJKLMNOPQRSTUVWXYZ"
            } else {
                "123456789"
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
            let mut list = vec![];
            for column in 0..board_size {
                list.push((row * board_size + column) * board_size + number);
            }
            solver.add_column(list.into_iter());
        }
    }
    for column in 0..board_size {
        for number in 0..board_size {
            let mut list = vec![];
            for row in 0..board_size {
                list.push((row * board_size + column) * board_size + number);
            }
            solver.add_column(list.into_iter());
        }
    }
    for sudoku_box in 0..board_size {
        for number in 0..board_size {
            let mut list = vec![];
            for r in 0..box_size.0 {
                for c in 0..box_size.1 {
                    let row = sudoku_box / box_size.0 * box_size.0 + r;
                    let column = sudoku_box % box_size.0 * box_size.1 + c;
                    list.push((row * board_size + column) * board_size + number);
                }
            }
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

    let print_solution = |solution: &Vec<usize>| {
        let mut solution_board = vec![vec![0; board_size]; board_size];
        for &row in solution.iter() {
            solution_board[row / board_size / board_size][row / board_size % board_size] =
                row % board_size + 1;
        }

        print!("┌─");
        for column in 0..board_size {
            if column > 0 {
                print!("─");
                if column % box_size.1 == 0 {
                    print!("┬─");
                }
            }
            print!("─");
        }
        println!("─┐");

        for row in 0..board_size {
            if row > 0 && row % box_size.0 == 0 {
                print!("├─");
                for column in 0..board_size {
                    if column > 0 {
                        print!("─");
                        if column % box_size.1 == 0 {
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
                    if column % box_size.1 == 0 {
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
                if column % box_size.1 == 0 {
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
            print_solution(&solution);
            solution_count += 1;
        }
    } else {
        for (solution_index, solution) in solver.solve().enumerate() {
            if solution_index == 0 {
                print_solution(&solution);
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
            "Total {} solution{}.",
            solution_count,
            if solution_count == 1 { "" } else { "s" }
        );
    }
}
