extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

struct A {
  msg: &'static str,
}

impl Drop for A {
  fn drop(&mut self) {
    web_sys::window().expect("window").alert_with_message("dropping").expect("alert");
  }
}

#[wasm_bindgen]
pub fn test_no_panic() {
  let _a = A {
    msg: "no_panic"
  };
}

#[wasm_bindgen]
pub fn test_with_panic() {
  let _a = A {
    msg: "panic"
  };
  panic!("panic");
}

