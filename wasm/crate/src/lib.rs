#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

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

