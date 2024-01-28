# Virtual Attributes

Virtual attributes allow finer control over code generation.

## Ignore

Allows you to ignore a marked item.

```rust
#[synca::synca(
  feature = "tokio",
  #[tokio::test] => #[test],
)] 
mod tests {
  #[tokio::test]
  pub async fn my_test() { 
    #[synca::ignore]
    assert_eq!(format!(".aw{}", "ait"), ".await");
  }
}
```