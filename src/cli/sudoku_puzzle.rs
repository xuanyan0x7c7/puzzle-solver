use puzzle_solver::DancingLinks;

pub fn solve_sudoku_puzzle(
    box_size: (usize, usize),
    board_string: &str,
    alphabet: Option<&str>,
    show_all_solutions: bool,
) {
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

    let mut solver = DancingLinks::new();
    for _ in 0..board_size * board_size {
        solver.add_rows(board_size);
    }
    for row in 0..board_size {
        for number in 0..board_size {
            let mut list = vec![];
            for column in 0..board_size {
                list.push((row * board_size + column) * board_size + number);
            }
            solver.add_constraint(&list, true);
        }
    }
    for column in 0..board_size {
        for number in 0..board_size {
            let mut list = vec![];
            for row in 0..board_size {
                list.push((row * board_size + column) * board_size + number);
            }
            solver.add_constraint(&list, true);
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
            solver.add_constraint(&list, true);
        }
    }
    for row in 0..board_size {
        for column in 0..board_size {
            if board[row][column] != 0 {
                solver.select(
                    (row * board_size + column) * board_size + (board[row][column] as usize - 1),
                );
            }
        }
    }

    let print_solution = |solution: &Vec<usize>| {
        let mut solution_board = vec![vec![0; board_size]; board_size];
        for row in solution.iter() {
            solution_board[row / board_size / board_size][row / board_size % board_size] =
                row % board_size + 1;
        }

        print!("+-");
        for column in 0..board_size {
            if column > 0 {
                print!("-");
                if column % box_size.1 == 0 {
                    print!("+-");
                }
            }
            print!("-");
        }
        println!("-+");

        for row in 0..board_size {
            if row > 0 && row % box_size.0 == 0 {
                print!("+-");
                for column in 0..board_size {
                    if column > 0 {
                        print!("-");
                        if column % box_size.1 == 0 {
                            print!("+-");
                        }
                    }
                    print!("-");
                }
                println!("-+");
            }
            print!("| ");
            for column in 0..board_size {
                if column > 0 {
                    print!(" ");
                    if column % box_size.1 == 0 {
                        print!("| ");
                    }
                }
                print!(
                    "{}",
                    alphabet_bytes[solution_board[row][column] - 1] as char
                );
            }
            println!(" |");
        }

        print!("+-");
        for column in 0..board_size {
            if column > 0 {
                print!("-");
                if column % box_size.1 == 0 {
                    print!("+-");
                }
            }
            print!("-");
        }
        println!("-+");
    };

    let mut solution_found = false;
    if show_all_solutions {
        for (solution_index, solution) in solver.solve().enumerate() {
            if solution_index != 0 {
                println!();
            }
            println!("{}: ", solution_index + 1);
            print_solution(&solution);
            solution_found = true;
        }
    } else {
        for solution in solver.solve() {
            print_solution(&solution);
            solution_found = true;
            break;
        }
    }
    if !solution_found {
        println!("No solution found!");
    }
}
