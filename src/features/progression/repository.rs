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
        let rows = client
            .query(
                "SELECT c.card_id,
                        c.name,
                        c.rarity,
                        c.set_name,
                        c.unlock_level,
                        CONVERT(VARCHAR(33), uc.unlocked_at, 126) AS unlocked_at
                 FROM dbo.user_cards uc
                 INNER JOIN dbo.cards c ON c.card_id = uc.card_id
                 WHERE uc.user_id = @P1
                 ORDER BY uc.unlocked_at, c.name;",
                &[&user_id],
            )
            .await?
            .into_first_result()
            .await?;

        Ok(rows.into_iter().map(map_unlocked_card).collect())
    }

    pub async fn list_user_rewards(
        &self,
        user_id: &str,
    ) -> Result<Vec<RewardInventoryItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows = client
            .query(
                "SELECT r.reward_id,
                        r.name,
                        r.reward_type,
                        r.amount,
                        r.unlock_level,
                        ur.quantity,
                        CONVERT(VARCHAR(33), ur.last_unlocked_at, 126) AS last_unlocked_at
                 FROM dbo.user_rewards ur
                 INNER JOIN dbo.rewards r ON r.reward_id = ur.reward_id
                 WHERE ur.user_id = @P1
                 ORDER BY ur.last_unlocked_at, r.name;",
                &[&user_id],
            )
            .await?
            .into_first_result()
            .await?;

        Ok(rows.into_iter().map(map_inventory_reward).collect())
    }

    pub async fn find_cards_for_levels(
        &self,
        min_level: i32,
        max_level: i32,
    ) -> Result<Vec<CardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows = client
            .query(
                "SELECT card_id, name, rarity, set_name, unlock_level
                 FROM dbo.cards
                 WHERE unlock_level BETWEEN @P1 AND @P2
                 ORDER BY unlock_level, name;",
                &[&min_level, &max_level],
            )
            .await?
            .into_first_result()
            .await?;

        Ok(rows.into_iter().map(map_catalog_card).collect())
    }

    pub async fn find_rewards_for_levels(
        &self,
        min_level: i32,
        max_level: i32,
    ) -> Result<Vec<RewardCatalogItem>, ApiError> {
        let mut client = self.database.connect().await?;
        let rows = client
            .query(
                "SELECT reward_id, name, reward_type, amount, unlock_level
                 FROM dbo.rewards
                 WHERE unlock_level BETWEEN @P1 AND @P2
                 ORDER BY unlock_level, name;",
                &[&min_level, &max_level],
            )
            .await?
            .into_first_result()
            .await?;

        Ok(rows.into_iter().map(map_catalog_reward).collect())
    }

    pub async fn unlock_card(&self, user_id: &str, card_id: &str) -> Result<(), ApiError> {
        let mut client = self.database.connect().await?;
        client
            .execute(
                "IF NOT EXISTS (
                        SELECT 1
                        FROM dbo.user_cards
                        WHERE user_id = @P1 AND card_id = @P2
                 )
                 BEGIN
                    INSERT INTO dbo.user_cards (user_id, card_id, unlocked_at)
                    VALUES (@P1, @P2, SYSUTCDATETIME())
                 END;",
                &[&user_id, &card_id],
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
            .execute(
                "IF EXISTS (
                        SELECT 1
                        FROM dbo.user_rewards
                        WHERE user_id = @P1 AND reward_id = @P2
                 )
                 BEGIN
                    UPDATE dbo.user_rewards
                    SET quantity = quantity + @P3,
                        last_unlocked_at = SYSUTCDATETIME()
                    WHERE user_id = @P1 AND reward_id = @P2
                 END
                 ELSE
                 BEGIN
                    INSERT INTO dbo.user_rewards (user_id, reward_id, quantity, last_unlocked_at)
                    VALUES (@P1, @P2, @P3, SYSUTCDATETIME())
                 END;",
                &[&user_id, &reward_id, &quantity],
            )
            .await?;

        Ok(())
    }
}

fn map_catalog_card(row: tiberius::Row) -> CardCatalogItem {
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

fn map_catalog_reward(row: tiberius::Row) -> RewardCatalogItem {
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

fn map_unlocked_card(row: tiberius::Row) -> UnlockedCard {
    UnlockedCard {
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
        unlocked_at: row
            .get::<&str, _>("unlocked_at")
            .unwrap_or_default()
            .to_string(),
    }
}

fn map_inventory_reward(row: tiberius::Row) -> RewardInventoryItem {
    RewardInventoryItem {
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
        quantity: row.get::<i32, _>("quantity").unwrap_or_default(),
        last_unlocked_at: row
            .get::<&str, _>("last_unlocked_at")
            .unwrap_or_default()
            .to_string(),
    }
}
