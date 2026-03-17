use tokio::time::{Duration, sleep};

use crate::{
    api::error::ApiError,
    config::BootstrapConfig,
    db::Database,
};

const SCHEMA_SQL: &str = r#"
IF OBJECT_ID(N'dbo.users', N'U') IS NULL
BEGIN
    CREATE TABLE dbo.users (
        user_id NVARCHAR(64) NOT NULL PRIMARY KEY,
        username NVARCHAR(64) NOT NULL UNIQUE,
        xp INT NOT NULL DEFAULT 0,
        level INT NOT NULL DEFAULT 1,
        created_at DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME()
    );
END;

IF OBJECT_ID(N'dbo.cards', N'U') IS NULL
BEGIN
    CREATE TABLE dbo.cards (
        card_id NVARCHAR(64) NOT NULL PRIMARY KEY,
        name NVARCHAR(120) NOT NULL,
        rarity NVARCHAR(32) NOT NULL,
        set_name NVARCHAR(64) NOT NULL,
        unlock_level INT NOT NULL
    );
END;

IF OBJECT_ID(N'dbo.rewards', N'U') IS NULL
BEGIN
    CREATE TABLE dbo.rewards (
        reward_id NVARCHAR(64) NOT NULL PRIMARY KEY,
        name NVARCHAR(120) NOT NULL,
        reward_type NVARCHAR(32) NOT NULL,
        amount INT NOT NULL,
        unlock_level INT NOT NULL
    );
END;

IF OBJECT_ID(N'dbo.user_cards', N'U') IS NULL
BEGIN
    CREATE TABLE dbo.user_cards (
        user_id NVARCHAR(64) NOT NULL,
        card_id NVARCHAR(64) NOT NULL,
        unlocked_at DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME(),
        CONSTRAINT PK_user_cards PRIMARY KEY (user_id, card_id),
        CONSTRAINT FK_user_cards_users FOREIGN KEY (user_id) REFERENCES dbo.users(user_id),
        CONSTRAINT FK_user_cards_cards FOREIGN KEY (card_id) REFERENCES dbo.cards(card_id)
    );
END;

IF OBJECT_ID(N'dbo.user_rewards', N'U') IS NULL
BEGIN
    CREATE TABLE dbo.user_rewards (
        user_id NVARCHAR(64) NOT NULL,
        reward_id NVARCHAR(64) NOT NULL,
        quantity INT NOT NULL DEFAULT 1,
        last_unlocked_at DATETIME2 NOT NULL DEFAULT SYSUTCDATETIME(),
        CONSTRAINT PK_user_rewards PRIMARY KEY (user_id, reward_id),
        CONSTRAINT FK_user_rewards_users FOREIGN KEY (user_id) REFERENCES dbo.users(user_id),
        CONSTRAINT FK_user_rewards_rewards FOREIGN KEY (reward_id) REFERENCES dbo.rewards(reward_id)
    );
END;
"#;

const SEED_SQL: &str = r#"
IF NOT EXISTS (SELECT 1 FROM dbo.cards)
BEGIN
    INSERT INTO dbo.cards (card_id, name, rarity, set_name, unlock_level) VALUES
        (N'card-ember-fox', N'Ember Fox', N'Common', N'Origins', 1),
        (N'card-tidal-mage', N'Tidal Mage', N'Rare', N'Origins', 2),
        (N'card-iron-warden', N'Iron Warden', N'Rare', N'Frontier', 3),
        (N'card-shadow-lynx', N'Shadow Lynx', N'Super Rare', N'Eclipse', 4),
        (N'card-aurora-drake', N'Aurora Drake', N'Epic', N'Skies', 5),
        (N'card-celestial-titan', N'Celestial Titan', N'Legendary', N'Mythic', 7);
END;

IF NOT EXISTS (SELECT 1 FROM dbo.rewards)
BEGIN
    INSERT INTO dbo.rewards (reward_id, name, reward_type, amount, unlock_level) VALUES
        (N'reward-gold-100', N'Starter Gold', N'gold', 100, 1),
        (N'reward-pack-basic', N'Basic Booster', N'booster_pack', 1, 2),
        (N'reward-gems-25', N'Gem Cache', N'gems', 25, 4),
        (N'reward-ticket-3', N'Arena Tickets', N'ticket', 3, 6);
END;
"#;

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
        "IF DB_ID(N'{name_literal}') IS NULL BEGIN CREATE DATABASE [{name_identifier}] END;",
        name_literal = escape_string(database.database_name()),
        name_identifier = escape_identifier(database.database_name()),
    );

    let mut master_client = database.connect_to_master().await?;
    master_client
        .simple_query(create_database_sql)
        .await?
        .into_results()
        .await?;

    let mut app_client = database.connect().await?;
    app_client
        .simple_query(SCHEMA_SQL)
        .await?
        .into_results()
        .await?;
    app_client
        .simple_query(SEED_SQL)
        .await?
        .into_results()
        .await?;

    Ok(())
}

fn escape_string(input: &str) -> String {
    input.replace('\'', "''")
}

fn escape_identifier(input: &str) -> String {
    input.replace(']', "]]")
}
