//! This library defines the interface of a SAT solver. It is meant to be consumed by
//! [croissant-crossword](https://crates.io/crates/croissant-crossword/).

/// Definition of a SAT solver.
///
/// It is an iterator over the models satisfying the problem. A model is a vector indexed by the variables, whose values
/// indicate the state of the corresponding variable. A positive value indicates that the corresponding variable is
/// true; a negative value indicates that the corresponding variable is false.
///
/// Implementation *may* return only the state of relevant variables defined by [SolverConfigurator::set_relevant_variables]
/// instead of all the variables of the problems.
///
/// A solver can either be mutable - a [ConfigurableSolver] - or immutable and built using a [SolverBuilder]. Implement
/// one of these two traits, at your convenience: Both can be used by
/// [croissant-crossword](https://crates.io/crates/croissant-crossword/).
pub trait Solver: Iterator<Item = Vec<i32>> {
    // Nothing more than an iterator on the solutions for now.
}

/// Definition of a solver configurator.
///
/// The main function to implement is [add_clause](Self::add_clause). Other functions contain default implementations
/// which may be overridden for better performances.
pub trait SolverConfigurator {
    /// Gives a hint about the number of variables. May be implemented to optimize performance.
    ///
    /// Default implementation does nothing.
    fn allocate_variables(&mut self, _variables_count: usize) {
        // Do nothing by default.
    }

    /// Indicates which variables are relevant for the problem.
    /// It is a hint for the solver, that can help implementation to avoid duplica
    fn set_relevant_variables(&mut self, _relevant_variables: Vec<usize>) {
        // Do nothing by default.
    }

    /// Adds the given literals as an *at-least-one* clause, i.e. a disjunction (= or).
    fn add_clause(&mut self, literals: &[i32]);

    /// Adds the given literals as an *exactly-one* clause.
    ///
    /// An *exactly-one* clause is equivalent to an *at-least-one* and a *at-most-one* clauses.
    ///
    /// Default implementation creates these corresponding clauses and add them using [add_clause](Self::add_clause) and
    /// [add_at_most_one](Self::add_at_most_one). Implementors may override this function for better performances.
    fn add_exactly_one(&mut self, literals: &[i32]) {
        self.add_clause(literals);
        self.add_at_most_one(literals);
    }

    /// Adds the given literals as an *at-most-one* clause.
    ///
    /// An *at-most-one* clause is equivalent to saying there is no pair of literals for which both literals are true.
    /// This is equivalent to saying that for all pairs of literals, *at-least-one* is false. In other words, an
    /// *at-most-one* clause is equivalent to all the *at-least-one* clauses for each pair of negated literals.
    ///
    /// Default implementation creates these corresponding clauses and add them using [add_clause](Self::add_clause).
    /// Implementors may override this function for better performances
    fn add_at_most_one(&mut self, literals: &[i32]) {
        for i in 0..literals.len() {
            for j in (i + 1)..literals.len() {
                self.add_clause(&[-literals[i], -literals[j]]);
            }
        }
    }

    /// Adds clauses describing the equivalence between the given literal and the given conjunction
    /// (= and) of literals, i.e.: *literal ⇔ conjunction\[0\] ∧ conjunction\[1\] ∧ ... ∧ conjunction\[n\]*
    ///
    /// The corresponding clauses are: *(￢literal ∨ conjunction\[0\]) ∧
    /// (￢literal ∨ conjunction\[1\]) ∧ ... ∧ (￢literal ∨ conjunction\[1\]) ∧ (￢conjunction\[0\]
    /// ∨ ￢conjunction\[1\] ∨ ... ∨ ￢conjunction\[n\] ∨ literal)*
    ///
    /// Default implementation adds these corresponding clauses using [add_clause](Self::add_clause). Implementors
    /// may override this function for better performance.
    fn add_and(&mut self, literal: i32, conjunction: &[i32]) {
        let mut last_clause = Vec::with_capacity(conjunction.len() + 1);
        for &conjunction_literal in conjunction {
            self.add_clause(&[-literal, conjunction_literal]);
            last_clause.push(-conjunction_literal);
        }
        last_clause.push(literal);
        self.add_clause(&last_clause);
    }
}

/// Definition of a configurable [Solver].
///
/// A mutable solver if you will. Implement this, unless you're adventurous and you want to try implementing
/// [SolverBuilder] instead.
pub trait ConfigurableSolver: SolverConfigurator + Solver {
    // Marker trait.
}

/// Definition of a [Solver] builder.
///
/// Implement this if you can and want to efficiently register and share clauses between solver instances. If not,
/// then implementing this trait will probably lead to a costly copy of the clauses between the builder and the solver
/// upon call to the build function, and you'd better implement [ConfigurableSolver] instead.
pub trait SolverBuilder: SolverConfigurator {
    /// Builds the solver.
    fn build(&self) -> Box<dyn Solver<Item = Vec<i32>>>;
}

/// Tests for default [SolverConfigurator] function implementations.
#[cfg(test)]
mod test {
    use super::*;

    struct TestSolverConfigurator {
        clauses: Vec<Vec<i32>>,
    }

    impl SolverConfigurator for TestSolverConfigurator {
        fn add_clause(&mut self, literals: &[i32]) {
            self.clauses.push(literals.to_vec())
        }
    }

    #[test]
    fn add_exactly_one() {
        let mut solver_builder = TestSolverConfigurator { clauses: vec![] };
        let literals = vec![1, 2, 3];

        solver_builder.add_exactly_one(&literals);

        assert_eq!(
            vec![vec![1, 2, 3], vec![-1, -2], vec![-1, -3], vec![-2, -3]],
            solver_builder.clauses
        );
    }

    #[test]
    fn add_at_most_one() {
        let mut solver_builder = TestSolverConfigurator { clauses: vec![] };
        let literals = vec![1, 2, 3];

        solver_builder.add_at_most_one(&literals);

        assert_eq!(
            vec![vec![-1, -2], vec![-1, -3], vec![-2, -3],],
            solver_builder.clauses
        );
    }

    #[test]
    fn add_and() {
        let mut solver_builder = TestSolverConfigurator { clauses: vec![] };
        let conjunction = vec![-1, 6, -7];

        // 42 ⇔ -1 ∧ 6 ∧ -7
        solver_builder.add_and(42, &conjunction);

        // (-42 ∨ -1) ∧ (-42 ∨ 6) ∧ (-42 ∨ -7) ∧ (1 ∨ -6 ∨ 7 ∨ 42)
        assert_eq!(
            vec![
                vec![-42, -1],
                vec![-42, 6],
                vec![-42, -7],
                vec![1, -6, 7, 42],
            ],
            solver_builder.clauses
        );
    }
}
