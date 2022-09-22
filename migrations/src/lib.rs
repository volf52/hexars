mod migration_files;

use refinery::embed_migrations;

embed_migrations!("src/migration_files");

// #[derive(Debug, Error)]
// pub enum ConfigError {
//     #[error("Reason: {0}")]
//     InvalidDatabaseUrl(String),
// }
