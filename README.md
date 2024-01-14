# SyncA

Write asynchronous code, and synca will create a synchronous version.

## How to use

```rust
use synca::synca;

#[synca(
  feature = "tokio",
  tokio_postgres::Client => postgres::Client
)]
pub struct Calc {
  client: tokio_postgres::Client
}

#[synca(
  feature = "tokio",
  tokio_postgres::Error => postgres::Error
)]
impl Calc {
  type Error = tokio_postgres::Error;

  pub async fn select(&mut self) -> Result<i32, Error> {
    let row = self.query_one("SELECT 1 as result").await.unwrap();

    row.get("result")
  }
}
```

Generated code

```rust
use synca::synca;

#[cfg(not(feature = "tokio"))]
pub struct Calc {
  client: postgres::Client
}

#[cfg(feature = "tokio")]
pub struct Calc {
  client: tokio_postgres::Client
}

#[cfg(not(feature = "tokio"))]
impl Calc {
  type Error = postgres::Error;

  pub fn select(&mut self) -> Result<i32, Error> {
    let row = self.query_one("SELECT 1 as result").unwrap();

    row.get("result")
  }
}

#[cfg(feature = "tokio")]
impl Calc {
  type Error = tokio_postgres::Error;

  pub async fn select(&mut self) -> Result<i32, Error> {
    let row = self.query_one("SELECT 1 as result").await.unwrap();

    row.get("result")
  }
}
```

## How to test

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
}
```

Generated code

```rust
#[cfg(test)]
#[cfg(not(feature = "tokio"))]
mod tests {
  #[test]
  fn calc_plus() {
    let mut calc = connect().unwrap();

    assert_eq!(calc.calc("10 + 2").unwrap(), 12);
  }
}

#[cfg(test)]
#[cfg(feature = "tokio")]
mod tests {
  #[tokio::test]
  async fn calc_plus() {
    let mut calc = connect().await.unwrap();

    assert_eq!(calc.calc("10 + 2").await.unwrap(), 12);
  }
}
```

## Examples

### pg_as_calc

A library that allows you to use Postgres as a calculator