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
        let rows = client
            .query(
                "SELECT card_id, name, rarity, set_name, unlock_level
                 FROM dbo.cards
                 ORDER BY unlock_level, rarity, name;",
                &[],
            )
            .await?
            .into_first_result()
            .await?;

        Ok(rows.into_iter().map(map_card).collect())
    }

    pub async fn list_rewards(&self) -> Result<Vec<RewardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows = client
            .query(
                "SELECT reward_id, name, reward_type, amount, unlock_level
                 FROM dbo.rewards
                 ORDER BY unlock_level, reward_type, name;",
                &[],
            )
            .await?
            .into_first_result()
            .await?;

        Ok(rows.into_iter().map(map_reward).collect())
    }

    pub async fn get_card(&self, card_id: &str) -> Result<Option<CardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let row = client
            .query(
                "SELECT card_id, name, rarity, set_name, unlock_level
                 FROM dbo.cards
                 WHERE card_id = @P1;",
                &[&card_id],
            )
            .await?
            .into_row()
            .await?;

        Ok(row.map(map_card))
    }

    pub async fn get_reward(
        &self,
        reward_id: &str,
    ) -> Result<Option<RewardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let row = client
            .query(
                "SELECT reward_id, name, reward_type, amount, unlock_level
                 FROM dbo.rewards
                 WHERE reward_id = @P1;",
                &[&reward_id],
            )
            .await?
            .into_row()
            .await?;

        Ok(row.map(map_reward))
    }
}

fn map_card(row: tiberius::Row) -> CardCatalogItem {
    CardCatalogItem {
        card_id: row
            .get::<&str, _>("card_id")
            .unwrap_or_default()
            .to_string(),
        name: row.get::<&str, _>("name").unwrap_or_default().to_string(),
        rarity: row
            .get::<&str, _>("rarity")
            .unwrap_or_default()
            .to_string(),
        set_name: row
            .get::<&str, _>("set_name")
            .unwrap_or_default()
            .to_string(),
        unlock_level: row.get::<i32, _>("unlock_level").unwrap_or_default(),
    }
}

fn map_reward(row: tiberius::Row) -> RewardCatalogItem {
    RewardCatalogItem {
        reward_id: row
            .get::<&str, _>("reward_id")
            .unwrap_or_default()
            .to_string(),
        name: row.get::<&str, _>("name").unwrap_or_default().to_string(),
        reward_type: row
            .get::<&str, _>("reward_type")
            .unwrap_or_default()
            .to_string(),
        amount: row.get::<i32, _>("amount").unwrap_or_default(),
        unlock_level: row.get::<i32, _>("unlock_level").unwrap_or_default(),
    }
}
