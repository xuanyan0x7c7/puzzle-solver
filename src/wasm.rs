use crate::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PuzzleSolver {
    solver: DancingLinks,
}

#[wasm_bindgen]
impl PuzzleSolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        PuzzleSolver {
            solver: DancingLinks::new(),
        }
    }

    #[wasm_bindgen(js_name = addRows)]
    pub fn add_rows(&mut self, row_count: usize) {
        self.solver.add_rows(row_count as usize);
    }

    #[wasm_bindgen(js_name = addConstraint)]
    pub fn add_constraint(&mut self, rows: &JsValue, unique: bool) {
        self.solver
            .add_constraint(&rows.into_serde::<Vec<usize>>().unwrap(), unique);
    }

    #[wasm_bindgen(js_name = solveNext)]
    pub fn solve_next(&mut self) -> JsValue {
        match self.solver.solve_next() {
            Some(solution) => JsValue::from_serde(&solution).unwrap(),
            _ => JsValue::NULL,
        }
    }
}
