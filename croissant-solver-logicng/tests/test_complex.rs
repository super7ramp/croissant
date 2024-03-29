use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use croissant_crossword::crossword::{Crossword, CrosswordSolutions};
use croissant_solver_logicng::LogicngSolverBuilder;

#[test]
fn empty3x3() {
    let mut solutions = solve("...\n...\n...");
    assert_eq!(Some("BIZ\nONO\nAKA".to_string()), solutions.next());
}

#[test]
#[ignore = "too long (3m29s at 1GHz)"]
fn empty4x4() {
    let mut solutions = solve("....\n....\n....\n....");
    assert_eq!(Some("EGIS\nGADI\nGLEG\nYEAH".to_string()), solutions.next());
}

#[test]
#[ignore = "too long (1m48s at 1GHz)"]
fn shaded5x5() {
    let mut solutions = solve("##..#\n#...#\n.....\n#...#\n##.##");
    assert_eq!(
        Some("##AB#\n#ECU#\nLARRY\n#SEY#\n##S##".to_string()),
        solutions.next()
    );
}

/// Solves the given grid using the logic-ng solver.
fn solve(grid: &str) -> CrosswordSolutions {
    let words = ukacd();
    let crossword = Crossword::try_from(grid, &words).unwrap();
    let solver = Box::new(LogicngSolverBuilder::new());
    crossword.solve_with_solver_built_by(solver)
}

/// Reads the UKACD word list.
fn ukacd() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wordlist")
        .join("UKACD18plus.txt");
    let file = File::open(path).expect("Test word list not found");
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|word| word.replace(['-', '\'', '.'], "").to_uppercase())
        .filter(|word| word.chars().all(|letter| letter >= 'A' && letter <= 'Z'))
        .filter(|word| !word.is_empty())
        .collect()
}
