use synca::synca;

#[cfg(all(feature = "sync", feature = "tokio"))]
compile_error!(r#"feature "sync" and feature "tokio" cannot be enabled at the same time"#);

#[cfg(all(not(feature = "sync"), not(feature = "tokio")))]
compile_error!(r#"one of the "sync" and "tokio" features must be enabled"#);

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
  pub async fn connect<T: Into<String>>(connection_string: T) -> Result<Self, tokio_postgres::Error> {
    let conn_str = connection_string.into();

    #[cfg(feature = "tokio")]
    let (client, connection) = tokio_postgres::connect(&conn_str, tokio_postgres::NoTls).await?;
    #[cfg(feature = "tokio")]
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
      }
    });

    #[cfg(not(feature = "tokio"))]
    let client = postgres::Client::connect(&conn_str, postgres::NoTls)?;

    Ok(Self { client })
  }

  pub async fn calc(&mut self, s: &str) -> Result<i32, tokio_postgres::Error> {
    let row = self.client.query_one(&format!("SELECT {} result", s), &[]).await?;

    Ok(row.get("result"))
  }
}

#[cfg(test)]
#[synca(
  feature = "tokio",
  tokio_postgres::Error => postgres::Error,
  #[tokio::test] => #[test],
)]
mod tests {
  use super::Calc;

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