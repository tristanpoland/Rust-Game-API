use crate::{
    api::error::ApiError,
    db::Database,
    features::catalog::{
        models::{CardCatalogItem, RewardCatalogItem},
        repository::CatalogRepository,
    },
};

pub struct CatalogService<'a> {
    repository: CatalogRepository<'a>,
}

impl<'a> CatalogService<'a> {
    pub fn new(database: &'a Database) -> Self {
        Self {
            repository: CatalogRepository::new(database),
        }
    }

    pub async fn list_cards(&self) -> Result<Vec<CardCatalogItem>, ApiError> {
        self.repository.list_cards().await
    }

    pub async fn list_rewards(&self) -> Result<Vec<RewardCatalogItem>, ApiError> {
        self.repository.list_rewards().await
    }
}
