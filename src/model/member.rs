use async_graphql::*;

#[derive(SimpleObject)]
pub struct Member {
    pub(crate) name: String,
    pub(crate) age: u8,
}
