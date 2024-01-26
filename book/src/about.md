# About

SyncA is a framework for creating crates with both synchronous and asynchronous versions.

## Motivation

When we write a library, we cannot control the environment in which our code will be used.

Often this leads to the fact that there is only a synchronous version, or two different crates.
If the problems of the first solution are obvious, then the second leads to a violation of the 
[DRY principle](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself).

SyncA solves this problem by hiding the asynchronous implementation behind a feature.

## Concept

One macro for everything.

The macro synca::synca generates 2 versions of code: async with attribute #[cfg(feature)] and 
sync with attribute #[cfg(not(feature))].

## Full example

```rust
/// # synca
/// 
/// The macro will create 2 versions of the my_mod
/// With the asynchrony feature enabled - asynchronous version, 
/// without it - synchronous.
#[synca::synca(feature = "async")]
mod my_mod { }

/// # Functions
/// 
/// The macro turns asynchronous functions into synchronous 
/// implementations - removing all the .await calls
#[synca::synca(feature = "async")]
impl MyStruct {
  pub async fn first_name(&mut self) -> String {
    let row = self.client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }
}

/// # Types
/// 
/// synca supports type substitution 
/// The synchronous version will use type postgres::Client
#[synca::synca(
  feature = "async",
  tokio_postgres::Client => postgres::Client
)]
struct MyStruct {
  client: tokio_postgres::Client
} 

/// # Tests
/// 
/// synca support attributes substitution too.
/// This allows you to test not only the asynchronous version, but also the synchronous one
#[synca::synca(
  feature = "async",
  tokio_postgres::Client => postgres::Client,
  #[tokio::test] => #[test]
)]
mod tests {
  #[tokio::test]
  pub async fn my_test() {

  }
}

/// # Traits
///
/// synca also knows how to work with traits
#[synca::synca(
  feature = "async",
  tokio_postgres::Client => postgres::Client,
)]
pub trait MyTrait { 
  async fn get_name(client: &mut tokio_postgres::Client);
}


/// # Docs
/// 
/// synca also contains a documentation processor that allows to generate 
/// different documentation for the synchronous and asynchronous versions
/// 
/// [synca::sync]
/// It's sync doc
/// [/synca::sync]
/// [synca::async]
/// It's async doc
/// [/synca::async]
/// 
/// # Match
/// 
/// You can also use "synca::match" to replace part of a string.
/// 
/// Arguments 
/// - [synca::match]tokio_postgres|postgres[/synca::match]::Client - postgres client
#[synca::synca(
  feature = "async",
  tokio_postgres::Client => postgres::Client,
)]
pub async fn my_fn(client: &mut tokio_postgres::Client) { }

/// # Virtual attributes
/// 
/// - synca::ignore - ignore in code fold
/// - synca::sync_only - the code will only be available in the synchronous version
/// - synca::async_only - the code will only be available in the asynchronous version
#[synca::synca(
  feature = "async",
  tokio_postgres::Client => postgres::Client,
  #[tokio::test] => #[test],
)]
mod my_mod {
  #[tokio::test]
  async fn my_test() { 
    #[synca::ignore]
    assert_eq!(format!(".aw{}", "ait"), ".await");
  }

  #[test]
  #[synca::sync_only]
  fn sync_test() { 
  }

  #[test]
  #[synca::async_only]
  fn async_test() { 
  }
}
```