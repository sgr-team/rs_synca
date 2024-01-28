# synca example: pg_as_calc

The module allows to use Postgres as a calculator. Contains two features: sync & tokio

## Usage

```
// For sync use
synca_example_pg_as_calc = { version = "0.1.0", features = [ "sync" ] }

// For async use
synca_example_pg_as_calc = { version = "0.1.0", features = [ "tokio" ] }
```

## Features

Describes features

```toml
// Cargo.toml

[features]
default = [ ]
sync = [ "dep:postgres" ]
tokio = [ "dep:tokio", "dep:tokio-postgres" ]
```

## Declaring the structure

Synchronous implementation uses class postgres::Client, 
and asynchronous implementation uses class tokio_postgres::Client.

```rust
#[synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync { 
    sync!();
    replace!(
      tokio_postgres::Client => postgres::Client,
      tokio_postgres::Error => postgres::Error,
      #[tokio::test] => #[test]
    );
  }
)]
pub mod synca_mod {
  pub struct Calc {
    client: tokio_postgres::Client
  }
}
```

## Adding implementation

All asynchronous functions will become synchronous

```rust
impl Calc {
  pub async fn calc(&mut self, s: &str) -> Result<i32, tokio_postgres::Error> {
    let row = self.client.query_one(&format!("SELECT {} result", s), &[]).await?;

    Ok(row.get("result"))
  }
}
```

## Tests

All asynchronous functions will become synchronous without the "tokyo" feature enabled.

Don't forget to add CI scripts

```bash
cargo test --no-default-features --features=sync
cargo test --no-default-features --features=tokio
```

```rust
#[cfg(test)]
#[synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync { 
    sync!();
    replace!(
      tokio_postgres::Client => postgres::Client,
      tokio_postgres::Error => postgres::Error,
      #[tokio::test] => #[test]
    );
  }
)]
mod tests {
  #[tokio::test]
  async fn calc_plus() {
    let mut calc = connect().await.unwrap();

    assert_eq!(calc.calc("10 + 2").await.unwrap(), 12);
  }

  async fn connect() -> Result<Calc, Error> {
    Calc::connect("postgresql://postgres:123456@localhost:5432/main").await
  }
}
```