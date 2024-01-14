# synca example: pg_as_calc

The module allows to use Postgres as a calculator. Contains two features: sync & tokio

## Usage

```
// For sync use
synca_example_pg_as_calc = { version = "0.1.0", default-features = false, features = [ "sync" ] }

// For async use
synca_example_pg_as_calc = { version = "0.1.0", default-features = false, features = [ "tokio" ] }
```

## Features

Describes features

```toml
// Cargo.toml

[features]
default = [ "tokio" ]
sync = [ "dep:postgres" ]
tokio = [ "dep:tokio", "dep:tokio-postgres" ]
```

### We prohibit enabling both features at the same time

```rust
// lib.rs

#[cfg(all(feature = "sync", feature = "tokio"))]
compile_error!(r#"feature "sync" and feature "tokio" cannot be both enabled at the same time"#);
```

### Require that either sync or tokyo be selected

```rust
// lib.rs

#[cfg(all(not(feature = "sync"), not(feature = "tokio")))]
compile_error!(r#"one of the "sync" and "tokio" features must be enabled"#);
```

## Declaring the structure

Synchronous implementation uses class postgres::Client, 
and asynchronous implementation uses class tokio_postgres::Client.

```rust
#[synca(
  feature = "tokio",
  tokio_postgres::Client => postgres::Client
)]
pub struct Calc {
  client: tokio_postgres::Client
}
```

## Adding implementation

All asynchronous functions will become synchronous

```rust
#[synca(
  feature = "tokio",
  tokio_postgres::Error => postgres::Error
)]
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
  feature = "tokio",
  tokio_postgres::Error => postgres::Error,
  #[tokio::test] => #[test],
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