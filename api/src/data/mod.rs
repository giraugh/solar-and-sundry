pub mod dto;
pub mod page;

use crate::DbClient;

pub trait Model: Sized {
    type Id;

    fn table_sql() -> &'static str;

    async fn get(db: DbClient, id: Self::Id) -> Result<Self, anyhow::Error>;
    async fn get_all(db: DbClient) -> Result<Vec<Self>, anyhow::Error>;
    async fn upsert(db: DbClient, id: Self::Id) -> Result<(), anyhow::Error>;
    async fn delete(db: DbClient, id: Self::Id) -> Result<(), anyhow::Error>;

    async fn create_table(db: DbClient) -> Result<(), anyhow::Error> {
        db.lock().await.execute(Self::table_sql()).await?;
        Ok(())
    }
}

#[macro_export]
macro_rules! create_tables {
    ($db: expr; $t: tt) => {{
        use $crate::data::Model;
        $t::create_table($db.clone()).await.unwrap();
    }};

    ($db: expr; $t: tt, $($ot:tt),+) => {{
        use $crate::data::Model;
        $t::create_table($db.clone()).await.unwrap();
        create_tables!($db; $($ot),+);
    }}
}
