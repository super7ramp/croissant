#![feature(trait_upcasting)]

//! # A crossword, defined as a boolean satisfiability problem
//!
//! This crate provides a library to transform a crossword problem – defined by a grid and a list of words –
//! into a boolean satisfiability problem. This boolean satisfiability problem can then be solved by any SAT solver
//! implementing the [croissant_solver::Solver] trait (not included in this crate).
//!
//! In simpler words: `croissant_crossword` + a `croissant_solver` implementation = a crossword solver.
//!
//! ## Example
//!
//! ```ignore
//! use croissant_crossword::crossword::Crossword;
//! use croissant_solver_logicng::LogicngSolverBuilder;
//!
//! // Crossword construction
//! let words = ["AAA".to_string()];
//! let crossword = Crossword::try_from("A..\n.#.\n...", &words).unwrap();
//!
//! // Solving using logicng solver (not included)
//! let solver_builder = Box::new(LogicngSolverBuilder::new());
//! let solutions = crossword.solve_with_solver_built_by(solver_builder);
//! for solution in solutions {
//!     println!("{solution}")
//! }
//! ```
//!
//! ## See Also
//!
//! - [Croissant CLI](https://crates.io/crates/croissant-cli): A command-line crossword solver that leverages
//!   `croissant_crossword`.

// API
pub mod crossword;

// Implementation
mod alphabet;
mod constraints;
mod grid;
mod pos;
mod slot;
mod variables;
