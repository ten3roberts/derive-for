# derive-for
A macro for defining multiple structs using the same derive procedures.

The most common use case is in combination with the [`derive_more`](https://crates.io/crates/derive_more) for using the
same derives to create transparent newtypes.

## Usage
```rust
derive_for!(
( Clone, Debug, PartialEq, Eq),
pub struct Foo{a: i32, name: String};
pub struct Bar(u32, u32);
);
```

`Clone`, `Debug`, `PartialEq`, and `Eq` will now be implemented for both
`Foo` and `Bar`. If deriving many traits for many newtypes this significantly
shortens the code.

## State
This crate is very small and feature complete. If there are any issues, feel
free to open an issue.
