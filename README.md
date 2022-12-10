# dbg_mac
`dbg_mac` is a Rust crate that provides macros that are only compiled in if debug symbols are
present.

## Present Macros

1. `dbg_unimplemented!()`
2. `dbg_unreachable!()`
3. `dbg_todo!()`
4. `dbg_panic!()`
5. `dbg_compile_error!(expr|block)`
6. `if_dbg!`[^1]

[^1] Anything inside is only evaluated if debug symbols are present.

## Authors
Milo Banks
