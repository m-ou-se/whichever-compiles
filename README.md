# whichever-compiles

The `whichever_compiles!{}` macro `fork()`s the compiler to try out different
alternatives to find one that compiles.

https://twitter.com/m_ou_se/status/1368632701448818691

Please do not use this.

## Example

```rust
use whichever_compiles::whichever_compiles;

fn main() {
    whichever_compiles! {
        try { thisfunctiondoesntexist(); }
        try { invalid syntax 1 2 3 }
        try { println!("missing arg: {}"); }
        try { println!("hello {}", world()); }
        try { 1 + 2 }
    }
}

whichever_compiles! {
    try { }
    try { fn world() {} }
    try { fn world() -> &'static str { "world" } }
}
```

```
$ cargo run
hello world
```
