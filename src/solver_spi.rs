/// Definition of a SAT solver.
///
/// The main function to implement is [add_clause]. Other functions contain default implementations
/// which may be overridden for better performances.
pub trait Solver {
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
    /// ∨ ￢conjunction\[1\] ∨ ... ∨ ￢conjunction\[n\] ∨ y)*
    ///
    /// Default implementation adds these corresponding clauses using [add_clause]. Implementors
    /// may override this function for better performance.
    fn add_and(&mut self, literal: i32, conjunction: &Vec<i32>) {
        todo!()
    }
}
