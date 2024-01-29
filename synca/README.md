# SyncA

SyncA is a framework for creating crates with both synchronous and asynchronous versions.

## Motivation

When we write a library, we cannot control the environment in which our code will be used.

Often this leads to the fact that there is only a synchronous version, or two different crates.
If the problems of the first solution are obvious, then the second leads to a violation of the 
[DRY principle](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself).

SyncA solves the problem by creating copies of modules.

## Concept

The attribute synca macro is applied to the module that is to be copied. 
As an argument, it accepts modules that should be created based on the template.

In the body of the module it is possible to describe code modifiers.

### Code Modifiers

- sync - converts module code into a synchronous version
- replace - replace types and attributes

## Example

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
      #[tokio::test] => #[test]
    );
  }
)] 
mod my_mod { 
  struct MyStruct {
    client: tokio_postgres::Client
  } 
  
  pub async fn get_name(client: &mut tokio_postgres::Client) -> String {
    let row = self.client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }

  #[cfg(test)]
  mod tests {
    use super::get_name;

    #[tokio::test]
    pub async fn get_name_test() {
      assert_eq!(get_name(&mut client()).await, "My name");
    }

    fn client() -> tokio_postgres::Client { 
      #[synca::cfg(tokio)]
      let (client, connection) = tokio_postgres::connect("CONNECTION_STRING", tokio_postgres::NoTls).await?;
      #[synca::cfg(tokio)]
      tokio::spawn(async move {
        if let Err(e) = connection.await {
          eprintln!("connection error: {}", e);
        }
      });

      #[synca::cfg(sync)]
      let client = postgres::Client::connect(&conn_str, postgres::NoTls)?;

      client
    }
  }
}
```

Generated code

```rust
#[cfg(feature = "tokio")]
pub mod tokio { 
  struct MyStruct {
    client: tokio_postgres::Client
  } 
  
  pub async fn get_name(client: &mut tokio_postgres::Client) -> String {
    let row = self.client.query_one("SQL", &[]).await.unwrap();

    row.get("name")
  }

  #[cfg(test)]
  mod tests {
    use super::get_name;

    #[tokio::test]
    pub async fn get_name_test() {
      assert_eq!(get_name(&mut client()).await, "My name");
    }

    fn client() -> tokio_postgres::Client { 
      let (client, connection) = tokio_postgres::connect("CONNECTION_STRING", tokio_postgres::NoTls).await?;
      tokio::spawn(async move {
        if let Err(e) = connection.await {
          eprintln!("connection error: {}", e);
        }
      });

      #[сfg(all(feature = "tokio", not(feature = "tokio")))]
      let client = postgres::Client::connect(&conn_str, postgres::NoTls)?;

      client
    }
  }
}

#[cfg(feature = "sync")]
mod sync { 
  struct MyStruct {
    client: postgres::Client
  } 
  
  pub fn get_name(client: &mut postgres::Client) -> String {
    let row = self.client.query_one("SQL", &[]).unwrap();

    row.get("name")
  }

  #[cfg(test)]
  mod tests {
    use super::get_name;

    #[test]
    pub async fn get_name_test() {
      assert_eq!(get_name(&mut client()), "My name");
    }

    fn client() -> postgres::Client {  
      #[сfg(all(feature = "tokio", not(feature = "tokio")))]
      let (client, connection) = tokio_postgres::connect("CONNECTION_STRING", tokio_postgres::NoTls).await?;
      #[сfg(all(feature = "tokio", not(feature = "tokio")))]
      tokio::spawn(async move {
        if let Err(e) = connection.await {
          eprintln!("connection error: {}", e);
        }
      });

      let client = postgres::Client::connect(&conn_str, postgres::NoTls)?;

      client
    }
  }
}
```