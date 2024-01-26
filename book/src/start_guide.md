# Start guide

How to implement SyncA into your project, step by step.

## Install synca

```bash
cargo add synca
```

## Describes features

```toml
// Cargo.toml

[features]
default = [ "sync" ]
sync = [ ]
tokio = [ ]
```

## Prohibit enabling both features at the same time

```rust
#[cfg(all(feature = "sync", feature = "tokio"))]
compile_error!(r#"feature "sync" and feature "tokio" cannot be both enabled at the same time"#);
```

## Require that either sync or tokyo be selected

```rust
#[cfg(all(not(feature = "sync"), not(feature = "tokio")))]
compile_error!(r#"one of the "sync" and "tokio" features must be enabled"#);
```

## Write code

```rust
#[synca(feature = "tokio")]
mod my_mod {
}
```

## Tests

Don't forget to add CI scripts

```bash
cargo test --no-default-features --features=sync
cargo test --no-default-features --features=tokio
```

Add some tests

```rust
#[cfg(test)]
#[synca(
  feature = "tokio",
  #[tokio::test] => #[test],
)]
mod tests {
  #[tokio::test]
  async fn my_test() { }
}
```

## Enjoy

Everything is ready to write only the asynchronous version of the code, 
the synchronous one is generated by the SyncA.