use pg_extras::{
    all_locks, bloat, blocking, buffercache_stats, buffercache_usage, cache_hit, calls,
    connections, db_settings, duplicate_indexes, extensions, index_cache_hit, index_scans,
    index_size, index_usage, indexes, locks, long_running_queries, mandelbrot, null_indexes,
    outliers, records_rank, render_table, seq_scans, ssl_used, table_cache_hit, table_index_scans,
    table_indexes_size, table_size, tables, total_index_size, total_table_size, unused_indexes,
    vacuum_stats, PgExtrasError,
};

use std::{env, fmt};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1);

    match execute(command).await {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

async fn execute(command: Option<&String>) -> Result<(), PgExtrasCmdError> {
    if let Some(command) = command {
        match command.as_str() {
            "all_locks" => {
                render_table(all_locks().await?);
            }
            "bloat" => {
                render_table(bloat().await?);
            }
            "blocking" => {
                render_table(blocking(None).await?);
            }
            "buffercache_stats" => {
                render_table(buffercache_stats().await?);
            }
            "buffercache_usage" => {
                render_table(buffercache_usage().await?);
            }
            "cache_hit" => {
                render_table(cache_hit(None).await?);
            }
            "calls" => {
                render_table(calls(None).await?);
            }
            "connections" => {
                render_table(connections().await?);
            }
            "db_settings" => {
                render_table(db_settings().await?);
            }
            "duplicate_indexes" => {
                render_table(duplicate_indexes().await?);
            }
            "extensions" => {
                render_table(extensions().await?);
            }
            "index_cache_hit" => {
                render_table(index_cache_hit(None).await?);
            }
            "index_scans" => {
                render_table(index_scans(None).await?);
            }
            "index_size" => {
                render_table(index_size().await?);
            }
            "index_usage" => {
                render_table(index_usage(None).await?);
            }
            "indexes" => {
                render_table(indexes().await?);
            }
            "locks" => {
                render_table(locks().await?);
            }
            "long_running_queries" => {
                render_table(long_running_queries().await?);
            }
            "mandelbrot" => {
                render_table(mandelbrot().await?);
            }
            "null_indexes" => {
                render_table(null_indexes(None).await?);
            }
            "outliers" => {
                render_table(outliers().await?);
            }
            "records_rank" => {
                render_table(records_rank(None).await?);
            }
            "seq_scans" => {
                render_table(seq_scans(None).await?);
            }
            "ssl_used" => {
                render_table(ssl_used().await?);
            }
            "table_cache_hit" => {
                render_table(table_cache_hit().await?);
            }
            "table_index_scans" => {
                render_table(table_index_scans(None).await?);
            }
            "table_indexes_size" => {
                render_table(table_indexes_size(None).await?);
            }
            "table_size" => {
                render_table(table_size().await?);
            }
            "tables" => {
                render_table(tables(None).await?);
            }
            "total_index_size" => {
                render_table(total_index_size().await?);
            }
            "total_table_size" => {
                render_table(total_table_size().await?);
            }
            "unused_indexes" => {
                render_table(unused_indexes(None).await?);
            }
            "vacuum_stats" => {
                render_table(vacuum_stats().await?);
            }
            _ => {
                return Err(PgExtrasCmdError::UnknownCommand(command.to_string()));
            }
        }
    } else {
        return Err(PgExtrasCmdError::MissingCommand);
    }

    Ok(())
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PgExtrasCmdError {
    UnknownCommand(String),
    MissingCommand,
    Other(PgExtrasError),
}

impl fmt::Display for PgExtrasCmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let check_command_text =
            "Check https://github.com/pawurb/pg-extras-rs for list of available commands.";

        let msg = match self {
            Self::UnknownCommand(command) => {
                format!("Unknown command '{}'. {}", command, check_command_text)
            }
            Self::MissingCommand => {
                format!("Missing command. {}", check_command_text)
            }
            Self::Other(error) => format!("{}", error),
        };
        write!(f, "{}", msg)
    }
}

impl From<pg_extras::PgExtrasError> for PgExtrasCmdError {
    fn from(error: pg_extras::PgExtrasError) -> Self {
        Self::Other(error)
    }
}

impl std::error::Error for PgExtrasCmdError {}

#[test]
fn normal_types() {
    fn is_normal<T: Sized + Send + Sync + Unpin>() {}
    is_normal::<PgExtrasCmdError>();
}
