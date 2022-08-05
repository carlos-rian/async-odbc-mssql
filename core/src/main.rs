use quaint::{prelude::*, single::Quaint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Procedure {
    /// Procedure name.
    pub name: String,
    /// The definition of the procedure.
    pub definition: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), quaint::error::Error> {
    let conn = Quaint::new("file:///tmp/example.db").await?;
    let sql = "select 1 as number;";
    let rows = conn.query_raw(sql, &[]).await?;

    let mut procedures = Vec::with_capacity(rows.len());

    for row in rows.into_iter() {
        procedures.push(Procedure {
            name: row.get_expect_string("name"),
            definition: row.get_string("definition"),
        });
    }

    println!("{:#?}", data);

    Ok(())
}