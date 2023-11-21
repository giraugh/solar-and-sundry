use super::Model;
use crate::DbClient;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Page {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_id: String,
    pub name: String,
    pub published_status: PublishedStatus,
}

#[derive(Debug, Clone)]
pub enum PublishedStatus {
    Draft,
    PublishedAt(DateTime<Utc>),
}

impl Model for Page {
    /// Pages are uniquely identified by a page number
    type Id = i64;

    fn table_sql() -> &'static str {
        r"CREATE TABLE IF NOT EXISTS page (
            page_number INT PRIMARY KEY,
            chapter_number INT,
            image_id TEXT,
            name TEXT,
            published BOOLEAN
        );"
    }

    async fn get(db: DbClient, id: Self::Id) -> Result<Self, anyhow::Error> {
        todo!()
    }

    async fn get_all(db: DbClient) -> Result<Vec<Self>, anyhow::Error> {
        todo!()
    }

    async fn upsert(db: DbClient, id: Self::Id) -> Result<(), anyhow::Error> {
        todo!()
    }

    async fn delete(db: DbClient, id: Self::Id) -> Result<(), anyhow::Error> {
        todo!()
    }
}
