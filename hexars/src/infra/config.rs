use once_cell::sync::OnceCell;

struct Config {
    db_url: String,
}

pub static APP_CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init_config() -> color_eyre::Result<()> {
    let db_url = "".to_string();

    let cfg = Config { db_url };

    APP_CONFIG.set(cfg);

    Ok(())
}
