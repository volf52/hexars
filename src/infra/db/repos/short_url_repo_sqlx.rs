use crate::domain::{
    base::BaseEntity,
    short_url_entity::{ShortUrl, ShortUrlRepo},
};

#[derive(Default)]
pub struct ShortUrlRepoSqlx;

#[async_trait::async_trait]
impl ShortUrlRepo for ShortUrlRepoSqlx {
    async fn fetch_all(&self) -> Vec<ShortUrl> {
        let pool = crate::infra::db::POOL.get().unwrap();

        let res = sqlx::query!(
            "
            SELECT id, url from short_urls
            "
        )
        .fetch_all(pool)
        .await
        .unwrap();

        let mut results = Vec::with_capacity(res.len());

        for r in res {
            let ent = ShortUrl::from(r.id, r.url);
            results.push(ent);
        }

        results
    }

    async fn insert(&self, ent: &ShortUrl) -> ShortUrl {
        let pool = crate::infra::db::POOL.get().unwrap();

        sqlx::query!(
            "
            INSERT INTO short_urls (id, url) VALUES ($1, $2)
            ",
            ent.id(),
            ent.url()
        )
        .execute(pool)
        .await
        .unwrap();

        ent.clone()
    }

    // fn fetch_by_id(&self, id: String) -> anyhow::Result<ShortUrl> {
    //     todo!()
    // }
}
