use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug)]
pub struct SslUsed {
    ssl_used: bool,
}

impl Tabular for SslUsed {
    const FILE_NAME: &'static str = "ssl_used";

    fn new(row: &Row) -> Self {
        SslUsed {
            ssl_used: row.get::<_, Option<bool>>(0).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.ssl_used.to_string()]
    }

    fn headers() -> prettytable::Row {
        row!["ssl_used"]
    }
}
