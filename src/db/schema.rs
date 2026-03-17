use tokio::time::{Duration, sleep};

use crate::{
    api::error::ApiError,
    config::BootstrapConfig,
    db::Database,
};

use mysql_async::prelude::Queryable;

const SCHEMA_SQL: &[&str] = &[
    r#"CREATE TABLE IF NOT EXISTS users (
        user_id VARCHAR(64) NOT NULL PRIMARY KEY,
        username VARCHAR(64) NOT NULL UNIQUE,
        password_hash VARCHAR(255) NULL,
        xp INT NOT NULL DEFAULT 0,
        level INT NOT NULL DEFAULT 1,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#,
    r#"ALTER TABLE users
       ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255) NULL AFTER username;"#,
    r#"CREATE TABLE IF NOT EXISTS cards (
        card_id VARCHAR(64) NOT NULL PRIMARY KEY,
        name VARCHAR(120) NOT NULL,
        rarity VARCHAR(32) NOT NULL,
        set_name VARCHAR(64) NOT NULL,
        unlock_level INT NOT NULL
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#,
    r#"CREATE TABLE IF NOT EXISTS rewards (
        reward_id VARCHAR(64) NOT NULL PRIMARY KEY,
        name VARCHAR(120) NOT NULL,
        reward_type VARCHAR(32) NOT NULL,
        amount INT NOT NULL,
        unlock_level INT NOT NULL
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#,
    r#"CREATE TABLE IF NOT EXISTS user_cards (
        user_id VARCHAR(64) NOT NULL,
        card_id VARCHAR(64) NOT NULL,
        unlocked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (user_id, card_id),
        CONSTRAINT fk_user_cards_users FOREIGN KEY (user_id) REFERENCES users(user_id),
        CONSTRAINT fk_user_cards_cards FOREIGN KEY (card_id) REFERENCES cards(card_id)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#,
    r#"CREATE TABLE IF NOT EXISTS user_rewards (
        user_id VARCHAR(64) NOT NULL,
        reward_id VARCHAR(64) NOT NULL,
        quantity INT NOT NULL DEFAULT 1,
        last_unlocked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (user_id, reward_id),
        CONSTRAINT fk_user_rewards_users FOREIGN KEY (user_id) REFERENCES users(user_id),
        CONSTRAINT fk_user_rewards_rewards FOREIGN KEY (reward_id) REFERENCES rewards(reward_id)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#,
];

const SEED_SQL: &[&str] = &[
    r#"INSERT IGNORE INTO cards (card_id, name, rarity, set_name, unlock_level) VALUES
        ('card-ember-fox', 'Ember Fox', 'Common', 'Origins', 1),
        ('card-tidal-mage', 'Tidal Mage', 'Rare', 'Origins', 2),
        ('card-iron-warden', 'Iron Warden', 'Rare', 'Frontier', 3),
        ('card-shadow-lynx', 'Shadow Lynx', 'Super Rare', 'Eclipse', 4),
        ('card-aurora-drake', 'Aurora Drake', 'Epic', 'Skies', 5),
        ('card-celestial-titan', 'Celestial Titan', 'Legendary', 'Mythic', 7);"#,
    r#"INSERT IGNORE INTO rewards (reward_id, name, reward_type, amount, unlock_level) VALUES
        ('reward-gold-100', 'Starter Gold', 'gold', 100, 1),
        ('reward-pack-basic', 'Basic Booster', 'booster_pack', 1, 2),
        ('reward-gems-25', 'Gem Cache', 'gems', 25, 4),
        ('reward-ticket-3', 'Arena Tickets', 'ticket', 3, 6);"#,
];

pub async fn initialize_database(
    database: &Database,
    bootstrap: &BootstrapConfig,
) -> Result<(), ApiError> {
    let mut last_error: Option<ApiError> = None;

    for attempt in 1..=bootstrap.connect_retries {
        match ensure_database(database).await {
            Ok(()) => return Ok(()),
            Err(error) => {
                last_error = Some(error);

                if attempt == bootstrap.connect_retries {
                    break;
                }

                sleep(Duration::from_secs(bootstrap.connect_delay_secs)).await;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        ApiError::Database("Database bootstrap failed for an unknown reason".to_string())
    }))
}

async fn ensure_database(database: &Database) -> Result<(), ApiError> {
    let create_database_sql = format!(
        "CREATE DATABASE IF NOT EXISTS `{name_identifier}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;",
        name_identifier = escape_identifier(database.database_name()),
    );

    let mut server_client = database.connect_to_server().await?;
    server_client.query_drop(create_database_sql).await?;

    let mut app_client = database.connect().await?;

    for statement in SCHEMA_SQL {
        app_client.query_drop(*statement).await?;
    }

    for statement in SEED_SQL {
        app_client.query_drop(*statement).await?;
    }

    Ok(())
}

fn escape_identifier(input: &str) -> String {
    input.replace('`', "``")
}
