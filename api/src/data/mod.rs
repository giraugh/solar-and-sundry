pub mod chapter;
pub mod dto;
pub mod page;

use crate::DbRef;

pub trait Model: Sized {
    fn table_sql() -> &'static str;

    async fn create_table(db: DbRef) -> Result<(), anyhow::Error> {
        db.lock().await.execute(Self::table_sql(), ()).await?;
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

pub struct RowsIter(libsql::Rows);

pub trait RowsIterExt {
    fn iter(self) -> RowsIter;
}

impl RowsIterExt for libsql::Rows {
    fn iter(self) -> RowsIter {
        RowsIter(self)
    }
}

impl Iterator for RowsIter {
    type Item = libsql::Result<libsql::Row>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().transpose()
    }
}
