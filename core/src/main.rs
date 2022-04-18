use tiberius::error::Error;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};

struct ConnectParams {
    host: String, 
    port: u16, 
    username: String, 
    password: String,
    trust_cert: bool
} 

pub struct Connection {
    conn_params: ConnectParams
} 

impl Connection {
    fn new(params: ConnectParams) -> Self {
        Self {
            conn_params: params
        }
    }
    async fn connect(&mut self) -> Result<Client<Compat<TcpStream>>, Error>{//Result<Client<tokio_util::compat::Compat<TcpStream>>, Error> {
        let mut config = Config::new();

        config.host(&self.conn_params.host);
        config.port(self.conn_params.port);
        config.authentication(AuthMethod::sql_server(
            &self.conn_params.username, 
            &self.conn_params.password)
        );
        if self.conn_params.trust_cert {
            config.trust_cert();
        }
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        Ok(Client::connect(config, tcp.compat_write()).await?)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let params = ConnectParams {
        host: "localhost".to_string(),
        port: 1433,
        username: "SA".to_string(),
        password: "Super&-23".to_string(),
        trust_cert: true
    };

    let mut client = Connection::new(params);

    let mut conn = client.connect().await?;

    let res = conn.query("SELECT @P1", &[&-4i32]).await?;

    let result = res.into_results().await?;

    println!("{:?}", result);

    Ok(())
}
