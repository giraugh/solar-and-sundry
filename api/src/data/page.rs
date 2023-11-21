use std::env;

use super::{Model, RowsIterExt};
use crate::DbRef;

use chrono::{DateTime, Utc};
use libsql::{de, params};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Page {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_id: String,
    pub name: String,
    pub published_at: Option<DateTime<Utc>>,
}

impl Page {
    pub fn image_url(&self, variant: &str) -> String {
        let account_hash = env::var("CLOUDFLARE_IMAGES_ACCOUNT_HASH")
            .expect("Can\t find ACCOUNT_HASH binding")
            .to_string();

        format!(
            "https://imagedelivery.net/{}/{}/{}",
            account_hash, self.image_id, variant
        )
    }

    pub async fn get(db: DbRef, page_number: usize) -> Result<Option<Page>, anyhow::Error> {
        let page: Result<Option<Page>, _> = db
            .lock()
            .await
            .query(
                "SELECT * FROM page WHERE published_at IS NOT NULL AND page_number = ?",
                params![page_number as i64],
            )
            .await?
            .next()?
            .map(|row| de::from_row(&row))
            .transpose();
        Ok(page?)
    }

    pub async fn get_all_published(db: DbRef) -> Result<Vec<Page>, anyhow::Error> {
        let pages = db
            .lock()
            .await
            .query(
                "SELECT * FROM page WHERE published_at IS NOT NULL ORDER BY page_number",
                (),
            )
            .await?
            .iter()
            .map(|row| de::from_row(&row.unwrap()))
            .collect::<Result<Vec<Self>, _>>();
        Ok(pages?)
    }

    pub async fn upsert(db: DbRef, page: Page) -> Result<(), anyhow::Error> {
        db.lock()
            .await
            .execute(
                r"
             REPLACE INTO page
               (page_number, chapter_number, image_id, name, published_at)
             VALUES
               (?, ?, ?, ?, ?)
         ",
                params![
                    page.page_number as i64,
                    page.chapter_number as i64,
                    page.image_id,
                    page.name,
                    page.published_at.map(|d| d.to_rfc3339())
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn set_published(
        db: DbRef,
        page_number: usize,
        published: bool,
    ) -> Result<(), anyhow::Error> {
        db.lock()
            .await
            .execute(
                r"UPDATE page SET published_at=? WHERE page_number=?",
                params![
                    published.then(|| Utc::now().to_rfc3339()),
                    page_number as i64,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn delete(db: DbRef, page_number: usize) -> Result<(), anyhow::Error> {
        db.lock()
            .await
            .execute(
                r"DELETE FROM page WHERE page_number=?",
                params![page_number as i64,],
            )
            .await?;
        Ok(())
    }
}

impl Model for Page {
    fn table_sql() -> &'static str {
        r"CREATE TABLE IF NOT EXISTS page (
            page_number INT PRIMARY KEY,
            chapter_number INT NOT NULL,
            image_id TEXT NOT NULL,
            name TEXT NOT NULL,
            published_at STRING
        );"
    }
}
