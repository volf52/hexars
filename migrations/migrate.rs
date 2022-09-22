use hexars::{db::migrations, errors::ConfigError};
use refinery::config::{Config, ConfigDbType};
use url::Url;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // let migrations_path = "migrations";
    // println!("cargo:rerun-if-changed={migrations_path}");
    color_eyre::install()?;

    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    // Maybe use dsn crate
    let parsed_db_url = Url::parse(&db_url)?;

    let db_host = parsed_db_url
        .host_str()
        .ok_or(ConfigError::InvalidDatabaseUrl)?;
    let db_port = parsed_db_url
        .port()
        .map(|e| e.to_string())
        .ok_or(ConfigError::InvalidDatabaseUrl)?;

    let db_user = match parsed_db_url.username() {
        "" => Err(ConfigError::InvalidDatabaseUrl),
        val => Ok(val),
    }?;

    let db_pass = parsed_db_url
        .password()
        .ok_or(ConfigError::InvalidDatabaseUrl)?;
    let db_name = parsed_db_url
        .path_segments()
        .ok_or(ConfigError::InvalidDatabaseUrl)
        .map(|mut e| e.next())?
        .ok_or(ConfigError::InvalidDatabaseUrl)?;

    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_user(db_user)
        .set_db_pass(db_pass)
        .set_db_host(db_host)
        .set_db_port(&db_port)
        .set_db_name(db_name);

    let runner = migrations::runner();

    println!("Starting migrations...");

    let report = runner.run_async(&mut conn).await?;

    for migration in report.applied_migrations() {
        println!(
            "Migration Applied - Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    println!("Finished migrations...");

    Ok(())
}
