use splr::solver::SolverIter;
use splr::Config;

use croissant_solver::solver::{Solver, SolverBuilder};

/// Implementation of [SolverBuilder]. Since splr doesn't provide any utilities to create clauses,
/// just use default [SolverBuilder] implementations and store clauses in a vector.
struct SplrSolverBuilder {
    clauses: Vec<Vec<i32>>,
}

impl SplrSolverBuilder {
    fn new() -> Self {
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
struct SplrSolverWrapper<'solving> {
    solver: splr::Solver,
    iter: Option<SolverIter<'solving>>,
}

impl SplrSolverWrapper<'_> {
    fn new(clauses: &Vec<Vec<i32>>) -> Self {
        // TODO error handling
        let solver = splr::Solver::try_from((Config::default(), clauses.as_slice())).unwrap();
        SplrSolverWrapper { solver, iter: None }
    }
}

impl Iterator for SplrSolverWrapper<'_> {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        // FIXME lifetime
        //  3 |     fn next(&mut self) -> Option<Self::Item> {
        //    |             ---------
        //    |             |
        //    |             let's call the lifetime of this reference `'1`
        //    |             has type `&mut SplrSolverWrapper<'2>`
        //  ...
        //  53 |         self.iter.get_or_insert_with(|| self.solver.iter()).next()
        //     |                                         ^^^^^^^^^^^^^^^^^^ method was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
        self.iter.get_or_insert_with(|| self.solver.iter()).next()
    }
}

impl Solver for SplrSolverWrapper<'_> {
    // Nothing to do.
}

#[cfg(test)]
mod test {
    use croissant_crossword::crossword::Crossword;

    use super::*;

    #[test]
    fn crossword_simple_empty_word_list() {
        let words = vec![];
        let crossword = Crossword::from("...\n...\n...", &words).unwrap();
        let solver_builder = Box::new(SplrSolverBuilder::new());
        let mut solutions = crossword.solve_with(solver_builder);

        assert_eq!(None, solutions.next())
    }
}
