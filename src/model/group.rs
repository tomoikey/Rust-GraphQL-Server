use crate::model::member::Member;
use async_graphql::*;

#[derive(SimpleObject)]
pub struct Group {
    pub(crate) name: String,
    pub(crate) members: Vec<Member>,
}
