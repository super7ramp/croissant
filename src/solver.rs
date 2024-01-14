/// A SAT solver.
///
/// It is an iterator over the models satisfying the problem. A model is the vector of the problem's variable states.
/// A state is a positive integer if variable is true, or a negative integer if variable is false.
///
/// The solver is instructed the problem using [SolverBuilder]'s add functions, and finally built
/// using [SolverBuilder::build].
pub trait Solver : Iterator<Item=Vec<i32>> {
    // Nothing more than an iterator on the solutions for now.
}

/// Definition of a SAT solver builder.
///
/// The main function to implement is [add_clause]. Other functions contain default implementations
/// which may be overridden for better performances.
pub trait SolverBuilder {

    /// Adds the given literals as an *at-least-one* clause, i.e. a disjunction (= or).
    fn add_clause(&mut self, literals: &Vec<i32>);

    /// Adds the given literals as an *exactly-one* clause.
    ///
    /// An *exactly-one* clause is equivalent to an *at-least-one* and a *at-most-one* clauses. An *at-most-one*
    /// clause is equivalent to an *at-least-n-minus-one* of the negated literals, which essentially is an
    /// *at-least-one* clause.
    ///
    /// Default implementation creates these corresponding clauses and add them using [add_clause].
    /// Implementors may override this function for better performance.
    fn add_exactly_one(&mut self, literals: &Vec<i32>) {
        todo!()
    }

    /// Adds clauses describing the equivalence between the given literal and the given conjunction
    /// (= and) of literals, i.e.: *literal ⇔ conjunction\[0\] ∧ conjunction\[1\] ∧ ... ∧ conjunction\[n\]*
    ///
    /// The corresponding clauses are: *(￢literal ∨ conjunction\[0\]) ∧
    /// (￢literal ∨ conjunction\[1\]) ∧ ... ∧ (￢literal ∨ conjunction\[1\]) ∧ (￢conjunction\[0\]
    /// ∨ ￢conjunction\[1\] ∨ ... ∨ ￢conjunction\[n\] ∨ literal)*
    ///
    /// Default implementation adds these corresponding clauses using [add_clause]. Implementors
    /// may override this function for better performance.
    fn add_and(&mut self, literal: i32, conjunction: &Vec<i32>) {
        let mut last_clause = Vec::with_capacity(conjunction.len() + 1);
        for &conjunction_literal in conjunction {
            self.add_clause(&vec![-literal, conjunction_literal]);
            last_clause.push(-conjunction_literal);
        }
        last_clause.push(literal);
        self.add_clause(&last_clause);
    }

    /// Builds the solver.
    fn build(self) -> Box<dyn Solver<Item=Vec<i32>>>;
}

/// Tests for default [SolverBuilder] function implementations.
#[cfg(test)]
mod test {
    use super::*;

    struct TestSolverBuilder {
        clauses: Vec<Vec<i32>>,
    }

    impl SolverBuilder for TestSolverBuilder {
        fn add_clause(&mut self, literals: &Vec<i32>) {
            self.clauses.push(literals.to_vec())
        }

        fn build(self) -> Box<dyn Solver<Item=Vec<i32>>> {
            unimplemented!()
        }
    }

    #[test]
    fn add_and() {
        let mut solver_builder = TestSolverBuilder { clauses: vec![] };
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
