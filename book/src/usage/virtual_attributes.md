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

## (Async/Sync) only

Allows you to leave a marked item in only one of the code versions.

```rust
#[synca::synca(feature = "tokio")] 
mod my_mod {
  #[synca::only(async)]
  pub async fn connect() { 
    // async code here
  }

  #[synca::only(sync)]
  pub fn connect() { 
    // sync code here
  }
}
```