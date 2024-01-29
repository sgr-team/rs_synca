# Traits

For modules with the sync modifier, the asyncness attribute will be removed 
from all trait functions.

Tested with [async-trait](https://github.com/dtolnay/async-trait).

```rust
#[synca::synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync { 
    sync!();
    replace!(tokio_postgres::Client => postgres::Client);
  }
)]
mod exemple {
  trait MyTrait {
    type Client = tokio_postgres::Client;

    fn new() -> Self;
    async fn select() -> String;
  }
}
```

## Generated code

```rust
#[cfg(feature = "tokio")]
pub mod tokio {
  trait MyTrait {
    type Client = tokio_postgres::Client;

    fn new() -> Self;
    async fn select() -> String;
  }
}

#[cfg(feature = "sync")]
pub mod sync {
  trait MyTrait {
    type Client = postgres::Client;

    fn new() -> Self;
    fn select() -> String;
  }
}
```