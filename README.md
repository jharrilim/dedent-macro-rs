# dedent-macro

This crate provides a simple compile-time macro to dedent strings.
Useful for keeping consistent indentation within code.

Before:

```rust
fn write_win_status(x: bool) {
    let s = if x {
      "
Congratulations!
You won!
      "
    } else {
      "
Uh oh,
try again!
      "
    }
    println!("{s}");
}
```

after:

```rust
fn write_win_status(x: bool) {
    let s = if x {
      dedent!("
        Congratulations!
        You won!
      ")
    } else {
      dedent!("
        Uh oh,
        try again!
      ")
    }
    println!("{s}");
}

let s = 
assert_eq!(s, "foo\nbar");
```
