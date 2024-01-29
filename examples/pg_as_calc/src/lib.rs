#[synca::synca(
  #[cfg(feature = "tokio")]
  pub mod tokio { },
  #[cfg(feature = "sync")]
  pub mod sync { 
    sync!();
    replace!(
      crate::tokio::Calc => crate::sync::Calc,
      tokio_postgres::Client => postgres::Client,
      tokio_postgres::Error => postgres::Error,
      #[tokio::test] => #[test]
    );
  }
)]
mod calc {
  pub struct Calc {
    client: tokio_postgres::Client
  }

  impl Calc {
    pub async fn connect<T: Into<String>>(connection_string: T) -> Result<Self, tokio_postgres::Error> {
      let conn_str = connection_string.into();
  
      #[synca::cfg(tokio)]
      let (client, connection) = tokio_postgres::connect(&conn_str, tokio_postgres::NoTls).await?;
      #[synca::cfg(tokio)]
      tokio::spawn(async move {
        if let Err(e) = connection.await {
          eprintln!("connection error: {}", e);
        }
      });
  
      #[synca::cfg(sync)]
      let client = postgres::Client::connect(&conn_str, postgres::NoTls)?;
  
      Ok(Self { client })
    }
  
    pub async fn calc(&mut self, eval_str: &str) -> Result<i32, tokio_postgres::Error> {
      let row = self.client.query_one(
        &format!("SELECT {} result", eval_str), 
        &[]
      ).await?;
  
      Ok(row.get("result"))
    }
  }

  #[cfg(test)]
  mod tests {
    type Calc = crate::tokio::Calc;
    type Error = tokio_postgres::Error;

    #[tokio::test]
    async fn calc_plus() {
      let mut calc = connect().await.unwrap();

      assert_eq!(calc.calc("10 + 2").await.unwrap(), 12);
    }

    #[tokio::test]
    async fn calc_minus() {
      let mut calc = connect().await.unwrap();

      assert_eq!(calc.calc("10 - 2").await.unwrap(), 8);
    }

    async fn connect() -> Result<Calc, Error> {
      Calc::connect("postgresql://postgres:123456@localhost:5432/main").await
    }
  }
}