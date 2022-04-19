use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncWriteCompatExt};


#[warn(unused_variables)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn: String = "server=tcp:localhost,1433;username=SA;password=Super&-2;TrustServerCertificate=true".to_string();
    
    let config: Config = Config::from_ado_string(conn.as_str())?;
    let tcp: TcpStream = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut conn = Client::connect(config, tcp.compat_write()).await?; 

    let res = conn.query("SELECT @P1", &[&-4i32]).await?;
    let _result = res.into_results().await?;

    print!("{:?}", _result);
    
    Ok(())
}
