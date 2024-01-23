use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct IndexScans {
    schemaname: String,
    tablename: String,
    indexname: String,
    index_size: String,
    index_scans: i64,
}

impl Tabular for IndexScans {
    fn new(row: &Row) -> Self {
        Self {
            schemaname: row.get::<_, Option<String>>(0).unwrap_or_default(),
            tablename: row.get::<_, Option<String>>(1).unwrap_or_default(),
            indexname: row.get::<_, Option<String>>(2).unwrap_or_default(),
            index_size: row.get::<_, Option<String>>(3).unwrap_or_default(),
            index_scans: row.get::<_, Option<i64>>(4).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.schemaname,
            self.tablename,
            self.indexname,
            self.index_size,
            self.index_scans.to_string()
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "schemaname",
            "tablename",
            "indexname",
            "index_size",
            "index_scans"
        ]
    }
}
