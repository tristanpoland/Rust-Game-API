use mysql_async::{Conn, OptsBuilder, prelude::Queryable};

use crate::{
    api::error::ApiError,
    config::DatabaseConfig,
};

#[derive(Clone)]
pub struct Database {
    config: DatabaseConfig,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }

    pub async fn connect(&self) -> Result<Conn, ApiError> {
        self.connect_to(Some(&self.config.name)).await
    }

    pub async fn connect_to_server(&self) -> Result<Conn, ApiError> {
        self.connect_to(None).await
    }

    pub async fn ping(&self) -> Result<(), ApiError> {
        let mut client = self.connect().await?;
        client.query_drop("SELECT 1;").await?;
        Ok(())
    }

    async fn connect_to(&self, database_name: Option<&str>) -> Result<Conn, ApiError> {
        let mut builder = OptsBuilder::default()
            .ip_or_hostname(Some(self.config.host.clone()))
            .tcp_port(self.config.port)
            .user(Some(self.config.user.clone()))
            .pass(Some(self.config.password.clone()));

        if let Some(database_name) = database_name {
            builder = builder.db_name(Some(database_name.to_string()));
        }

        Conn::new(builder).await.map_err(ApiError::from)
    }

    pub fn database_name(&self) -> &str {
        &self.config.name
    }
}
