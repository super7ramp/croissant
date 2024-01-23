use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use croissant_crossword::crossword::{Crossword, CrosswordSolutions};
use croissant_solver_logicng::solver_logicng::LogicngSolverBuilder;

#[test]
fn empty3x3() {
    let mut solutions = solve("...\n...\n...");
    assert_eq!("BIZ\nONO\nAKA".to_string(), solutions.next().unwrap());
}

#[test]
#[ignore = "too long (2m53s at 1GHz)"]
fn empty4x4() {
    let mut solutions = solve("....\n....\n....\n....");
    assert_eq!(
        "EGIS\nGADI\nGLEG\nYEAH".to_string(),
        solutions.next().unwrap()
    );
}

/// Solves the given grid using the logic-ng solver.
fn solve(grid: &str) -> CrosswordSolutions {
    let words = ukacd();
    let crossword = Crossword::from(grid, &words).unwrap();
    let solver = Box::new(LogicngSolverBuilder::new());
    crossword.solve_with_solver_built_by(solver)
}

/// Reads the UKACD word list.
fn ukacd() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("UKACD18plus.txt");
    let file = File::open(path).expect("Test word list not found");
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .filter(|word| word.is_ascii())
        .map(|word| {
            word.replace("-", "")
                .replace("'", "")
                .replace(".", "")
                .to_uppercase()
        })
        .filter(|word| !word.is_empty())
        .collect()
}
