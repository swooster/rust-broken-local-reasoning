# Rust local reasoning failure

The body of `random_unrelated_code()` in `main.rs` affects whether `main()` compiles.

This works:
```bash
cargo run
```

This doesn't:
```bash
cargo run --features broken
```
