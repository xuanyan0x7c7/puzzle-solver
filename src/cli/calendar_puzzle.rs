use std::collections::{HashMap, HashSet};

use chrono::{Datelike, Local, NaiveDate, Weekday};
use clap::Args;

use puzzle_solver::PuzzleSolver;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Tile {
    points: Vec<Point>,
}

impl Tile {
    fn new(points: Vec<Point>) -> Self {
        let mut tile = Tile { points };
        tile.points.sort();
        tile.adjust();
        tile
    }

    fn adjust(&mut self) {
        let min_x = self.points.iter().map(|point| point.x).min().unwrap();
        let min_y = self.points.iter().map(|point| point.y).min().unwrap();
        for point in self.points.iter_mut() {
            point.x -= min_x;
            point.y -= min_y;
        }
    }

    fn rotate(&self) -> Self {
        Self::new(
            self.points
                .iter()
                .map(|&point| Point {
                    x: -point.y,
                    y: point.x,
                })
                .collect(),
        )
    }

    fn flip(&self) -> Self {
        Self::new(
            self.points
                .iter()
                .map(|&point| Point {
                    x: -point.x,
                    y: point.y,
                })
                .collect(),
        )
    }

    fn gravity_support(&self) -> Vec<Point> {
        self.points
            .iter()
            .map(|&point| Point {
                x: point.x,
                y: point.y + 1,
            })
            .filter(|point| !self.points.contains(point))
            .collect()
    }
}

fn generate_tiles(basic_tile: &Tile) -> Vec<Tile> {
    let mut tile_set = HashSet::new();
    tile_set.insert(basic_tile.clone());
    let mut rotated_tile = basic_tile.rotate();
    while !tile_set.contains(&rotated_tile) {
        tile_set.insert(rotated_tile.clone());
        rotated_tile = rotated_tile.rotate();
    }
    let flipped_tile = basic_tile.flip();
    if !tile_set.contains(&flipped_tile) {
        tile_set.insert(flipped_tile.clone());
        let mut rotated_tile = flipped_tile.rotate();
        while !tile_set.contains(&rotated_tile) {
            tile_set.insert(rotated_tile.clone());
            rotated_tile = rotated_tile.rotate();
        }
    }
    tile_set.into_iter().collect()
}

#[derive(Args)]
pub struct CalendarArgs {
    /// Date to solve
    #[arg(value_parser = parse_date)]
    date: Option<NaiveDate>,
    /// No gravity
    #[arg(long)]
    no_gravity: bool,
    /// Show all solutions
    #[arg(short, long)]
    all: bool,
    /// Count Solutions
    #[arg(long)]
    count: bool,
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(format!("Invalid date: {s}")),
    }
}

