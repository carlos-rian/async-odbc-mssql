use async_obdc_mssql_core::{record::try_convert, base::error::ConversionFailure};
use quaint::{prelude::*, single::Quaint};

async fn sql_test() -> Result<(), ConversionFailure> {
    let conn = match Quaint::new("file:///tmp/example.db").await {
        Ok(r) => r,
        Err(_) => return Err(
          ConversionFailure {
              from: "Infinity",
              to: "",
          })  
      };
    let sql = "select 1 as number;";
    let result = match conn.query_raw(sql, &[]).await {
      Ok(r) => r,
      Err(_) => return Err(
        ConversionFailure {
            from: "Infinity",
            to: "",
        })  
    };
    let rows = try_convert(result)?;
    println!("{:#?}", rows);
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), ConversionFailure> {
    sql_test().await?;
    Ok(())
}
