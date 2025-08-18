use crate::solver::SudokuSolver;

mod all_equal;
mod board;
mod join;
mod possibility_matrix;
mod region;
mod solver;
mod subset;

const KNOWN_VALUES: [(usize, usize, u16); 30] = [
    (0, 0, 5),
    (0, 1, 3),
    (0, 4, 7),
    (1, 0, 6),
    (1, 3, 1),
    (1, 4, 9),
    (1, 5, 5),
    (2, 1, 9),
    (2, 2, 8),
    (2, 7, 6),
    (3, 0, 8),
    (3, 4, 6),
    (3, 8, 3),
    (4, 0, 4),
    (4, 3, 8),
    (4, 5, 3),
    (4, 8, 1),
    (5, 0, 7),
    (5, 4, 2),
    (5, 8, 6),
    (6, 1, 6),
    (6, 6, 2),
    (6, 7, 8),
    (7, 3, 4),
    (7, 4, 1),
    (7, 5, 9),
    (7, 8, 5),
    (8, 4, 8),
    (8, 7, 7),
    (8, 8, 9),
];

const KNOWN_VALUES2: [(usize, usize, u16); 24] = [
    (0, 1, 5),
    (0, 4, 6),
    (0, 7, 3),
    (1, 0, 4),
    (1, 2, 8),
    (1, 3, 5),
    (2, 0, 3),
    (2, 8, 8),
    (3, 0, 8),
    (3, 2, 7),
    (3, 3, 3),
    (4, 1, 1),
    (5, 6, 6),
    (5, 7, 8),
    (5, 8, 4),
    (6, 1, 6),
    (6, 3, 1),
    (6, 6, 4),
    (6, 8, 7),
    (7, 7, 9),
    (7, 8, 1),
    (8, 1, 9),
    (8, 5, 4),
    (8, 8, 5),
];

const KNOWN_VALUES3: [(usize, usize, u16); 17] = [
    (1, 5, 3),
    (1, 7, 8),
    (1, 8, 5),
    (2, 2, 1),
    (2, 4, 2),
    (3, 3, 5),
    (3, 5, 7),
    (4, 2, 4),
    (4, 6, 1),
    (5, 1, 9),
    (6, 0, 5),
    (6, 7, 7),
    (6, 8, 3),
    (7, 2, 2),
    (7, 4, 1),
    (8, 4, 4),
    (8, 8, 9),
];

fn main() {
    let mut sudoku_solver = SudokuSolver::<9>::new();
    for (index, known) in KNOWN_VALUES.into_iter().enumerate() {
        println!(
            "{:}. {:}, at ({:},{:})",
            index + 1,
            known.2,
            known.0,
            known.1
        );
        sudoku_solver.set(known.0, known.1, known.2)
    }

    let solved_board = sudoku_solver.solve();
    match solved_board {
        Ok(solved_board) => {
            println!("Final Board:\n{:?}", solved_board);
            println!("Solved:\n{:}", solved_board);
        }
        Err(msg) => {
            println!("{:}", msg);
        }
    };
}
