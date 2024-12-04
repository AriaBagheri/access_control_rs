use rust_iam::{EngineTrait, PolicyCollection};
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, FromRow, Eq)]
pub struct RoleRecord<Engine: EngineTrait> {
    pub name: String,
    pub inline_policies: PolicyCollection<Engine>,
}

impl<Engine: EngineTrait> Unpin for RoleRecord<Engine> {}

