use crate::{
    app::short_url_service::ShortUrlServ, db::repos::short_url_repo_sqlx::ShortUrlRepoSqlx,
};

pub struct Container {
    pub short_url: ShortUrlServ,
}

impl Container {
    #[must_use]
    pub fn init() -> Self {
        // ---------- Repositories --------------
        let short_url_repo = ShortUrlRepoSqlx::default();

        // ---------- Services -----------
        let short_url_serv = ShortUrlServ::new(Box::new(short_url_repo));

        Self {
            short_url: short_url_serv,
        }
    }
}
