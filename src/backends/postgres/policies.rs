use rust_iam::{EngineTrait, Statement};
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, FromRow, Eq)]
pub struct PolicyRecord<Engine: EngineTrait> {
    pub name: String,
    pub statements: Vec<Statement<Engine>>,
}

impl<Engine: EngineTrait> Unpin for PolicyRecord<Engine> {}

