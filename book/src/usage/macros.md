# Macros

Macros are the most painful thing. Their flexibility makes it 
difficult to understand which particular token is their argument.

Which leads to the idea of ignoring macros, but this in turn will 
lead to the inoperability of such code (fn awesome_test).

```rust
#[synca::synca(
  feature = "tokio",
  #[tokio::test] => #[test],
)] 
mod tests {
  #[tokio::test]
  pub async fn awesome_test() { 
    assert_eq!(my_struct.may_by_async().await, 42);
  }

  #[tokio::test]
  pub async fn sad_variant_but_working() { 
    let answer = my_struct.may_by_async().await;
    
    assert_eq!(answer, 42);
  }
}
```

Therefore, synca::synca treats macro arguments as text and removes 
occurrences of ".await" and ". await".

You can disable this behavior with the 
[virtual attribute "#[synca_ignore]"](./virtual_attributes.html#ignore).

```rust
#[synca::synca(
  feature = "tokio",
  #[tokio::test] => #[test],
)] 
mod tests {
  #[tokio::test]
  pub async fn my_test() { 
    #[synca_ignore]
    assert_eq!(format!(".aw{}", "ait"), ".await");
  }
}
```