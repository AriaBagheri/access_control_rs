use ss::types::{Timestamp, Username};
use uuid::Uuid;
use crate::backends::{POSTGRES_DB};

#[derive(sqlx::FromRow, Debug)]
pub struct UserToken {
    pub token: Uuid,
    pub username: Username,
    pub token_type: String,
    pub expires_at: Timestamp,
    pub created_at: Timestamp,
    pub last_used_at: Option<Timestamp>,
    pub is_revoked: bool,
}

impl UserToken {
    pub async fn from_token(token: &Uuid) -> Result<Self, sqlx::Error> {
        let db = &*POSTGRES_DB.get().await;
        sqlx::query_as::<_, UserToken>(
            "SELECT * FROM user_tokens WHERE token=$1")
            .bind(token.to_string())
            .fetch_one(db)
            .await
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//
// }