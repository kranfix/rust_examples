use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod test {
  use super::add;
  use wasm_bindgen_test::wasm_bindgen_test;

  wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

  #[wasm_bindgen_test]
  fn test_add() {
    assert!(add(1, 2) == 3);
  }
}
