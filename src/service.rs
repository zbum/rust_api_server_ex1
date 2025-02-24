use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqlitePool};

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub message: String,
}

#[derive(Serialize, FromRow)]
pub struct Greeting {
    id: i32,
    name: String,
}

#[derive(Clone)]
pub struct GreetingService {
    db_pool: SqlitePool,
}

impl GreetingService {
    pub fn new(db_pool: SqlitePool) -> Self {
        GreetingService { db_pool }
    }

    pub async fn greet(&self, info: Info) -> Result<ResponseMessage, sqlx::Error> {
        sqlx::query("INSERT INTO greetings (name) VALUES (?)")
            .bind(&info.name)
            .execute(&self.db_pool)
            .await?;

        Ok(ResponseMessage {
            message: format!("Hello, {}!", info.name),
        })
    }

    pub async fn welcome(&self) -> Result<ResponseMessage, sqlx::Error> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM greetings")
            .fetch_one(&self.db_pool)
            .await?;
        Ok(ResponseMessage {
            message: format!("Welcome! {} names have been greeted.", count.0),
        })
    }

    pub async fn list_names(&self) -> Result<Vec<Greeting>, sqlx::Error> {
        let names = sqlx::query_as::<_, Greeting>("SELECT id, name FROM greetings")
            .fetch_all(&self.db_pool)
            .await?;
        Ok(names)
    }

    pub async fn name_by_id(&self, id: i32) -> Result<Option<Greeting>, sqlx::Error> {
        let name = sqlx::query_as::<_, Greeting>("SELECT id, name FROM greetings WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(name)
    }
}
