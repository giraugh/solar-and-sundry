use std::collections::HashMap;

use futures::future::try_join_all;
use serde::{Deserialize, Serialize};
use worker::{
    kv::{KvError, KvStore},
    Url,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_id: String,
    pub is_published: bool,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePageBody {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub chapter_number: usize,
    pub pages: Vec<Page>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterResponse {
    pub chapter_number: usize,
    pub pages: Vec<PageResponse>,
}

impl From<CreatePageBody> for Page {
    fn from(create_page: CreatePageBody) -> Self {
        Page {
            page_number: create_page.page_number,
            chapter_number: create_page.chapter_number,
            image_id: create_page.image_id,
            name: create_page.name,
            is_published: false,
        }
    }
}

impl Page {
    fn page_key(page_number: usize) -> String {
        format!("page-{}", page_number)
    }

    pub fn key(&self) -> String {
        Self::page_key(self.page_number)
    }

    pub async fn get_by_number(
        store: &KvStore,
        page_number: usize,
    ) -> Result<Option<Self>, KvError> {
        store.get(&Self::page_key(page_number)).json().await
    }

    pub async fn save(&self, store: &KvStore) -> Result<(), KvError> {
        store.put(&self.key(), self)?.execute().await
    }

    pub async fn delete(&self, store: &KvStore) -> Result<(), KvError> {
        store.delete(&self.key()).await
    }

    pub fn image_url(&self, account_hash: &str) -> Url {
        // let account_hash =
        //     std::env::var("ACCOUNT_HASH").expect("ACCOUNT_HASH environment var not set");
        format!(
            "https://imagedelivery.net/{}/{}/format=webp",
            account_hash, self.image_id
        )
        .parse()
        .unwrap()
    }

    pub fn to_response(&self, url_base: Url) -> PageResponse {
        // Compute image url for page
        let image_url = {
            let mut url = url_base;
            url.path_segments_mut().unwrap().clear().extend([
                "page",
                &self.page_number.to_string(),
                "image",
            ]);
            url
        };

        // Create response
        PageResponse {
            chapter_number: self.chapter_number,
            page_number: self.page_number,
            name: self.name.clone(),
            image_url: image_url.to_string(),
        }
    }
}

impl Chapter {
    pub async fn get_by_number(
        store: &KvStore,
        chapter_number: usize,
    ) -> Result<Option<Self>, KvError> {
        // for now I will just fetch every page and then filter for pages in this chapter
        // but we may want to have that indexed or cached or something
        let page_keys = store.list().prefix("page-".to_owned()).execute().await?;
        let pages: Vec<Page> = try_join_all(
            page_keys
                .keys
                .iter()
                .map(|key| store.get(&key.name).json::<Page>()),
        )
        .await?
        .into_iter()
        .flatten()
        .filter(|page| page.chapter_number == chapter_number && page.is_published)
        .collect();

        match pages.len() {
            0 => Ok(None),
            _ => Ok(Some(Self {
                pages,
                chapter_number,
            })),
        }
    }

    pub async fn get_all(store: &KvStore) -> Result<Vec<Self>, KvError> {
        // Fetch all pages
        let page_keys = store.list().prefix("page-".to_owned()).execute().await?;
        let pages: Vec<Page> = try_join_all(
            page_keys
                .keys
                .iter()
                .map(|key| store.get(&key.name).json::<Page>()),
        )
        .await?
        .into_iter()
        .flatten()
        .filter(|page| page.is_published)
        .collect();

        // Group pages into chapters
        let mut chapters: HashMap<usize, Vec<Page>> = HashMap::new();
        for page in pages {
            chapters.entry(page.chapter_number).or_default().push(page);
        }

        // Return chapters as a vec
        Ok(chapters
            .into_iter()
            .map(|(chapter_number, pages)| Self {
                chapter_number,
                pages,
            })
            .collect())
    }

    pub fn to_response(&self, url_base: Url) -> ChapterResponse {
        ChapterResponse {
            chapter_number: self.chapter_number,
            pages: self
                .pages
                .clone()
                .into_iter()
                .map(|page| page.to_response(url_base.clone()))
                .collect(),
        }
    }
}
