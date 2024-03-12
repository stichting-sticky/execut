use axum::{
    body::{self, Body},
    extract::State, Json,
};
use csv::Reader;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::Claims, users::Role, Context, Error, Result};

use super::{Badge, Token};

#[derive(Debug, Deserialize)]
struct Record {
    role: Role,
    badge: Badge,
    name: String,
    mail: String,
    token: Token,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    users: Vec<Uuid>,
}

pub async fn populate(claims: Claims, State(state): State<Context>, body: Body) -> Result<Json<Response>> {
    let Context { pool, .. } = state;
    let Claims { role, .. } = claims;

    // Only admins can populate the database
    if role != Role::Admin {
        return Err(Error::Unauthorized);
    }

    let buf = body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| Error::Internal)?;

    let mut reader = Reader::from_reader(buf.as_ref());

    let mut users = Vec::new();

    for result in reader.deserialize() {
        let record = result.map_err(|_| Error::InvalidRequest)?;

        let Record {
            role,
            badge,
            name,
            mail,
            token,
        } = record;

        let mut transaction = pool.begin().await.map_err(|_| Error::Internal)?;

        let id = sqlx::query!(
            "   insert into users ( role, name, mail )
   values ( $1, $2, $3 )
returning id",
            role as Role,
            name,
            mail,
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| Error::Internal)?
        .id;

        let Badge(badge) = badge;

        sqlx::query!(
            "insert into badges ( user_id, badge )
values ( $1, $2 )",
            id,
            badge,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::DuplicateBadge)?;

        let Token(token) = token;

        sqlx::query!(
            "insert into tokens ( user_id, token )
values ( $1, $2 )",
            id,
            token,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::DuplicateToken)?;

        transaction.commit().await.map_err(|_| Error::Internal)?;

        users.push(id);
    }

    Ok(Json(Response { users }))
}
