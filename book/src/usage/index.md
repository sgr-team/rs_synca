# Usage

The attribute synca macro is applied to the module that is to be copied. 
As an argument, it accepts modules that should be created based on the template.

In the body of the module it is possible to describe code modifiers.

### Code Modifiers

- sync - converts module code into a synchronous version
- replace - replace types and attributes

```rust
#[synca::synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync { 
    sync!();
    replace!(
      tokio_postgres::Client => postgres::Client,
      tokio_postgres::Error => postgres::Error,
      tokio_postgres::NoTls => postgres::NoTls,
      #[tokio::test] => #[test]
    );
  }
)]
mod my_mod { }
```

Based on this description, two modules will be created

```rust
#[cfg(feature = "tokio")]
pub mod tokio { 
  /* contains an asynchronous version of the code */
}

#[cfg(feature = "sync")]
pub mod sync { 
  /* 
  contains a synchronous version of the code 
  with replaced types and attributes
  */
}
```