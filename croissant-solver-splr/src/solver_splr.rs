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

#[cfg(test)]
mod test {
    use croissant_crossword::crossword::{Crossword, CrosswordSolutions};
    use std::collections::HashSet;

    use super::*;

    #[test]
    #[ignore = "fix me!"]
    fn empty() {
        let solutions = solve("", []);
        assert_solutions_eq([], solutions);
    }

    #[test]
    fn trivial() {
        let solutions = solve("...\n...\n...", ["AAA", "BBB", "CDE", "ABC", "ABD", "ABE"]);
        assert_solutions_eq(
            [
                "BBB\nBBB\nBBB",
                "ABC\nABD\nABE",
                "AAA\nBBB\nCDE",
                "AAA\nAAA\nAAA",
            ],
            solutions,
        );
    }

    #[test]
    fn partially_prefilled_1x3() {
        let solutions = solve("AB.", ["ABC"]);
        assert_solutions_eq(["ABC"], solutions);
    }

    #[test]
    fn partially_prefilled_3x3() {
        let solutions = solve("ABC\n...\n...", ["AAA", "BBB", "CDE", "ABC", "ABD", "ABE"]);
        assert_solutions_eq(["ABC\nABD\nABE"], solutions);
    }

    #[test]
    #[ignore = "fix me!"]
    fn with_blocks() {
        let solutions = solve("ABC\n..#\n#..", ["AA", "BBB", "ABC", "AB", "BE"]);
        assert_solutions_eq(["ABC\nAB#\n#BE"], solutions);
    }

    #[test]
    fn impossible_no_solution() {
        let solutions = solve(
            "ABC\n...\n...",
            [
                "AAA", "BBB", "CDF", /* should be CDE */
                "ABC", "ABD", "ABE",
            ],
        );
        assert_solutions_eq([], solutions);
    }

    #[test]
    #[ignore = "fix me!"]
    fn impossible_no_candidate() {
        let solutions = solve("...\n...\n...", []);
        assert_solutions_eq([], solutions);
    }

    /// Solves the given grid using the splr solver.
    fn solve<const N: usize>(grid: &str, words: [&str; N]) -> CrosswordSolutions {
        let words_vec = Vec::from(words);
        let crossword = Crossword::from(grid, &words_vec).unwrap();
        let solver = Box::new(SplrSolverBuilder::new());
        crossword.solve_with(solver)
    }

    /// Helper to verify that all solutions are present, in any order.
    fn assert_solutions_eq<const N: usize>(
        expected_solutions: [&str; N],
        mut actual_solutions: CrosswordSolutions,
    ) {
        let mut expected_solutions = HashSet::from(expected_solutions);
        while let Some(solution) = actual_solutions.next() {
            assert_eq!(
                true,
                expected_solutions.remove(solution.as_str()),
                "Unexpected solution: {solution:?}"
            );
        }
        assert_eq!(
            true,
            expected_solutions.is_empty(),
            "Missing solutions: {expected_solutions:?}"
        );
    }
}
