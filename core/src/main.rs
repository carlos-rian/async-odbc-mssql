use quaint::error::Error;
use quaint::{prelude::*, single::Quaint};

async fn sql_test() -> Result<ResultSet, Error> {
    let conn = Quaint::new("file:///tmp/example.db").await?;
    let sql = "select 1 as number;";
    let result = conn.query_raw(sql, &[]).await?;
    Ok(result)
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let rows =  sql_test().await?;
    println!("{:#?}", rows);
    Ok(())
}

//#[pyfunction]
//fn call_sql_test(py: Python<'_>) -> PyResult<&PyAny>{
//    pyo3_asyncio::async_std::future_into_py(py, async move {
//        sql_test().await;
//        Ok(Python::with_gil(|py| py.None()))
//    })
//}