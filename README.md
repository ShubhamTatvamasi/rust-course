# rust-course

https://play.rust-lang.org

Create a new project
```bash
cargo new hello_world --bin
```
> We’re passing `--bin` because we’re making a binary program: if we were making a library, we’d pass `--lib`.

Compile your application
```bash
cargo build
```
> puts the resulting binary in `target/debug`

Compile and run it
```bash
cargo run
```

Release your files with optimizations turned on:
```bash
cargo build --release
```
> puts the resulting binary in `target/release`

For testing your app run:
```bash
cargo test
```
---

Use `rustc` command to just compile the file
```bash
rustc main.rs
```
