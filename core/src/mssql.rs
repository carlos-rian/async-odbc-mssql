use tiberius::{Client, Config, AuthMethod};
use async_std::net::TcpStream;

#[async_std::main]
async fn conn(conn_str: str) -> anyhow::Result<()> {
    let mut config = Config::from_ado_string(&conn_str);

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await;
    client
}

#[async_std::main]
async fn query(sql: str) -> anyhow::Result<()> {
    let client: Client::connect = conn();
    
    let stream = client.query(sql).await?;
    let row = stream.into_row().await?.unwrap();
    row
} 

#[async_std::main]
async fn execute(sql: str) -> anyhow::Result<()> {
    let client: Client::connect = conn();
    
    let stream = client.execute(sql).await?;
    row
}