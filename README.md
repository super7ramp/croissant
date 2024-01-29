## Croissant

This is a crossword solver backed by various SAT solvers.

This is *very* slow.

This is unusable.

*This is a toy project.*

### Goals

- ✅ Have fun!
- ✅ Explore SAT solvers available in Rust:
    - ✅ [Splr](https://crates.io/crates/Splr)
    - ✅ [LogicNG](https://crates.io/crates/Logicng)
    - ✅ [CaDiCaL](https://crates.io/crates/Cadical)
- 🚧 Implement a CLI using [clap](https://crates.io/crates/clap).
- 🚧 Understand Cargo feature configuration: Put each bundled solver behind a feature flag.
- 🚧 Scratch dynamic loading in Rust: Allow to use solvers loaded from shared libraries put in some folder.
- 🚧 Document and publish a crate.

### Other Projects

If you're looking for a fast crossword solver in Rust, check out [xwords-rs](https://github.com/szunami/xwords-rs).

If you're looking for another SAT-based crossword solver - slow, in Java, but still way faster than Croissant - check
out [Croiseur](https://github.com/super7ramp/croiseur) and
its [SAT solver plugin](https://github.com/super7ramp/croiseur/tree/master/croiseur-solver/croiseur-solver-sat).