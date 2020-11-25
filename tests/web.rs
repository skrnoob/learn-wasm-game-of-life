//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[cfg(test)]
pub fn input_render() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(3);
    universe.set_height(3);
    universe.set_cells(&[(0,0),(1,1),(2,2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_render() {
    let input_universe = input_render();
    let expected_output = String::from("◼◻◻\n◻◼◻\n◻◻◼\n");

    assert_eq!(input_universe.render(),expected_output);
}

#[wasm_bindgen_test] 
pub fn test_toggle_cell() {
    let mut input_universe = input_render();
    input_universe.toggle_cell(0, 0);
    let expected_output = String::from("◻◻◻\n◻◼◻\n◻◻◼\n");

    assert_eq!(input_universe.render(),expected_output);

}

#[wasm_bindgen_test]
pub fn test_tick() {
    let input_universe = input_spaceship();
    let expected_universe = expected_spaceship();

    assert_eq!(&input_universe.render(), &input_universe.render());
}
