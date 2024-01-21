use croissant_solver::solver::{Solver, SolverBuilder};
use logicng::datastructures::Model;
use logicng::formulas::{CType, EncodedFormula, FormulaFactory, Literal, Variable};
use logicng::solver::minisat::MiniSat;
use std::rc::Rc;

/// Implementation of [SolverBuilder].
pub struct LogicngSolverBuilder {
    /// The helper to create/register boolean formulas.
    formula_factory: Rc<FormulaFactory>,
    /// The created formulas.
    formulas: Vec<EncodedFormula>,
    /// The number of variables.
    variables_count: usize,
}

impl LogicngSolverBuilder {
    /// Creates an instance.
    pub fn new() -> Self {
        let formula_factory = Rc::new(FormulaFactory::new());
        let formulas = Vec::new();
        LogicngSolverBuilder {
            formula_factory,
            formulas,
            variables_count: 0,
        }
    }

    /// Converts a raw literal to an [EncodedFormula].
    fn encoded_formula_from(&self, literal: i32) -> EncodedFormula {
        EncodedFormula::from(self.literal_from_raw(literal))
    }

    /// Converts a raw literal to a [Literal].
    fn literal_from_raw(&self, literal: i32) -> Literal {
        let variable_name = literal.abs().to_string();
        let literal_phase = literal > 0;
        self.formula_factory
            .lit(variable_name.as_str(), literal_phase)
    }
}

impl SolverBuilder for LogicngSolverBuilder {
    fn allocate_variables(&mut self, variables_count: usize) {
        self.variables_count = variables_count;
    }

    fn add_clause(&mut self, literals: &Vec<i32>) {
        let operands: Vec<EncodedFormula> = literals
            .iter()
            .map(|&literal| self.encoded_formula_from(literal))
            .collect();
        let or_formula = self.formula_factory.or(&operands.as_slice());
        self.formulas.push(or_formula);
    }

    // Overriding default implementation for performance.
    fn add_exactly_one(&mut self, literals: &Vec<i32>) {
        let lits: Vec<Literal> = literals
            .iter()
            .map(|&literal| self.literal_from_raw(literal))
            .collect();
        let formula = self
            .formula_factory
            .pbc(CType::EQ, 1, lits, vec![1; literals.len()]);
        self.formulas.push(formula);
    }

    // Overriding default implementation for performance.
    fn add_at_most_one(&mut self, literals: &Vec<i32>) {
        let lits: Vec<Literal> = literals
            .iter()
            .map(|&literal| self.literal_from_raw(literal))
            .collect();
        let formula = self
            .formula_factory
            .pbc(CType::LE, 1, lits, vec![1; literals.len()]);
        self.formulas.push(formula);
    }

    // Overriding default implementation for performance.
    fn add_and(&mut self, literal: i32, conjunction: &Vec<i32>) {
        let and_operands: Vec<EncodedFormula> = conjunction
            .iter()
            .map(|&literal| self.encoded_formula_from(literal))
            .collect();
        let right = self.formula_factory.and(&and_operands.as_slice());
        let left = self.encoded_formula_from(literal);
        let eq_formula = self.formula_factory.equivalence(left, right);
        self.formulas.push(eq_formula);
    }

    fn build(&self) -> Box<dyn Solver<Item = Vec<i32>>> {
        Box::new(LogicngSolver::new(
            &self.formulas,
            self.formula_factory.clone(),
            self.variables_count,
        ))
    }
}

/// Implementation of [Solver].
pub struct LogicngSolver {
    /// The actual solver.
    solver: MiniSat,
    /// The helper to retrieve registered formula names.
    formula_factory: Rc<FormulaFactory>,
    /// The number of variables (since it is hard to get the information from solver)
    variables_count: usize,
    /** Literals of the last solution. */
    last_solution_literals: Vec<Literal>,
    /// Whether all solutions have been found.
    no_more_solution: bool,
}

impl LogicngSolver {
    /// Creates an instance.
    fn new(
        formulas: &Vec<EncodedFormula>,
        formula_factory: Rc<FormulaFactory>,
        variables_count: usize,
    ) -> Self {
        let mut solver = MiniSat::new();
        solver.add_all(formulas, &formula_factory);
        LogicngSolver {
            solver,
            formula_factory,
            variables_count,
            last_solution_literals: Vec::new(),
            no_more_solution: false,
        }
    }

    /// Solves the problem. Returns Some [Model] satisfying the problem, or [None] if no solution found.
    fn solve(&mut self) -> Option<Model> {
        self.solver.sat();
        self.solver.model(None)
    }

    /// Refutes the last solution, i.e. don't propose the last solution again.
    /// Does nothing if no solution has been found yet.
    fn refute_previous_solution(&mut self) {
        if self.last_solution_literals.is_empty() {
            return;
        }
        let not_model: Vec<Literal> = self
            .last_solution_literals
            .iter()
            .map(Literal::negate)
            .collect();
        let dont_propose_this_solution_anymore = self.formula_factory.clause(not_model.as_slice());
        self.solver
            .add(dont_propose_this_solution_anymore, &self.formula_factory);
        self.last_solution_literals.clear();
    }

    /// Translates solver [Model] to a vector of variables states.
    fn variable_states_from(&self, model: Model) -> Vec<i32> {
        let mut literals = vec![0; self.variables_count];
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
        variable
            .name(&self.formula_factory)
            .parse::<usize>()
            .unwrap()
            - 1
    }
}

impl Iterator for LogicngSolver {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.no_more_solution {
            return None;
        }
        self.refute_previous_solution();

        let optional_model = self.solve();

        if optional_model.is_none() {
            self.no_more_solution = true;
            return None;
        }
        let model = optional_model.unwrap();
        self.last_solution_literals = model.literals();
        let solution = self.variable_states_from(model);
        Some(solution)
    }
}

impl Solver for LogicngSolver {}
