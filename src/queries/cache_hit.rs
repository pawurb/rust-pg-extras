use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::types::BigDecimal;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct CacheHit {
    pub name: String,
    pub ratio: BigDecimal,
}

impl Query for CacheHit {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            ratio: row.try_get("ratio").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.ratio]
    }

    fn headers() -> prettytable::Row {
        row!["name", "ratio"]
    }

    fn read_file() -> String {
        include_str!("../sql/cache_hit.sql").to_string()
    }
}
