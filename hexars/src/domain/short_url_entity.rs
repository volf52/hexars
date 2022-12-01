use super::base::{gen_id, BaseEntity, BaseRepo};

#[derive(Debug, Clone)]
pub struct ShortUrl {
    id: String,
    url: String,
}

impl BaseEntity for ShortUrl {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl ShortUrl {
    pub fn new(url: String) -> Result<Self, ShortUrlError> {
        let id = gen_id();

        Self::validate_url(&url)?;

        Ok(Self { id, url })
    }

    pub const fn from(id: String, url: String) -> Self {
        Self { id, url }
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    fn validate_url(url: &str) -> Result<(), ShortUrlError> {
        url::Url::parse(url)
            .map(|_| ())
            .map_err(|_| ShortUrlError::InvalidUrl(url.to_string()))
    }
}

// ---------- Exceptions --------
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ShortUrlError {
    #[error("URL is not valid <{0}>")]
    InvalidUrl(String),
    #[error("Error hydrating from data source")]
    HydrationError,
}

// ----- Repo ------
// #[async_trait::async_trait]
// pub trait ShortUrlRepo {
//     async fn fetch_all(&self) -> Vec<ShortUrl>;
//     // fn fetch_by_id(&self, id: String) -> anyhow::Result<ShortUrl>;
//     async fn insert(&self, ent: &ShortUrl) -> ShortUrl;
// }
#[async_trait::async_trait]
pub trait IShortUrlRepo: BaseRepo {}

pub type ShortUrlRepo = Box<dyn IShortUrlRepo<Entity = ShortUrl> + Sync + Send>;

impl std::fmt::Debug for ShortUrlRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    mod short_url {
        use crate::domain::base::BaseEntity;
        use crate::domain::short_url_entity::{ShortUrl, ShortUrlError};

        #[test]
        fn new_with_invalid_url() {
            let url = "abc".to_string();

            let n_result = ShortUrl::new(url.clone());

            assert!(n_result.is_err());

            assert_eq!(n_result.unwrap_err(), ShortUrlError::InvalidUrl(url));
        }

        #[test]
        fn new_with_valid_url() {
            let url = "https://www.google.com".to_string();

            let result = ShortUrl::new(url.clone());

            assert!(result.is_ok());

            let ent = result.unwrap();

            assert_eq!(ent.id().len(), 6);
            assert_eq!(ent.url(), url);
        }
    }
}
