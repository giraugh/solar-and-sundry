use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{chapter::Chapter, page::Page};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePage {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_url: String,
    pub name: String,
    pub published_at: Option<DateTime<Utc>>,
}

impl From<CreatePage> for Page {
    fn from(create_page: CreatePage) -> Self {
        Self {
            page_number: create_page.page_number,
            name: create_page.name,
            image_id: create_page.image_id,
            chapter_number: create_page.chapter_number,
            published_at: None,
        }
    }
}

impl From<Page> for PageResponse {
    fn from(value: Page) -> Self {
        Self {
            image_url: value.image_url(),
            page_number: value.page_number,
            chapter_number: value.chapter_number,
            name: value.name,
            published_at: value.published_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterResponse {
    pub chapter_number: usize,
    pub pages: Vec<PageResponse>,
}

impl From<Chapter> for ChapterResponse {
    fn from(value: Chapter) -> Self {
        Self {
            chapter_number: value.chapter_number,
            pages: value.pages.into_iter().map(Into::into).collect(),
        }
    }
}
