use crate::DbRef;

use itertools::Itertools;
use libsql::{de, params};
use serde::Deserialize;

use super::{page::Page, RowsIterExt};

#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub chapter_number: usize,
    pub pages: Vec<Page>,
}

impl Chapter {
    pub async fn get(db: DbRef, chapter_number: usize) -> Result<Option<Chapter>, anyhow::Error> {
        // Get pages in the chapter
        let pages = db
            .lock()
            .await
            .query(
                "SELECT * FROM page WHERE published_at IS NOT NULL AND chapter_number = ? ORDER BY page_number",
                params![chapter_number as i64]
            )
            .await?
            .iter()
            .map(|row| de::from_row(&row.unwrap()))
            .collect::<Result<Vec<Page>, _>>()?;

        Ok(if pages.is_empty() {
            None
        } else {
            Some(Chapter {
                chapter_number,
                pages,
            })
        })
    }

    pub async fn get_all(db: DbRef) -> Result<Vec<Chapter>, anyhow::Error> {
        // Get all published pages
        let pages = Page::get_all_published(db).await?;

        // Group into chapters
        let chapter_pages = pages.into_iter().group_by(|page| page.chapter_number);
        Ok(chapter_pages
            .into_iter()
            .map(|(chapter_number, pages)| Chapter {
                chapter_number,
                pages: pages.collect(),
            })
            .collect())
    }
}
