use std::rc::Rc;
use logicng::datastructures::Model;
use croissant_solver::solver::{Solver, SolverBuilder};
use logicng::formulas::{EncodedFormula, FormulaFactory, Literal, Variable};
use logicng::solver::minisat::MiniSat;

/// Implementation of [SolverBuilder].
pub struct LogicngSolverBuilder {
    formula_factory: Rc<FormulaFactory>,
    formulas: Vec<EncodedFormula>,
}

impl LogicngSolverBuilder {
    pub fn new() -> Self {
        let formula_factory = Rc::new(FormulaFactory::new());
        let formulas = Vec::new();
        LogicngSolverBuilder { formula_factory, formulas }
    }
}

impl SolverBuilder for LogicngSolverBuilder {
    fn add_clause(&mut self, literals: &Vec<i32>) {
        let mut operands = Vec::with_capacity(literals.len());
        for &literal in literals {
            let variable_name = literal.abs().to_string();
            let literal_phase = literal > 0;
            let literal = self.formula_factory.literal(variable_name.as_str(), literal_phase);
            operands.push(literal);
        }
        let or_formula = self.formula_factory.or(&operands.as_slice());
        self.formulas.push(or_formula);
    }

    fn build(&self) -> Box<dyn Solver<Item = Vec<i32>>> {
        let mut solver = MiniSat::new();
        solver.add_all(&self.formulas, &self.formula_factory);
        Box::new(LogicngSolver::new(solver, self.formula_factory.clone()))
    }
}

pub struct LogicngSolver {
    solver: MiniSat,
    formula_factory: Rc<FormulaFactory>,
    no_more_solution: bool,
}

impl LogicngSolver {
    /// Creates an instance.
    fn new(solver: MiniSat, formula_factory: Rc<FormulaFactory>) -> Self {
        LogicngSolver { solver, formula_factory, no_more_solution: false }
    }

    /// Solves the problem. Returns Some [Model] satisfying the problem, or [None] if no solution found.
    fn solve(&mut self) -> Option<Model> {
        self.solver.sat();
        let model = self.solver.model(None);
        model
    }

    /// Refutes the given [Model], i.e. don't propose the given solution again.
    fn refute(&mut self, model: &Model) {
        let not_model: Vec<Literal> = model.literals().iter().map(Literal::negate).collect();
        let dont_propose_this_solution_anymore = self.formula_factory.clause(not_model.as_slice());
        self.solver.add(dont_propose_this_solution_anymore, &self.formula_factory);
    }

    /// Translates solver [Model] to a vector of variables states.
    fn variable_states_from(&self, model: Model) -> Vec<i32> {
        let mut literals = vec![0; model.len()];
        for positive_variable in model.pos() {
            literals[self.index_of(positive_variable)] = 1;
        }
        for negative_variable in model.neg() {
            literals[self.index_of(negative_variable)] = -1;
        }
        literals
    }

    /// Returns the index of the given [Variable].
    fn index_of(&self, variable: &Variable) -> usize {
        // Variable name stores the variable index as a string. See SolverBuilder's add_clause().
        variable.name(&self.formula_factory).parse::<usize>().unwrap() - 1
    }
}

impl Iterator for LogicngSolver {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.no_more_solution {
            return None;
        }

        let optional_model = self.solve();
        if optional_model.is_none() {
            self.no_more_solution = true;
            return None;
        }

        let model = optional_model.unwrap();
        self.refute(&model);

        let solution = self.variable_states_from(model);
        Some(solution)
    }
}

impl Solver for LogicngSolver {}
