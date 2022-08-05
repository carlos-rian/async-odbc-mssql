use quaint::{prelude::*, single::Quaint};

#[tokio::main]
async fn main() -> Result<(), quaint::error::Error> {
    let conn = Quaint::new("file:///tmp/example.db").await?;
    let result = conn.select(Select::default().value(1)).await?;

    assert_eq!(
        Some(1),
        result.into_iter().nth(0).and_then(|row| row[0].as_i64()),
    );

    Ok(())
}