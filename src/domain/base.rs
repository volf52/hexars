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
