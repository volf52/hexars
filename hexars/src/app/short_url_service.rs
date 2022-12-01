use crate::domain::short_url_entity::{ShortUrl, ShortUrlRepoBox};

use super::result::ServiceResult;

#[derive(Debug)]
pub struct ShortUrlServ {
    repo: ShortUrlRepoBox,
}

impl ShortUrlServ {
    #[must_use]
    pub fn new(repo: ShortUrlRepoBox) -> Self {
        Self { repo }
    }

    pub async fn create_short_url(&self, url: String) -> ServiceResult<ShortUrl> {
        let ent = ShortUrl::new(url)?;

        let _insert_res = self.repo.insert(&ent).await?;

        Ok(ent)
    }

    pub async fn get_all_urls(&self) -> ServiceResult<Vec<ShortUrl>> {
        let repos = self.repo.fetch_all().await?;

        Ok(repos)
    }
}
