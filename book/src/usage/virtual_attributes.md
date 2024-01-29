# Virtual Attributes

Virtual attributes allow finer control over code generation.

## Cfg

Filter by module name.

```rust
#[synca::cfg(module_name)]
```

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
    );
  }
)] 
mod example {
  fn client() -> tokio_postgres::Client { 
    #[synca::cfg(tokio)]
    let (client, connection) = tokio_postgres::connect(
      "CONNECTION_STRING", 
      tokio_postgres::NoTls
    ).await?;
    #[synca::cfg(tokio)]
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
      }
    });

    #[synca::cfg(sync)]
    let client = postgres::Client::connect(
      &conn_str, 
      postgres::NoTls
    )?;

    client
  }
}
```

## Ignore

Allows to ignore a marked item.

```rust
#[synca::ignore]
```

```rust
#[synca::synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync { 
    sync!();
    replace!(#[tokio::test] => #[test]);
  }
)] 
mod tests {
  #[tokio::test]
  pub async fn my_test() { 
    #[synca::ignore]
    assert_eq!(format!(".aw{}", "ait"), ".await");
  }
}
```