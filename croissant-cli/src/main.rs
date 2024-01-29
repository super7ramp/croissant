use clap::Parser;
use croissant_crossword::crossword::{Crossword, CrosswordSolutions};
use croissant_solver_cadical::CadicalSolver;
use croissant_solver_logicng::LogicngSolverBuilder;
use croissant_solver_splr::SplrSolverBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The grid.
    grid: String,
    /// The path to the word list.
    #[arg(short, long)]
    word_list: String,
    /// The solver to use.
    #[arg(short, long, default_value = "logicng")]
    solver: String,
    /// The desired number of solutions.
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn main() {
    let args = Args::parse();
    let words = read_words(args.word_list);
    let crossword = Crossword::try_from(args.grid.as_str(), &words).unwrap();
    let mut solutions = solve(crossword, args.solver);
    iterate_and_print(args.count, &mut solutions);
}

fn read_words(path: String) -> Vec<String> {
    let path = PathBuf::from(path);
    let file = File::open(path).expect("Test word list not found");
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|word| word.replace(['-', '\'', '.'], "").to_uppercase())
        .filter(|word| word.chars().all(|letter| letter >= 'A' && letter <= 'Z'))
        .filter(|word| !word.is_empty())
        .collect()
}

fn solve(crossword: Crossword, solver_name: String) -> CrosswordSolutions {
    match solver_name.as_str() {
        // TODO create an enum
        "cadical" => crossword.solve_with(Box::new(CadicalSolver::new())),
        "logicng" => {
            let solver_builder = Box::new(LogicngSolverBuilder::new());
            crossword.solve_with_solver_built_by(solver_builder)
        }
        "splr" => {
            let solver_builder = Box::new(SplrSolverBuilder::new());
            crossword.solve_with_solver_built_by(solver_builder)
        }
        unknown_solver => panic!("Unknown solver: {unknown_solver}"),
    }
}

fn iterate_and_print(count: usize, solutions: &mut CrosswordSolutions) {
    for number in 1..=count {
        let solution = solutions.next();
        match solution {
            None => {
                if number == 1 {
                    println!("No solution found.")
                } else {
                    println!("No more solution.")
                }
            }
            Some(grid) => {
                if number > 1 {
                    println!();
                }
                println!("{}", grid);
            }
        }
    }
}
