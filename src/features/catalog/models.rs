use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CardCatalogItem {
    pub card_id: String,
    pub name: String,
    pub rarity: String,
    pub set_name: String,
    pub unlock_level: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardCatalogItem {
    pub reward_id: String,
    pub name: String,
    pub reward_type: String,
    pub amount: i32,
    pub unlock_level: i32,
}
