use mysql_async::prelude::Queryable;

use crate::{
    api::error::ApiError,
    db::Database,
    features::catalog::models::{CardCatalogItem, RewardCatalogItem},
};

pub struct CatalogRepository<'a> {
    database: &'a Database,
}

impl<'a> CatalogRepository<'a> {
    pub fn new(database: &'a Database) -> Self {
        Self { database }
    }

    pub async fn list_cards(&self) -> Result<Vec<CardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows: Vec<(String, String, String, String, i32)> = client
            .exec(
                "SELECT card_id, name, rarity, set_name, unlock_level
                 FROM cards
                 ORDER BY unlock_level, rarity, name;",
                (),
            )
            .await?;

        Ok(rows.into_iter().map(map_card).collect())
    }

    pub async fn list_rewards(&self) -> Result<Vec<RewardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows: Vec<(String, String, String, i32, i32)> = client
            .exec(
                "SELECT reward_id, name, reward_type, amount, unlock_level
                 FROM rewards
                 ORDER BY unlock_level, reward_type, name;",
                (),
            )
            .await?;

        Ok(rows.into_iter().map(map_reward).collect())
    }

    pub async fn get_card(&self, card_id: &str) -> Result<Option<CardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let row: Option<(String, String, String, String, i32)> = client
            .exec_first(
                "SELECT card_id, name, rarity, set_name, unlock_level
                 FROM cards
                 WHERE card_id = ?;",
                (card_id,),
            )
            .await?;

        Ok(row.map(map_card))
    }

    pub async fn get_reward(
        &self,
        reward_id: &str,
    ) -> Result<Option<RewardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let row: Option<(String, String, String, i32, i32)> = client
            .exec_first(
                "SELECT reward_id, name, reward_type, amount, unlock_level
                 FROM rewards
                 WHERE reward_id = ?;",
                (reward_id,),
            )
            .await?;

        Ok(row.map(map_reward))
    }
}

fn map_card(row: (String, String, String, String, i32)) -> CardCatalogItem {
    CardCatalogItem {
        card_id: row.0,
        name: row.1,
        rarity: row.2,
        set_name: row.3,
        unlock_level: row.4,
    }
}

fn map_reward(row: (String, String, String, i32, i32)) -> RewardCatalogItem {
    RewardCatalogItem {
        reward_id: row.0,
        name: row.1,
        reward_type: row.2,
        amount: row.3,
        unlock_level: row.4,
    }
}
