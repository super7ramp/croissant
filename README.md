## Croissant

This is a crossword solver backed by various SAT solvers.

This is *slow*.

This is unusable.

*This is a toy project.*

### Play

First, install [Rust](https://rustup.rs/).

Then try this out:

```
cargo run "\
....
..#.
A..."
```

It should return:

```
CHIZ
HE#O
ASIA
```

Good job! You just created a crossword. Now you can read the help page:

```
🥐 Welcome to Croissant, a crossword solver that smells good

Usage: croissant-cli [OPTIONS] <GRID>

Arguments:
  <GRID>
          The grid as a string; Each new line is a new row, '.' is a blank, '#' is a block

Options:
  -w, --wordlist <WORDLIST>
          The path to the word list; File must contain one word per line and nothing else

  -s, --solver <SOLVER>
          The solver to use
          
          [default: logicng]

          Possible values:
          - cadical: The slow; Its name sounds good though, doesn't it?
          - logicng: The less slow and thus the default; Congrats!
          - splr:    The slowest and buggiest, but that's why we love it ❤️

  -c, --count <COUNT>
          The desired number of solutions
          
          [default: 1]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Enjoy!

### Goals

- ✅ Have fun!
- ✅ Explore SAT solvers available in Rust:
    - ✅ [Splr](https://crates.io/crates/Splr)
    - ✅ [LogicNG](https://crates.io/crates/Logicng)
    - ✅ [CaDiCaL](https://crates.io/crates/Cadical)
- ✅ Implement a CLI using [clap](https://crates.io/crates/clap).
- ✅ Understand Cargo feature configuration: Put each bundled solver behind a feature flag.
- 🚧 Discover WebAssembly: Compile Croissant to wasm and call it from a simple web application.
- 🚧 Scratch dynamic loading in Rust: Discover and use solvers compiled as shared libraries.
- 🚧 Document and publish a crate.

### Other Projects

If you're looking for a fast crossword solver in Rust, check out [xwords-rs](https://github.com/szunami/xwords-rs).

If you're looking for another SAT-based crossword solver, check out:

- [Croiseur](https://github.com/super7ramp/croiseur)
  and its [SAT solver plugin](https://github.com/super7ramp/croiseur/tree/master/croiseur-solver/croiseur-solver-sat).
  It's written in Java.
- [crogo](https://github.com/super7ramp/crogo): It's written in Go.
