# Traits

SyncA allows you to turn asynchronous traits into synchronous ones.

Supported [async-trait](https://github.com/dtolnay/async-trait).

```rust
#[synca::synca(
  feature = "tokio",
  tokio_postgres::Client => postgres::Client,
)]
trait MyTrait {
  type Client = tokio_postgres::Client;

  fn new() -> Self;
  async fn select() -> String;
}
```

## Generated code

```rust
#[cfg(not(feature = "tokio"))]
trait MyTrait {
  type Client = postgres::Client;

  fn new() -> Self;
  fn select() -> String;
}

#[cfg(feature = "tokio")]
trait MyTrait {
  type Client = tokio_postgres::Client;

  fn new() -> Self;
  async fn select() -> String;
}
```