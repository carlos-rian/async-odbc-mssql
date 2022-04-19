#[warn(unused_variables)]
#[warn(unused_imports)]
#[warn(dead_code)]

use tiberius::error::Error;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
//use pyo3::{prelude::*, wrap_pyfunction};

async fn query(q: &str, conn_str: &str) -> Result<Vec<Vec<tiberius::Row>>, Error> {
    let config: Config = Config::from_ado_string(conn_str)?;
    let tcp: TcpStream = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut conn = Client::connect(config, tcp.compat_write()).await?; 

    let query_stream = conn.query(q, &[]).await?;
    let rows = query_stream.into_results().await?;
    
    Ok(rows)
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn_str = "\
        server=tcp:localhost,1433; \
        username=SA; \
        password=Super&-23; \
        database=master; \
        TrustServerCertificate=true";
    let q = "select * from spt_monitor;";

    let result = query(&q, &conn_str).await?;
    println!("{:?}", result);

    Ok(())
}
