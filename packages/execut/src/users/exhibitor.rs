use axum::{
    body::{self, Body},
    extract::State,
    Json,
};
use csv::Reader;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::Claims,
    users::{Badge, Role, Token},
    Context, Error, Result,
};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exhibitor {
    #[serde(skip)]
    pub user_id: Uuid,
    pub company: String,
}

impl Exhibitor {
    pub fn new(user_id: Uuid, company: String) -> Self {
        Self { user_id, company }
    }
}

#[derive(Deserialize)]
struct Record {
    name: String,
    mail: String,
    company: String,
    badge: Badge,
    token: Token,
}

#[derive(Serialize)]
pub struct Response {
    users: Vec<Uuid>,
}

pub async fn seed(
    claims: Claims,
    State(state): State<Context>,
    body: Body,
) -> Result<Json<Response>> {
    let Claims { role, .. } = claims;

    let Context { pool, .. } = state;

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
            name,
            mail,
            company,
            badge,
            token,
        } = record;

        let mut transaction = pool.begin().await.map_err(|_| Error::Internal)?;

        let id = match sqlx::query!(
            "select id
               from users as u
                  , exhibitors as e
              where u.id = e.user_id
                and e.company = $1",
            company,
        )
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| Error::Internal)?
        .map(|row| row.id)
        {
            None => {
                let id = Uuid::now_v7();

                sqlx::query!(
                    "insert into users ( id, role, name, mail )
                     values ( $1, $2, $3, $4 )",
                    id,
                    Role::Exhibitor as Role,
                    name,
                    mail,
                )
                .execute(&mut *transaction)
                .await
                .map_err(|_| Error::Internal)?;

                sqlx::query!(
                    "insert into exhibitors ( user_id, company )
                     values ( $1, $2 )",
                    id,
                    company,
                )
                .execute(&mut *transaction)
                .await
                .map_err(|_| Error::Internal)?;

                id
            }
            Some(id) => id,
        };

        sqlx::query!(
            "insert into badges ( user_id, badge )
             values ( $1, $2 )",
            id,
            badge as Badge,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::DuplicateBadge)?;

        sqlx::query!(
            "insert into tokens ( user_id, token )
             values ( $1, $2 )",
            id,
            token as Token,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::DuplicateToken)?;

        transaction.commit().await.map_err(|_| Error::Internal)?;

        users.push(id);
    }

    Ok(Json(Response { users }))
}
