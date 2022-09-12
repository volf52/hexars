use nanoid::nanoid;

pub(super) fn gen_id() -> String {
    nanoid!(6)
}

pub trait BaseEntity {
    fn id(&self) -> String;
}
