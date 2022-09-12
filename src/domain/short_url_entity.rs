use super::base::{gen_id, BaseEntity};

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
    pub fn new(url: String) -> Self {
        let id = gen_id();

        Self { id, url }
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}
