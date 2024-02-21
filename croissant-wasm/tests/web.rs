//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use croissant_wasm::solve;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_solve() {
    let grid = "....\n..#.\nA...".to_string();
    let solved_grid = solve(grid);
    assert_eq!(Some("CHIZ\nHE#O\nASIA".to_string()), solved_grid);
}
