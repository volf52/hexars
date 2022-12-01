use nanoid::nanoid;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("DatabaseError: {0}")]
    DatabaseError(String),
}

pub type RepoResult<T> = std::result::Result<T, RepoError>;

#[inline]
pub(super) fn gen_id() -> String {
    nanoid!(6)
}

pub trait BaseEntity {
    fn id(&self) -> String;
}

#[async_trait::async_trait]
pub trait BaseRepo: std::fmt::Debug {
    type Entity: BaseEntity;

    async fn fetch_all(&self) -> RepoResult<Vec<Self::Entity>>;
    // todo: maybe change return to unit
    async fn insert(&self, ent: &Self::Entity) -> RepoResult<Self::Entity>;
}
