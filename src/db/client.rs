use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
use tiberius::{AuthMethod, Client, Config};

use crate::{
    api::error::ApiError,
    config::DatabaseConfig,
};

pub type SqlClient = Client<Compat<TcpStream>>;

#[derive(Clone)]
pub struct Database {
    config: DatabaseConfig,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }

    pub async fn connect(&self) -> Result<SqlClient, ApiError> {
        self.connect_to(Some(&self.config.name)).await
    }

    pub async fn connect_to_master(&self) -> Result<SqlClient, ApiError> {
        self.connect_to(Some("master")).await
    }

    pub async fn ping(&self) -> Result<(), ApiError> {
        let mut client = self.connect().await?;
        client.simple_query("SELECT 1;").await?.into_results().await?;
        Ok(())
    }

    async fn connect_to(&self, database_name: Option<&str>) -> Result<SqlClient, ApiError> {
        let mut config = Config::new();
        config.host(&self.config.host);
        config.port(self.config.port);
        config.authentication(AuthMethod::sql_server(
            self.config.user.clone(),
            self.config.password.clone(),
        ));
        config.trust_cert();

        if let Some(database_name) = database_name {
            config.database(database_name);
        }

        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        Client::connect(config, tcp.compat_write())
            .await
            .map_err(ApiError::from)
    }

    pub fn database_name(&self) -> &str {
        &self.config.name
    }
}
