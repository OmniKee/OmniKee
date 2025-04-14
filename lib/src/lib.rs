use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Default)]
pub struct AppState {
    counter: i32,
}

#[wasm_bindgen]
impl AppState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn greet(&self, name: &str) -> String {
        format!("Hello, {}! the counter is {}", name, self.counter)
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }
}
