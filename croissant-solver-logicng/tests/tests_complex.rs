use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use croissant_crossword::crossword::{Crossword, CrosswordSolutions};
use croissant_solver_logicng::solver_logicng::LogicngSolverBuilder;

#[test]
fn empty3x3() {
    let mut solutions = solve("...\n...\n...");
    assert_eq!("BIZ\nONO\nAKA".to_string(), solutions.next().unwrap());
}

/// Solves the given grid using the logic-ng solver.
fn solve(grid: &str) -> CrosswordSolutions {
    let words = ukacd();
    let crossword = Crossword::from(grid, &words).unwrap();
    let solver = Box::new(LogicngSolverBuilder::new());
    crossword.solve_with(solver)
}

/// Reads the UKACD word list.
fn ukacd() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("UKACD18plus.txt");
    let file = File::open(path).expect("Test word list not found");
    io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .filter(|word| word.is_ascii())
        .map(|word| word.replace("-", "").replace("'", "").to_uppercase())
        .filter(|word| !word.is_empty())
        .collect()
}
