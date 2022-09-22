use crate::domain::short_url_entity::{ShortUrl, ShortUrlRepo};

type Repo = Box<dyn ShortUrlRepo<Entity = ShortUrl> + Sync + Send>;

pub struct ShortUrlServ {
    repo: Repo,
}

impl ShortUrlServ {
    #[must_use]
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }

    pub async fn create_short_url(&self, url: String) -> anyhow::Result<ShortUrl> {
        let ent = ShortUrl::new(url)?;

        let _insert_res = self.repo.insert(&ent).await;

        Ok(ent)
    }

    pub async fn get_all_urls(&self) -> Vec<ShortUrl> {
        self.repo.fetch_all().await
    }
}
