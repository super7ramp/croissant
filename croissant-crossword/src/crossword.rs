use std::ops::DerefMut;

use croissant_solver::SolverBuilder;
use croissant_solver::{ConfigurableSolver, Solver, SolverConfigurator};

use crate::constraints::Constraints;
use crate::grid::Grid;
use crate::variables::Variables;

///
/// # A crossword, defined as a boolean satisfiability problem
///
/// It is a basic definition of the problem, without any optimization attempt. As such, it is quite
/// slow to [solve]. The problem definition follows.
///
/// ## Variables
///
/// - Cell variables: For each pair (cell,letter) is associated a variable.
/// - Slot variables: For each pair (slot,word) is associated a variable. They are placed "after"
///   the cell variables in the model.
///
/// ## Constraints
///
/// 1. Each cell must contain one and only one letter from the alphabet or a block.
/// 2. Each slot must contain one and only one word from the input word list. This is the tricky
///    part, as there must be a correspondence between cell variables and slot variables. Basically,
///    each slot variable - i.e. a representation of a (slot,word) pair - is equivalent to a
///    conjunction (= and) of cell variables - i.e. (cell,letter) pairs.
/// 3. Prefilled cells must be kept as is.
///
/// ## See Also
///
/// - [Martin Hořeňovský's introduction to SAT solvers](https://codingnest.com/modern-sat-solvers-fast-neat-underused-part-1-of-n/). It very clearly explains the basics with the
///   example of the sudoku problem. Associated code is in C++.
/// - [Sudoku4j](https://gitlab.com/super7ramp/sudoku4j), which is an example sudoku solver in Java.
///   (It is a translation in Java of Martin Hořeňovský's example sudoku C++ solver.)
/// - [Croiseur's crossword solver backed by Sat4j](https://gitlab.com/super7ramp/croiseur/-/tree/master/croiseur-solver/croiseur-solver-sat),
///   which is the original implementation in Java of this program.
pub struct Crossword<'wordlist> {
    variables: Variables,
    constraints: Constraints<'wordlist>,
}

impl<'wordlist> Crossword<'wordlist> {
    /// Creates a new crossword from given grid and word list.
    ///
    /// ## Arguments
    ///
    /// - `input_grid`: A string representing the grid rows. '.' indicates a blank cell, '#' indicates a block.
    /// - `words`: The word list. Must contain words with only letters from 'A' to 'Z'. Other words will be rejected.
    ///
    /// ## Returns
    ///
    /// A [Result] with the created Crossword, or a String containing the error details.
    ///
    /// ## Example
    ///
    /// ```
    /// use croissant_crossword::crossword::Crossword;
    ///
    /// let words = vec!["AAA".to_string()];
    /// let result: Result<Crossword, String> = Crossword::try_from("A..\n.#.\n...", &words);
    /// ```
    pub fn try_from(input_grid: &str, words: &'wordlist Vec<String>) -> Result<Self, String> {
        let grid_creation = Grid::from(input_grid);
        if grid_creation.is_err() {
            return Err(grid_creation.err().unwrap());
        }

        let grid = grid_creation.unwrap();
        let variables = Variables::new(grid.clone(), words.len());
        let constraints = Constraints::new(grid, variables.clone(), words);

        Ok(Crossword {
            variables,
            constraints,
        })
    }

    /// Solves this problem with the solver built using given [SolverBuilder]. Note that solution may not be actually
    /// computed when this function returns: It may be created as late as when calling the created
    /// [CrosswordSolutions::next].
    pub fn solve_with_solver_built_by(
        self,
        mut solver_builder: Box<dyn SolverBuilder>,
    ) -> CrosswordSolutions {
        self.add_clauses_to(solver_builder.deref_mut());
        let solver = solver_builder.build();
        CrosswordSolutions::new(self.variables, solver)
    }

    /// Solves this problem with given [ConfigurableSolver]. Note that solution may not be actually computed when this
    /// function returns: It may be created as late as when calling the created [CrosswordSolutions::next].
    pub fn solve_with(
        self,
        mut solver: Box<dyn ConfigurableSolver<Item = Vec<i32>>>,
    ) -> CrosswordSolutions {
        self.add_clauses_to(solver.deref_mut());
        CrosswordSolutions::new(self.variables, solver)
    }

    /// Adds clauses to the given solver configurator.
    fn add_clauses_to(&self, solver_configurator: &mut dyn SolverConfigurator) {
        solver_configurator.allocate_variables(self.variables.count());
        solver_configurator.set_relevant_variables(self.variables.cells());
        self.constraints
            .add_one_letter_or_block_per_cell_clauses_to(solver_configurator);
        self.constraints
            .add_one_word_per_slot_clauses_to(solver_configurator);
        self.constraints
            .add_input_grid_constraints_are_satisfied_clauses_to(solver_configurator);
    }
}

/// An iterator over crossword solutions.
pub struct CrosswordSolutions {
    variables: Variables,
    solver: Box<dyn Solver<Item = Vec<i32>>>,
}

impl CrosswordSolutions {
    fn new(variables: Variables, solver: Box<dyn Solver<Item = Vec<i32>>>) -> Self {
        CrosswordSolutions { variables, solver }
    }
}

impl Iterator for CrosswordSolutions {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.solver
            .next()
            .map(move |solution| self.variables.back_to_domain(&solution))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct StubSolverBuilder {}
    impl SolverConfigurator for StubSolverBuilder {
        fn add_clause(&mut self, _literals: &[i32]) { /* Do nothing */
        }
    }
    impl SolverBuilder for StubSolverBuilder {
        fn build(&self) -> Box<dyn Solver<Item = Vec<i32>>> {
            Box::new(StubSolver {})
        }
    }

    struct StubSolver {}
    impl Solver for StubSolver {}
    impl Iterator for StubSolver {
        type Item = Vec<i32>;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }
    impl ConfigurableSolver for StubSolver {}
    impl SolverConfigurator for StubSolver {
        fn add_clause(&mut self, _literals: &[i32]) { /* Do nothing. */
        }
    }

    #[test]
    fn new_ok() {
        let words: Vec<String> = ["ABC", "DEF", "AA", "BB", "CC"]
            .iter()
            .map(|&word| word.to_string())
            .collect();
        let crossword = Crossword::try_from("...\n...", &words);
        assert!(crossword.is_ok(), "Creation failed");
    }

    #[test]
    fn new_err() {
        let words = ["ABC", "DEF", "AA", "BB", "CC"]
            .iter()
            .map(|&word| word.to_string())
            .collect();
        let crossword = Crossword::try_from("___" /* invalid grid */, &words);
        assert!(
            crossword.is_err(),
            "Creation succeeded, while it should have failed"
        );
    }

    #[test]
    fn solve_with() {
        let words = ["ABC", "DEF", "AA", "BB", "CC"]
            .iter()
            .map(|&word| word.to_string())
            .collect();
        let crossword = Crossword::try_from("...\n...", &words).unwrap();
        let stub_solver = Box::new(StubSolver {});

        let mut solutions = crossword.solve_with(stub_solver);
        assert_eq!(None, solutions.next())
    }

    #[test]
    fn solve_with_builder() {
        let words = ["ABC", "DEF", "AA", "BB", "CC"]
            .iter()
            .map(|&word| word.to_string())
            .collect();
        let crossword = Crossword::try_from("...\n...", &words).unwrap();
        let stub_solver_builder = Box::new(StubSolverBuilder {});

        let mut solutions = crossword.solve_with_solver_built_by(stub_solver_builder);
        assert_eq!(None, solutions.next())
    }
}
