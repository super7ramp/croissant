use croissant_solver::{ConfigurableSolver, Solver, SolverConfigurator};

/// Implementation of [ConfigurableSolver].
pub struct CadicalSolver {
    /// The actual solver.
    solver: cadical::Solver,
    /// The problem's relevant variables.
    relevant_variables: Vec<usize>,
    /// The last solution found, if any, or an empty vector.
    last_solution: Vec<i32>,
    /// Whether there is no solution left.
    no_more_solution: bool,
}

impl Default for CadicalSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl CadicalSolver {
    /// Creates an instance.
    pub fn new() -> Self {
        let solver = cadical::Solver::default();
        CadicalSolver {
            solver,
            relevant_variables: Vec::new(),
            last_solution: Vec::new(),
            no_more_solution: false,
        }
    }

    /// Refutes the last solution found, if any. Otherwise, does nothing.
    fn refute_last_solution(&mut self) {
        if self.last_solution.is_empty() {
            return;
        }
        let not_last_solution: Vec<i32> = self
            .last_solution
            .iter()
            .enumerate()
            .map(|(variable, value)| -value.signum() * ((variable + 1) as i32))
            .collect();
        self.solver.add_clause(not_last_solution);
    }

    /// Returns the relevant variable with biggest id.
    fn max_relevant_variable(&self) -> usize {
        *self.relevant_variables.iter().max().unwrap_or(&0)
    }

    /// Returns the model, i.e. the state of the variables in the solver.
    fn model(&mut self) -> Vec<i32> {
        let variables_count = self.max_relevant_variable();
        let mut model = Vec::with_capacity(variables_count);
        for variable in 1..(variables_count + 1) {
            let variable_state = self
                .solver
                .value(variable as i32)
                .map(|pos| if pos { 1 } else { -1 })
                .unwrap_or_default();
            model.push(variable_state);
        }
        model
    }
}

impl SolverConfigurator for CadicalSolver {
    fn set_relevant_variables(&mut self, relevant_variables: Vec<usize>) {
        self.relevant_variables = relevant_variables;
    }
    fn add_clause(&mut self, literals: &[i32]) {
        self.solver.add_clause(literals.to_vec());
    }
}

impl Iterator for CadicalSolver {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.no_more_solution {
            return None;
        }
        self.refute_last_solution();
        if self.solver.solve() != Some(true) {
            self.no_more_solution = true;
            return None;
        }
        let model = self.model();
        self.last_solution.clone_from(&model);
        Some(model)
    }
}

impl Solver for CadicalSolver {}
impl ConfigurableSolver for CadicalSolver {}
