use anyhow::Result;
use std::path::Path;
use tracing::{debug, info};

use sqlx::{Pool, Row, Sqlite, SqlitePool, migrate::MigrateDatabase};

const DB_DIR: &str = "db/";
const DATABASE_URL: &str = "sqlite://db/exp_system.db";

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            pool: Database::setup_database().await?,
        })
    }

    async fn setup_database() -> Result<Pool<Sqlite>> {
        if !Path::new(DB_DIR).exists() {
            info!("[DB] Creating database directory");
            std::fs::create_dir(DB_DIR)?;
        }

        if !Sqlite::database_exists(DATABASE_URL).await? {
            info!("[DB] Creating database");
            Sqlite::create_database(DATABASE_URL).await?;
        }

        info!("[DB] Connecting to database");
        let db = SqlitePool::connect(DATABASE_URL).await?;

        info!("[DB] Ensuring tables exist");
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        exp INTEGER NOT NULL);",
        )
        .execute(&db)
        .await?;

        Ok(db)
    }

    async fn get_user_exp(&self, user_id: u64) -> Result<i64> {
        debug!("[DB] Fetching user experience for user_id: {}", user_id);
        Ok(sqlx::query("SELECT exp FROM users WHERE id=$1")
            .bind(user_id as i64)
            .fetch_one(&self.pool)
            .await
            .map_or(0, |row| row.get("exp")))
    }

    pub async fn get_user_data(&self, user_id: u64) -> Result<models::UserData> {
        debug!("[DB] Fetching user data for user_id: {}", user_id);
        let exp = self.get_user_exp(user_id).await?;

        debug!("[DB] Fetching user position for user_id: {}", user_id);
        let position: u64 =
            sqlx::query("SELECT COUNT(*) FROM users WHERE exp > $1 ORDER BY exp DESC")
                .bind(exp)
                .fetch_one(&self.pool)
                .await
                .map_or(0, |row| row.get(0));

        debug!("[DB] Fetching total users");
        let total_users: u64 = sqlx::query("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await
            .map_or(0, |row| row.get(0));

        Ok(models::UserData {
            id: user_id,
            exp,
            position: position + 1,
            total_users,
        })
    }

    pub async fn add_experience(&self, user_id: u64, exp: i64) -> Result<(i64, i64)> {
        debug!("[DB] Adding experience to user_id: {}", user_id);
        let before = self.get_user_exp(user_id).await?;
        sqlx::query("INSERT OR IGNORE INTO users (id, exp) VALUES ($1, 0)")
            .bind(user_id as i64)
            .execute(&self.pool)
            .await?;

        sqlx::query("UPDATE users SET exp = exp + $1 WHERE id = $2")
            .bind(exp)
            .bind(user_id as i64)
            .execute(&self.pool)
            .await?;

        Ok((before, before + exp))
    }

    pub async fn get_top(&self, limit: u64) -> Result<Vec<models::UserData>> {
        debug!("[DB] Fetching top users");
        let rows = sqlx::query("SELECT * FROM users ORDER BY exp DESC LIMIT $1")
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let mut users = Vec::new();
        for (index, row) in rows.iter().enumerate() {
            users.push(models::UserData {
                id: row.get("id"),
                exp: row.get("exp"),
                position: index as u64 + 1,
                total_users: rows.len() as u64 + 1,
            });
        }
        Ok(users)
    }
}

pub mod models {
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct UserData {
        pub id: u64,
        pub exp: i64,
        pub position: u64,
        pub total_users: u64,
    }
}
