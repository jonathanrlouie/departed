# departed
Departed is a library for Rust programmers that provides some of the benefits of dependent types using the ["Ghosts of Departed Proofs" technique](https://kataskeue.com/gdp.pdf). Namely, it allows library authors to write APIs with statically checked preconditions and invariants. In addition, it allows users of said APIs to prove that they are using the APIs correctly.

All credit for the technique this crate is based on goes to Matt Noonan.
