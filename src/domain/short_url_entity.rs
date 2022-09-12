use super::base::{gen_id, BaseEntity};

#[derive(Debug)]
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

        ShortUrl::validate_url(&url)?;

        Ok(Self { id, url })
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    fn validate_url(url: &str) -> Result<(), ShortUrlError> {
        url::Url::parse(url)
            .map(|_| ())
            .map_err(|e| ShortUrlError::InvalidUrl(url.to_string()))
    }
}

// ---------- Exceptions --------
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ShortUrlError {
    #[error("URL is not valid <{0}>")]
    InvalidUrl(String),
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
