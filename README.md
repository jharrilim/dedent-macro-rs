# dedent-rs

This crate provides a simple compile-time macro to dedent strings.

```rust
let s = dedent!("
  foo
  bar
");
assert_eq!(s, "foo\nbar");
```
