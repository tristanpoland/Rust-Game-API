use crate::{
    api::error::ApiError,
    db::Database,
    features::{
        catalog::{CatalogRepository, RewardCatalogItem},
        progression::{
            models::{ClaimRewardRequest, GrantProgressRequest, ProgressionResult},
            repository::ProgressionRepository,
        },
        users::{UserProfile, UsersRepository},
    },
};

pub struct ProgressionService<'a> {
    users_repository: UsersRepository<'a>,
    catalog_repository: CatalogRepository<'a>,
    progression_repository: ProgressionRepository<'a>,
}

impl<'a> ProgressionService<'a> {
    pub fn new(database: &'a Database) -> Self {
        Self {
            users_repository: UsersRepository::new(database),
            catalog_repository: CatalogRepository::new(database),
            progression_repository: ProgressionRepository::new(database),
        }
    }

    pub async fn list_user_cards(
        &self,
        user_id: &str,
    ) -> Result<Vec<super::UnlockedCard>, ApiError> {
        self.ensure_user_exists(user_id).await?;
        self.progression_repository.list_user_cards(user_id).await
    }

    pub async fn list_user_rewards(
        &self,
        user_id: &str,
    ) -> Result<Vec<super::RewardInventoryItem>, ApiError> {
        self.ensure_user_exists(user_id).await?;
        self.progression_repository.list_user_rewards(user_id).await
    }

    pub async fn grant_progress(
        &self,
        user_id: &str,
        request: GrantProgressRequest,
    ) -> Result<ProgressionResult, ApiError> {
        if request.xp_gained <= 0 {
            return Err(ApiError::Validation(
                "xp_gained must be greater than zero".to_string(),
            ));
        }

        let user = self.ensure_user_exists(user_id).await?;
        let new_xp = user.xp + request.xp_gained;
        let new_level = xp_to_level(new_xp);
        let previous_level = user.level;

        let updated_user = self
            .users_repository
            .update_progress(user_id, new_xp, new_level)
            .await?;

        let mut newly_unlocked_cards = Vec::new();
        let mut newly_unlocked_rewards = Vec::new();

        if new_level > previous_level {
            let cards = self
                .progression_repository
                .find_cards_for_levels(previous_level + 1, new_level)
                .await?;
            let rewards = self
                .progression_repository
                .find_rewards_for_levels(previous_level + 1, new_level)
                .await?;

            for card in &cards {
                self.progression_repository
                    .unlock_card(user_id, &card.card_id)
                    .await?;
            }

            for reward in &rewards {
                self.progression_repository
                    .grant_reward(user_id, &reward.reward_id, 1)
                    .await?;
            }

            newly_unlocked_cards = cards;
            newly_unlocked_rewards = rewards;
        }

        Ok(ProgressionResult {
            user: updated_user,
            xp_gained: request.xp_gained,
            previous_level,
            current_level: new_level,
            newly_unlocked_cards,
            newly_unlocked_rewards,
        })
    }

    pub async fn unlock_card(
        &self,
        user_id: &str,
        card_id: &str,
    ) -> Result<super::UnlockedCard, ApiError> {
        self.ensure_user_exists(user_id).await?;

        let card = self
            .catalog_repository
            .get_card(card_id)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Card with id '{card_id}' was not found")))?;

        self.progression_repository.unlock_card(user_id, card_id).await?;

        let card_id = card.card_id;
        let user_cards = self.progression_repository.list_user_cards(user_id).await?;

        user_cards
            .into_iter()
            .find(|entry| entry.card_id == card_id)
            .ok_or_else(|| {
                ApiError::Internal(format!(
                    "Card '{card_id}' was unlocked but could not be reloaded for user '{user_id}'"
                ))
            })
    }

    pub async fn claim_reward(
        &self,
        user_id: &str,
        reward_id: &str,
        request: ClaimRewardRequest,
    ) -> Result<RewardCatalogItem, ApiError> {
        self.ensure_user_exists(user_id).await?;

        let reward = self
            .catalog_repository
            .get_reward(reward_id)
            .await?
            .ok_or_else(|| {
                ApiError::NotFound(format!("Reward with id '{reward_id}' was not found"))
            })?;

        let quantity = request.quantity.unwrap_or(1);

        if quantity <= 0 {
            return Err(ApiError::Validation(
                "Reward quantity must be greater than zero".to_string(),
            ));
        }

        self.progression_repository
            .grant_reward(user_id, reward_id, quantity)
            .await?;

        Ok(reward)
    }

    async fn ensure_user_exists(&self, user_id: &str) -> Result<UserProfile, ApiError> {
        self.users_repository
            .get_user(user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("User with id '{user_id}' was not found")))
    }
}

pub fn xp_to_level(xp: i32) -> i32 {
    let normalized_xp = xp.max(0);
    (normalized_xp / 100) + 1
}

#[cfg(test)]
mod tests {
    use super::xp_to_level;

    #[test]
    fn computes_level_from_xp_in_100_point_bands() {
        assert_eq!(xp_to_level(0), 1);
        assert_eq!(xp_to_level(99), 1);
        assert_eq!(xp_to_level(100), 2);
        assert_eq!(xp_to_level(350), 4);
    }

    #[test]
    fn never_returns_level_below_one() {
        assert_eq!(xp_to_level(-50), 1);
    }
}
