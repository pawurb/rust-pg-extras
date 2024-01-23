use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct TableIndexesSize {
    table: String,
    index_size: String,
}

impl Tabular for TableIndexesSize {
    fn new(row: &Row) -> Self {
        Self {
            table: row.get::<_, Option<String>>(0).unwrap_or_default(),
            index_size: row.get::<_, Option<String>>(1).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.table, self.index_size]
    }

    fn headers() -> prettytable::Row {
        row!["table", "index_size"]
    }
}
