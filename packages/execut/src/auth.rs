pub mod claims;
pub mod keys;

use axum::{extract::State, Json};
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

pub use {claims::Claims, keys::Keys};

use crate::{
    users::{Badge, Role, Token},
    Context, Error, Result,
};

#[derive(Debug, Deserialize)]
pub struct Payload {
    badge: Badge,
    token: Token,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    token: String,
}

pub async fn authorize(
    State(state): State<Context>,
    Json(payload): Json<Payload>,
) -> Result<Json<Response>> {
    let Context { pool, keys } = state;
    let Payload { badge, token } = payload;

    let Badge(badge) = badge;
    let Token(token) = token;

    let mut transaction = pool.begin().await.map_err(|_| Error::Internal)?;

    let (subject, role) = sqlx::query!(
        "select users.id
     , users.role as \"role: Role\"
  from badges
     , tokens
     , users
 where badges.badge = $1
   and badges.user_id = users.id
   and tokens.token = $2
   and tokens.user_id = users.id
   and tokens.is_used = 'false'",
        badge,
        token,
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?
    .map(|row| (row.id, row.role))
    .ok_or_else(|| Error::WrongCredentials)?;

    let claims = Claims {
        subject,
        expires_at: 2000000000, // May 2033
        role,
    };

    sqlx::query!(
        "update tokens
   set is_used = 'true'
 where user_id = $1
   and token = $2",
        subject,
        token,
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| Error::WrongCredentials)?;

    transaction.commit().await.map_err(|_| Error::Internal)?;

    let token = encode(&Header::default(), &claims, &keys.encoding).map_err(|_| Error::Internal)?;

    Ok(Json(Response { token }))
}
