use splr::solver::SolverIter;
use splr::Config;

use croissant_solver::solver::{Solver, SolverBuilder};

/// Implementation of [SolverBuilder]. Since splr doesn't provide any utilities to create clauses,
/// just use default [SolverBuilder] implementations and store clauses in a vector.
pub struct SplrSolverBuilder {
    clauses: Vec<Vec<i32>>,
}

impl SplrSolverBuilder {
    pub fn new() -> Self {
        SplrSolverBuilder { clauses: vec![] }
    }
}

impl SolverBuilder for SplrSolverBuilder {
    fn add_clause(&mut self, literals: &Vec<i32>) {
        self.clauses.push(literals.to_vec())
    }
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
        // FIXME clauses are copied twice, that's inefficient; it would be better if we could move builder into solver
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
