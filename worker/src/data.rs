use serde::{Deserialize, Serialize};
use worker::kv::{KvError, KvStore};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_id: String,
    pub is_published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePage {
    pub page_number: usize,
    pub chapter_number: usize,
    pub image_id: String,
}

impl From<CreatePage> for Page {
    fn from(val: CreatePage) -> Self {
        Page {
            page_number: val.page_number,
            chapter_number: val.chapter_number,
            image_id: val.image_id,
            is_published: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub chapter_number: usize,
    pub pages: Vec<Page>,
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

    pub async fn get_image(&self) -> Option<()> {
        todo!()
    }
}

impl Chapter {
    pub async fn get_by_number(
        &self,
        store: &KvStore,
        chapter_number: usize,
    ) -> Result<Option<Self>, KvError> {
        // TODO: for now I will just fetch every page and then filter for pages in this chapter
        // but we may want to have that indexed or cached or something
        // let page_keys = store.list().prefix("page-").execute().await?;
        // TODO: we gonna have to pull all the values individually rip lol
        todo!()
    }
}
