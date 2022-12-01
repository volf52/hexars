use nanoid::nanoid;

#[derive(Debug, thiserror::Error)]
#[error("DatabaseError: {0}")]
pub struct DatabaseError(String);

pub(super) fn gen_id() -> String {
    nanoid!(6)
}

pub trait BaseEntity {
    fn id(&self) -> String;
}

#[async_trait::async_trait]
pub trait BaseRepo: std::fmt::Debug {
    type Entity: BaseEntity;

    async fn fetch_all(&self) -> Vec<Self::Entity>;
    async fn insert(&self, ent: &Self::Entity) -> Self::Entity;
}
