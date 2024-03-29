use splr::solver::SolverIter;
use splr::Config;

use croissant_solver::{Solver, SolverBuilder, SolverConfigurator};

/// Implementation of [SolverBuilder].
// TODO don't use splr's SolverIter (there is now way to specify the relevant variables, so solutions may be repeated
// TODO implement ConfigurableSolver instead
pub struct SplrSolverBuilder {
    clauses: Vec<Vec<i32>>,
}

impl Default for SplrSolverBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SplrSolverBuilder {
    pub fn new() -> Self {
        SplrSolverBuilder {
            clauses: Vec::new(),
        }
    }
}

impl SolverConfigurator for SplrSolverBuilder {
    fn add_clause(&mut self, literals: &[i32]) {
        self.clauses.push(literals.to_vec())
    }
}

impl SolverBuilder for SplrSolverBuilder {
    fn build(&self) -> Box<dyn Solver<Item = Vec<i32>>> {
        Box::new(SplrSolverWrapper::new(&self.clauses))
    }
}

/// Implementation of [Solver] wrapping the splr SAT solver.
struct SplrSolverWrapper {
    iter: SolverIter,
}

impl SplrSolverWrapper {
    fn new(clauses: &Vec<Vec<i32>>) -> Self {
        let iter = splr::Solver::try_from((Config::default(), clauses.as_slice()))
            .map(splr::solver::Solver::into_iter)
            .unwrap(); // TODO error handling
        SplrSolverWrapper { iter }
    }
}

impl Iterator for SplrSolverWrapper {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Solver for SplrSolverWrapper {
    // Nothing to do.
}
