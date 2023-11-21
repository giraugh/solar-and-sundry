use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CreatePage {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct PageResponse {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_url: String,
    pub name: String,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct ChapterResponse {
    pub chapter_number: usize,
    pub pages: Vec<PageResponse>,
}
