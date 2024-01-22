use croissant_solver::solver::{Solver, SolverBuilder};

/// Implementation of [SolverBuilder].
pub struct CadicalSolverBuilder {
    clauses: Vec<Vec<i32>>,
}

impl CadicalSolverBuilder {
    pub fn new() -> Self {
        let clauses = Vec::new();
        CadicalSolverBuilder { clauses }
    }
}

impl SolverBuilder for CadicalSolverBuilder {
    fn add_clause(&mut self, literals: &Vec<i32>) {
        self.clauses.push(literals.to_vec());
    }

    fn build(&self) -> Box<dyn Solver<Item = Vec<i32>>> {
        Box::new(CadicalSolver::new(&self.clauses))
    }
}

/// Implementation of [Solver].
struct CadicalSolver {
    solver: cadical::Solver,
    last_solution: Vec<i32>,
    no_more_solution: bool,
}

impl CadicalSolver {
    fn new(clauses: &Vec<Vec<i32>>) -> Self {
        // FIXME clauses are copied twice, that's inefficient; it would be better if we could move builder into solver
        let mut solver = cadical::Solver::default();
        clauses
            .iter()
            .for_each(|clause| solver.add_clause(clause.clone()));
        CadicalSolver {
            solver,
            last_solution: Vec::new(),
            no_more_solution: false,
        }
    }

    fn refute_last_solution(&mut self) {
        if self.last_solution.is_empty() {
            return;
        }
        // FIXME that's not as trivial as that, same solutions are returned. I don't understand why it doesn't work
        //  nor why it "works" with logic-ng solver.
        //  Idea: Try to filter on positive literals of the relevant variables (i.e. crossword cell variables) which
        //  could be given as a hint in SolverBuilder.
        let not_last_solution: Vec<i32> = self.last_solution.iter().map(|lit| -lit).collect();
        self.solver.add_clause(not_last_solution);
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
        let variables_count = self.solver.max_variable() as usize;
        let mut model = Vec::with_capacity(variables_count);
        for variable in 1..(variables_count + 1) {
            let variable_state = self
                .solver
                .value(variable as i32)
                .map(|pos| if pos { 1 } else { -1 })
                .unwrap_or_default();
            model.push(variable_state);
        }
        self.last_solution.clone_from(&model);
        Some(model)
    }
}

impl Solver for CadicalSolver {}