pub fn solve_calendar_puzzle(args: &CalendarArgs) {
    let date = args.date.unwrap_or_else(|| Local::now().date_naive());
    let has_gravity = !args.no_gravity;
    let show_all_solutions = args.all;
    let count_solutions = args.count;

    let board_size = (9, 6);
    let basic_tile_list = vec![
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ]),
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 0, y: 1 },
        ]),
        Tile::new(vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ]),
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
        ]),
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ]),
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 0, y: 1 },
        ]),
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 1, y: 1 },
        ]),
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ]),
        Tile::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 1, y: 2 },
        ]),
    ];
    let holes = vec![
        Point {
            x: (date.month() as i32 - 1) % 6,
            y: (date.month() as i32 - 1) / 6,
        },
        Point {
            x: (date.day() as i32 - 1) % 6,
            y: (date.day() as i32 - 1) / 6 + 2,
        },
        match date.weekday() {
            Weekday::Sun => Point { x: 5, y: 8 },
            Weekday::Mon => Point { x: 3, y: 7 },
            Weekday::Tue => Point { x: 4, y: 7 },
            Weekday::Wed => Point { x: 5, y: 7 },
            Weekday::Thu => Point { x: 2, y: 8 },
            Weekday::Fri => Point { x: 3, y: 8 },
            Weekday::Sat => Point { x: 4, y: 8 },
        },
    ];

    let mut solver = PuzzleSolver::new();
    let mut overlap_mapping = HashMap::<Point, Vec<usize>>::new();
    let mut tile_list = vec![];
    let mut tile_index = 0;
    let mut row_list = vec![];
    for (basic_tile_index, basic_tile) in basic_tile_list.iter().enumerate() {
        let tiles = generate_tiles(basic_tile);
        let mut row_count = 0;
        for tile in tiles.iter() {
            for x in 0..board_size.1 {
                for y in 0..board_size.0 {
                    let can_put = tile.points.iter().all(|&point| {
                        x + point.x < board_size.1
                            && y + point.y < board_size.0
                            && !holes.contains(&Point {
                                x: x + point.x,
                                y: y + point.y,
                            })
                    });
                    let will_fall = tile.gravity_support().iter().all(|&point| {
                        holes.contains(&Point {
                            x: x + point.x,
                            y: y + point.y,
                        })
                    });
                    if can_put && (!has_gravity || !will_fall) {
                        for &point in tile.points.iter() {
                            let overlapping_point = Point {
                                x: x + point.x,
                                y: y + point.y,
                            };
                            match overlap_mapping.get_mut(&overlapping_point) {
                                Some(list) => {
                                    list.push(tile_index);
                                }
                                None => {
                                    overlap_mapping.insert(overlapping_point, vec![tile_index]);
                                }
                            }
                        }
                        row_list.push((tile.clone(), basic_tile_index, x, y));
                        tile_index += 1;
                        row_count += 1;
                    }
                }
            }
        }
        solver.add_rows(row_count);
        tile_list.extend(tiles);
    }

    for list in overlap_mapping.into_values() {
        if !list.is_empty() {
            solver.add_column(list.into_iter());
        }
    }

    let print_solution = |solution: &Vec<usize>| {
        let mut board = vec![vec![0; board_size.1 as usize]; board_size.0 as usize];
        for &row in solution.iter() {
            let (tile, basic_tile_index, x, y) = &row_list[row];
            for &point in tile.points.iter() {
                board[(y + point.y) as usize][(x + point.x) as usize] = basic_tile_index + 1;
            }
        }
        print!("┌");
        for column_index in 0..board_size.1 as usize {
            if column_index > 0 {
                let row = &board[0];
                if row[column_index - 1] == row[column_index] && row[column_index] > 0 {
                    print!("─");
                } else {
                    print!("┬");
                }
            }
            print!("───");
        }
        println!("┐");
        for (row_index, row) in board.iter().enumerate() {
            if row_index > 0 {
                if board[row_index - 1][0] == board[row_index][0] && board[row_index][0] > 0 {
                    print!("│");
                } else {
                    print!("├");
                }
                for column_index in 0..board_size.1 as usize {
                    let top_right = board[row_index - 1][column_index];
                    let bottom_right = row[column_index];
                    if column_index > 0 {
                        let top_left = board[row_index - 1][column_index - 1];
                        let bottom_left = row[column_index - 1];
                        if top_left == top_right && top_left > 0 {
                            if bottom_left == bottom_right && bottom_left > 0 {
                                print!("{}", if top_left == bottom_left { " " } else { "─" });
                            } else if top_left == bottom_left {
                                print!("┌");
                            } else if top_right == bottom_right {
                                print!("┐");
                            } else {
                                print!("┬");
                            }
                        } else if bottom_left == bottom_right && bottom_left > 0 {
                            if top_left == bottom_left {
                                print!("└");
                            } else if top_right == bottom_right {
                                print!("┘");
                            } else {
                                print!("┴");
                            }
                        } else if top_left == bottom_left && top_left > 0 {
                            if top_right == bottom_right && top_right > 0 {
                                print!("│");
                            } else {
                                print!("├");
                            }
                        } else if top_right == bottom_right && top_right > 0 {
                            print!("┤");
                        } else {
                            print!("┼");
                        }
                    }
                    if top_right == bottom_right && top_right > 0 {
                        print!("   ");
                    } else {
                        print!("───");
                    }
                }
                if board[row_index - 1][board_size.1 as usize - 1]
                    == board[row_index][board_size.1 as usize - 1]
                    && board[row_index][board_size.1 as usize - 1] > 0
                {
                    println!("│");
                } else {
                    println!("┤");
                }
            }
            print!("│");
            for (column_index, &color) in row.iter().enumerate() {
                if column_index != 0 {
                    if row[column_index - 1] == color && color > 0 {
                        print!(" ");
                    } else {
                        print!("│");
                    }
                }
                print!(" {} ", if color == 0 { "x" } else { " " });
            }
            println!("│");
        }
        print!("└");
        for column_index in 0..board_size.1 as usize {
            if column_index > 0 {
                let row = &board[board_size.0 as usize - 1];
                if row[column_index - 1] == row[column_index] && row[column_index] > 0 {
                    print!("─");
                } else {
                    print!("┴");
                }
            }
            print!("───");
        }
        println!("┘");
    };

    let mut solution_count = 0usize;
    if show_all_solutions {
        for (solution_index, solution) in solver.solve().enumerate() {
            if solution_index != 0 {
                println!();
            }
            println!("{}:", solution_index + 1);
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
            "Total {solution_count} solution{}.",
            if solution_count == 1 { "" } else { "s" }
        );
    }
}
