# Tests

## Test everything

Remember to test both asynchronous and synchronous versions of the code

```bash
cargo test --no-default-features --features=sync
cargo test --no-default-features --features=tokio
```

## Tokio tests

Synca allows you to keep one code base for both synchronous and 
asynchronous versions of the code.

[Tokio](https://tokio.rs/) tests example.

```rust
#[cfg(test)]
#[synca::synca(
  feature = "tokio",
  #[tokio::test] => #[test]
)]
mod tests {
  #[tokio::test]
  pub async fn my_test() {

  }
}
```

Generated code

```rust
#[cfg(test)]
#[cfg(not(feature = "tokio"))]
mod tests {
  #[test]
  pub fn my_test() {

  }
}

#[cfg(test)]
#[cfg(feature = "tokio")]
mod tests {
  #[tokio::test]
  pub async fn my_test() {

  }
}
```