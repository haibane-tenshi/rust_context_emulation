# Emulation of contexts in Rust

> This project is intended only as a proof-of-concept and a playground for exploration.
> Use on your own risk.

To get started just locally clone the repo.
It is difficult to see how everything fits together, so I recommend to look into [examples](./examples/input) folder first.
It contains step-by-step introduction to all concepts.

(Also, examples use shorthand names like `input_01` or `gat_trait02`,
look at [Cargo.toml](./Cargo.toml) for those.)

### There are three preludes, which one I choose?

Start with `prelude_input`.
This is (presumably) the most robust approach.
It fully supports shared references.
Mutable references are only usable in concrete contexts, usage behind wildcard contexts is impossible.

`prelude_gat` features GAT-based approach.
It is as expressive as input-based one, but has subtle differences in how certain things are expressed
(notably late-bound lifetimes).

Use `prelude_hybrid` only if you want to experiment with mutable references behind wildcard contexts.
Still, it doesn't feature full support, in particular mixing shared and mutable references is impossible.
There are likely other limitations as well.

## License

This project is dual licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
