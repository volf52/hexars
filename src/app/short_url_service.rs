use crate::domain::short_url_entity::{ShortUrl, ShortUrlRepo};

pub struct ShortUrlServ<'a> {
    repo: &'a dyn ShortUrlRepo,
}

impl<'a> ShortUrlServ<'a> {
    pub fn new(repo: &'a dyn ShortUrlRepo) -> Self {
        Self { repo }
    }

    pub async fn create_short_url(&self, url: String) -> anyhow::Result<ShortUrl> {
        let ent = ShortUrl::new(url)?;

        self.repo.insert(&ent).await;

        Ok(ent)
    }

    pub async fn get_all_urls(&self) -> Vec<ShortUrl> {
        self.repo.fetch_all().await
    }
}
