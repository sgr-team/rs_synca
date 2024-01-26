# Usage

SyncA contains one attribute macro (synca::synca) that automatically generates 
a synchronous version of the code.

This macro can be applied to mod, structs, enums, impl and any other rust 
language items.

## Arguments

The first argument is a features expression and then describes the types 
and attributes that need to be replaced in the synchronized version of the code.

### Features expression

Alows any features expressions

```rust
#[synca::synca(feature = "tokio")] async fn simple() { }
#[synca::synca(all(feature = "async", feature = "tokio"))] async fn all() { }
```

### Types

It is often necessary to use different types for synchronous and asynchronous code. 
SyncA allows you to conveniently describe this once, without using cfg for each line.

```rust
#[synca::synca(
  feature = "tokio",
  tokio_postgres::Client => postgres::Client,
  tokio_postgres::Error => postgres::Error,
)] 
async fn example(client: &mut tokio_postgres::Client) {  }
```

### Attributes

It is also possible to describe the attributes that should be replaced. 
This allows you to keep one test codebase for both the synchronous and asynchronous versions.

```rust
#[synca::synca(
  feature = "tokio",
  #[tokio::test] => #[test],
)] 
mod tests {
  #[tokio::test]
  pub async fn my_test() { }
}
```

## Example

```rust
#[synca::synca(
  feature = "tokio",
  tokio_postgres::Client => postgres::Client,
  #[tokio::test] => #[test],
)]
mod my_mod {
  pub async fn my_fn(client: &mut tokio_postgres::Client) -> String { 
    let row = client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }

  #[cfg(test)]
  mod tests {
    #[tokio::test]
    async fn my_test() { }
  }
}
```

## Generated code

```rust
#[cfg(not(feature = "tokio"))]
mod my_mod {
  pub fn my_fn(client: &mut postgres::Client) -> String { 
    let row = client.query_one("SQL", &[]).unwrap();

    row.get("name")
  }

  #[cfg(test)]
  mod tests {
    #[test]
    fn my_test() { }
  }
}

#[cfg(feature = "tokio")]
mod my_mod {
  pub async fn my_fn(client: &mut tokio_postgres::Client) -> String { 
    let row = client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }

  #[cfg(test)]
  mod tests {
    #[tokio::test]
    async fn my_test() { }
  }
}
```