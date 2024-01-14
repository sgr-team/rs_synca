# synca

Write asynchronous code, and synca will create a synchronous version.

## Docs

[docs.rs](https://docs.rs/synca/latest/synca/)

## Examples

### pg_as_calc

A library that allows you to use Postgres as a calculator. Contains 2 implementations: synchronous and tokyo.
An example demonstrates how to work with postgres and tokio_postgres with synca and how to test it.

- [README.md](https://github.com/sgr-team/rs_synca/blob/main/examples/pg_as_calc/README.md)
- [Cargo.toml](https://github.com/sgr-team/rs_synca/blob/main/examples/pg_as_calc/Cargo.toml)
- [src/lib.rs](https://github.com/sgr-team/rs_synca/blob/main/examples/pg_as_calc/src/lib.rs)

## Concept

The crate contains one attribute macro "synca" which takes the features expression
and replaceable types and attributes.

The macro generates 2 versions of code: async with attribute #[cfg(feature)] and 
sync with attribute #[cfg(not(feature))].

This macro can be applied to the declaration of structures, enums, implementations, traits, 
modules, functions or macros.

```rust
#[synca::synca(
  feature = "tokio",
  tokio_postgres::Client => postgres::Client,
  tokio_postgres::Error => postgres::Error,
  #[tokio::test] => #[test],
)]
mod my_mod {
  type Err = tokio_postgres::Error;

  pub async fn select(client: &mut tokio_postgres::Client) -> Result<i32, Err> {
    let row = client.query_one("SELECT 1 + 2 result", &[]).await?;

    Ok(row.get("result"))
  }
}
```

The macro generates the next code:

```rust
#[cfg(not(feature = "tokio"))]
mod my_mod {
  type Err = postgres::Error;

  pub fn select(client: &mut postgres::Client) -> Result<i32, Err> {
    let row = client.query_one("SELECT 1 + 2 result", &[])?;

    Ok(row.get("result"))
  }
}

#[cfg(feature = "tokio")]
mod my_mod {
  type Err = tokio_postgres::Error;

  pub async fn select(client: &mut tokio_postgres::Client) -> Result<i32, Err> {
    let row = client.query_one("SELECT 1 + 2 result", &[]).await?;

    Ok(row.get("result"))
  }
}
```