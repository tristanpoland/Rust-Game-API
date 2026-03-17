use rocket::serde::Deserialize;
use serde::Serialize;

use crate::features::{
    catalog::{CardCatalogItem, RewardCatalogItem},
    users::UserProfile,
};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GrantProgressRequest {
    pub xp_gained: i32,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClaimRewardRequest {
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnlockedCard {
    pub card_id: String,
    pub name: String,
    pub rarity: String,
    pub set_name: String,
    pub unlock_level: i32,
    pub unlocked_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardInventoryItem {
    pub reward_id: String,
    pub name: String,
    pub reward_type: String,
    pub amount: i32,
    pub unlock_level: i32,
    pub quantity: i32,
    pub last_unlocked_at: String,
}

#[derive(Debug, Serialize)]
pub struct ProgressionResult {
    pub user: UserProfile,
    pub xp_gained: i32,
    pub previous_level: i32,
    pub current_level: i32,
    pub newly_unlocked_cards: Vec<CardCatalogItem>,
    pub newly_unlocked_rewards: Vec<RewardCatalogItem>,
}
