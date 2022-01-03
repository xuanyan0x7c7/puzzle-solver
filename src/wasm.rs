use wasm_bindgen::prelude::*;

use crate::PuzzleSolver as Solver;

#[wasm_bindgen]
pub struct PuzzleSolver {
    solver: Solver,
}

#[wasm_bindgen]
impl PuzzleSolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        PuzzleSolver {
            solver: Solver::new(),
        }
    }

    #[wasm_bindgen(js_name = newConditionalConstraint)]
    pub fn new_conditional_constraint(&mut self, holes: usize) -> usize {
        self.solver.new_conditional_constraint(holes)
    }

    #[wasm_bindgen(js_name = addRows)]
    pub fn add_rows(&mut self, row_count: usize) -> usize {
        self.solver.add_rows(row_count)
    }

    #[wasm_bindgen(js_name = addColumn)]
    pub fn add_column(&mut self, rows: &JsValue) {
        let transformed_rows = rows.into_serde::<Vec<usize>>().unwrap();
        self.solver.add_column(transformed_rows.into_iter());
    }

    #[wasm_bindgen(js_name = addConditionalColumn)]
    pub fn add_conditional_column(&mut self, rows: &JsValue, conditional_index: usize) {
        let transformed_rows = rows.into_serde::<Vec<usize>>().unwrap();
        self.solver
            .add_conditional_column(transformed_rows.into_iter(), conditional_index);
    }

    #[wasm_bindgen(js_name = addConstraint)]
    pub fn add_constraint(&mut self, rows: &JsValue) {
        let transformed_rows = rows.into_serde::<Vec<usize>>().unwrap();
        self.solver.add_constraint(transformed_rows.into_iter());
    }

    #[wasm_bindgen(js_name = selectRow)]
    pub fn select_row(&mut self, row: usize) {
        self.solver.select_row(row);
    }

    #[wasm_bindgen(js_name = deselectRow)]
    pub fn deselect_row(&mut self, row: usize) {
        self.solver.deselect_row(row);
    }

    #[wasm_bindgen(js_name = solveNext)]
    pub fn solve_next(&mut self) -> JsValue {
        match self.solver.solve_next() {
            Some(solution) => JsValue::from_serde(&solution).unwrap(),
            _ => JsValue::NULL,
        }
    }
}
