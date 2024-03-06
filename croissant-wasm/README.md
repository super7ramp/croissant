## croissant-wasm

A wasm module containing a small crossword solver, based on `croissant-crossword`.

### Build

You'll need [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/):

`cargo install wasm-pack`

You'll also need the wasm Rust target:

`rustup target add wasm32-unknown-unknown`

Then you can build the module using:

`wasm-pack build`