# Tests

## Test everything

Remember to test both asynchronous and synchronous versions of the code

```bash
cargo test --no-default-features --features=sync,tokio
```

## Tokio tests

Synca allows you to keep one code base for both synchronous and 
asynchronous versions of the code.

[Tokio](https://tokio.rs/) tests example.

```rust
#[synca::synca(
  #[cfg(test)]
  #[cfg(feature = "tokio")]
  pub mod tests_tokio { },
  #[cfg(test)]
  #[cfg(feature = "sync")]
  pub mod tests_sync { 
    sync!();
    replace!(#[tokio::test] => #[test]);
  }
)] 
mod tests {
  #[tokio::test]
  pub async fn my_test() {

  }
}
```