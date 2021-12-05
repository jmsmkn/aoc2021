use wasm_bindgen::prelude::*;

mod day01;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn solve(depths: Vec<isize>) {
    alert(&format!("The solution is {}", day01::solve(depths)));
}
