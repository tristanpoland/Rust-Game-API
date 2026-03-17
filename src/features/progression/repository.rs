use mysql_async::prelude::Queryable;

use crate::{
    api::error::ApiError,
    db::Database,
    features::{
        catalog::{CardCatalogItem, RewardCatalogItem},
        progression::models::{RewardInventoryItem, UnlockedCard},
    },
};

pub struct ProgressionRepository<'a> {
    database: &'a Database,
}

impl<'a> ProgressionRepository<'a> {
    pub fn new(database: &'a Database) -> Self {
        Self { database }
    }

    pub async fn list_user_cards(&self, user_id: &str) -> Result<Vec<UnlockedCard>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows: Vec<(String, String, String, String, i32, String)> = client
            .exec(
                "SELECT c.card_id,
                        c.name,
                        c.rarity,
                        c.set_name,
                        c.unlock_level,
                        DATE_FORMAT(uc.unlocked_at, '%Y-%m-%dT%H:%i:%sZ') AS unlocked_at
                 FROM user_cards uc
                 INNER JOIN cards c ON c.card_id = uc.card_id
                 WHERE uc.user_id = ?
                 ORDER BY uc.unlocked_at, c.name;",
                (user_id,),
            )
            .await?;

        Ok(rows.into_iter().map(map_unlocked_card).collect())
    }

    pub async fn list_user_rewards(
        &self,
        user_id: &str,
    ) -> Result<Vec<RewardInventoryItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows: Vec<(String, String, String, i32, i32, i32, String)> = client
            .exec(
                "SELECT r.reward_id,
                        r.name,
                        r.reward_type,
                        r.amount,
                        r.unlock_level,
                        ur.quantity,
                        DATE_FORMAT(ur.last_unlocked_at, '%Y-%m-%dT%H:%i:%sZ') AS last_unlocked_at
                 FROM user_rewards ur
                 INNER JOIN rewards r ON r.reward_id = ur.reward_id
                 WHERE ur.user_id = ?
                 ORDER BY ur.last_unlocked_at, r.name;",
                (user_id,),
            )
            .await?;

        Ok(rows.into_iter().map(map_inventory_reward).collect())
    }

    pub async fn find_cards_for_levels(
        &self,
        min_level: i32,
        max_level: i32,
    ) -> Result<Vec<CardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows: Vec<(String, String, String, String, i32)> = client
            .exec(
                "SELECT card_id, name, rarity, set_name, unlock_level
                 FROM cards
                 WHERE unlock_level BETWEEN ? AND ?
                 ORDER BY unlock_level, name;",
                (min_level, max_level),
            )
            .await?;

        Ok(rows.into_iter().map(map_catalog_card).collect())
    }

    pub async fn find_rewards_for_levels(
        &self,
        min_level: i32,
        max_level: i32,
    ) -> Result<Vec<RewardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows: Vec<(String, String, String, i32, i32)> = client
            .exec(
                "SELECT reward_id, name, reward_type, amount, unlock_level
                 FROM rewards
                 WHERE unlock_level BETWEEN ? AND ?
                 ORDER BY unlock_level, name;",
                (min_level, max_level),
            )
            .await?;

        Ok(rows.into_iter().map(map_catalog_reward).collect())
    }

    pub async fn unlock_card(&self, user_id: &str, card_id: &str) -> Result<(), ApiError> {
        let mut client = self.database.connect().await?;
        client
            .exec_drop(
                "INSERT INTO user_cards (user_id, card_id, unlocked_at)
                 VALUES (?, ?, UTC_TIMESTAMP())
                 ON DUPLICATE KEY UPDATE unlocked_at = unlocked_at;",
                (user_id, card_id),
            )
            .await?;

        Ok(())
    }

    pub async fn grant_reward(
        &self,
        user_id: &str,
        reward_id: &str,
        quantity: i32,
    ) -> Result<(), ApiError> {
        let mut client = self.database.connect().await?;
        client
            .exec_drop(
                "INSERT INTO user_rewards (user_id, reward_id, quantity, last_unlocked_at)
                 VALUES (?, ?, ?, UTC_TIMESTAMP())
                 ON DUPLICATE KEY UPDATE
                    quantity = quantity + ?,
                    last_unlocked_at = UTC_TIMESTAMP();",
                (user_id, reward_id, quantity, quantity),
            )
            .await?;

        Ok(())
    }
}

fn map_catalog_card(row: (String, String, String, String, i32)) -> CardCatalogItem {
    CardCatalogItem {
        card_id: row.0,
        name: row.1,
        rarity: row.2,
        set_name: row.3,
        unlock_level: row.4,
    }
}

fn map_catalog_reward(row: (String, String, String, i32, i32)) -> RewardCatalogItem {
    RewardCatalogItem {
        reward_id: row.0,
        name: row.1,
        reward_type: row.2,
        amount: row.3,
        unlock_level: row.4,
    }
}

fn map_unlocked_card(row: (String, String, String, String, i32, String)) -> UnlockedCard {
    UnlockedCard {
        card_id: row.0,
        name: row.1,
        rarity: row.2,
        set_name: row.3,
        unlock_level: row.4,
        unlocked_at: row.5,
    }
}

fn map_inventory_reward(row: (String, String, String, i32, i32, i32, String)) -> RewardInventoryItem {
    RewardInventoryItem {
        reward_id: row.0,
        name: row.1,
        reward_type: row.2,
        amount: row.3,
        unlock_level: row.4,
        quantity: row.5,
        last_unlocked_at: row.6,
    }
}
