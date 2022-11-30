mod migration_files;

pub mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("./src/migration_files");
}

// #[derive(Debug, Error)]
// pub enum ConfigError {
//     #[error("Reason: {0}")]
//     InvalidDatabaseUrl(String),
// }
