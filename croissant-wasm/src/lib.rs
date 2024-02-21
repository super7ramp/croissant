use std::io::{BufRead, BufReader, Read};
use wasm_bindgen::prelude::wasm_bindgen;

use croissant_crossword::crossword::Crossword;
use croissant_solver_logicng::LogicngSolverBuilder;

#[wasm_bindgen]
pub fn solve(grid: String) -> Option<String> {
    let wordlist = ukacd();
    let crossword = Crossword::try_from(grid.as_str(), &wordlist).unwrap();
    let solver_builder = Box::new(LogicngSolverBuilder::new());
    crossword.solve_with_solver_built_by(solver_builder).next()
}

/// Reads words from bundled UKACD.
fn ukacd() -> Vec<String> {
    let bytes_of_ukacd = include_bytes!("../../wordlist/UKACD18plus.txt");
    read(&bytes_of_ukacd[..])
}

/// Reads and sanitizes words from a source supporting [Read].
fn read<T: Read>(data: T) -> Vec<String> {
    let alphabet = 'A'..='Z';
    BufReader::new(data)
        .lines()
        .map(Result::unwrap)
        .map(|word| word.replace(['-', '\'', '.'], "").to_uppercase())
        .filter(|word| word.chars().all(|letter| alphabet.contains(&letter)))
        .filter(|word| !word.is_empty())
        .collect()
}
