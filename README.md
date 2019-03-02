# wasm-no-drop

When compiling Rust for WASM, if the code panics, drop() isn't called. This repo is a repr for that behavior.

## Running

### Not WASM

The code is in [`main.rs`](not-wasm/src/main.rs)

Just cd to `not-wasm` and run `cargo run`. It print out "panicked at 'panic'", and then the line after should say "dropping".

### WASM

The code is in [`lib.rs`](wasm/crate/src/lib.rs).

cd to `wasm` and run `npm i; npm run dev`. Then go to http://localhost:8080 and once it all compiles, you should see one
alert box that says "no_panic", but not one that says "panic".
