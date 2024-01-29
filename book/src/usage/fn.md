# Functions

For modules with the sync modifier, the asyncness attribute will be removed 
from all functions and all ".await" expressions will be removed.

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
    );
  }
)]
mod example {
  pub async fn get_name(
    client: &mut tokio_postgres::Client
  ) -> Result<String, tokio_postgres::Error> {
    let row = client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }
}
```

## Generated code

```rust
#[cfg(feature = "tokio")]
mod tokio {
  pub async fn get_name(
    client: &mut tokio_postgres::Client
  ) -> Result<String, tokio_postgres::Error> {
    let row = client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }
}

#[cfg(feature = "sync")]
mod sync {
  pub fn get_name(
    client: &mut postgres::Client
  ) -> Result<String, postgres::Error> {
    let row = client.query_one("SQL", &[]).unwrap();

    row.get("name")
  }
}
```