# dbg_mac
`dbg_mac` is a Rust crate that provides macros that are only compiled in if debug symbols are
present.

## Present Macros

Below are examples of every macro implemented in this crate:

```rust
// std::unimplemented, but only if built with debug assertions.
dbg_unimplemented!();

// std::unreachable, but only if built with debug assertions.
dbg_unreachable!();

// std::todo, but only if built with debug assertions.
dbg_todo!();

// std::panic, but only if built with debug assertions.
dbg_panic!();

// std::compile_error, if built with debug assertions.
dbg_compile_error!();

// Will print "Hello, debug!", if built with debug assertions.
if_dbg!(println!("Hello, debug!"));
```

## Authors
Milo Banks
