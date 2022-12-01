use crate::domain::{
    base::{BaseEntity, BaseRepo},
    short_url_entity::{ShortUrl, ShortUrlRepo},
};

#[derive(Default, Debug)]
pub struct ShortUrlRepoSqlx;

// Explore request scoped connections - https://github.com/tokio-rs/axum/blob/6c133be5b7561e0de74d2b94dd2b17607ed99028/examples/sqlx-postgres/src/main.rs

#[async_trait::async_trait]
impl BaseRepo for ShortUrlRepoSqlx {
    type Entity = ShortUrl;

    async fn fetch_all(&self) -> Vec<Self::Entity> {
        let pool = crate::infra::db::POOL.get().expect("POOL must be there");

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

    async fn insert(&self, ent: &Self::Entity) -> Self::Entity {
        let pool = crate::infra::db::POOL.get().expect("POOL not set");
        let id = ent.id();
        let url = ent.url();

        sqlx::query!(
            "
            INSERT INTO short_urls (id, url) VALUES ($1, $2)
            ",
            id,
            url
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

impl ShortUrlRepo for ShortUrlRepoSqlx {}
