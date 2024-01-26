# Functions

Synca will leave synchronous functions unchanged, but will format asynchronous ones.

```rust
#[synca::synca(
  feature = "tokio",
  tokio_postgres::Client => postgres::Client,
  tokio_postgres::Error => postgres::Error,
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
#[cfg(not(feature = "tokio"))]
mod example {
  pub fn get_name(
    client: &mut postgres::Client
  ) -> Result<String, postgres::Error> {
    let row = client.query_one("SQL", &[]).unwrap();

    row.get("name")
  }
}

#[cfg(feature = "tokio")]
mod example {
  pub async fn get_name(
    client: &mut tokio_postgres::Client
  ) -> Result<String, tokio_postgres::Error> {
    let row = client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }
}
```