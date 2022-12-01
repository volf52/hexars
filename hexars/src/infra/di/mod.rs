use crate::{
    app::short_url_service::ShortUrlServ, infra::db::repos::short_url_repo_sqlx::ShortUrlRepoSqlx,
};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Container {
    pub short_url: ShortUrlServ,
}

pub static CONTAINER: OnceCell<Container> = OnceCell::new();

impl Container {
    fn new() -> Self {
        // ---------- Repositories --------------
        let short_url_repo = ShortUrlRepoSqlx::default();

        // ---------- Services -----------
        let short_url_serv = ShortUrlServ::new(Box::new(short_url_repo));

        Self {
            short_url: short_url_serv,
        }
    }

    pub fn init() {
        let container = Self::new();

        CONTAINER
            .set(container)
            .expect("DI Container already initialized");
    }
}

macro_rules! get_container {
    () => {
        crate::infra::di::CONTAINER
            .get()
            .expect("CONTAINER not initialized")
    };
}

pub(crate) use get_container;
