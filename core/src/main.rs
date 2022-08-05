
use quaint::prelude::Query;

async fn query() -> Result<(), Error> {//Result<Vec<sqlx::mssql::MssqlRow>, Error> {
    let conn_uri = "mssql://sa:Super&-23@localhost:1433/master";

    let query = "
        SELECT *
        FROM TempTest
    ";

    let conn = MssqlPool::connect(conn_uri).await?;

    let rows = sqlx::query(query).fetch_all(&conn).await?;

    println!("{}", rows.len());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let _row = query().await?;
    Ok(())
}
