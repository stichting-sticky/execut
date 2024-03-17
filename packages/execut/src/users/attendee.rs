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
pub struct Attendee {
    #[serde(skip)]
    pub user_id: Uuid,
    pub linkedin: Option<String>,
    pub study: Option<String>,
    pub degree: Option<String>,
    pub institution: Option<String>,
    pub graduation_year: Option<String>,
}

impl Attendee {
    pub fn new(
        user_id: Uuid,
        linkedin: Option<String>,
        study: Option<String>,
        degree: Option<String>,
        institution: Option<String>,
        graduation_year: Option<String>,
    ) -> Self {
        Self {
            user_id,
            linkedin,
            study,
            degree,
            institution,
            graduation_year,
        }
    }
}

#[derive(Deserialize)]
struct Record {
    name: String,
    mail: String,
    linkedin: Option<String>,
    study: Option<String>,
    degree: Option<String>,
    institution: Option<String>,
    graduation_year: Option<String>,
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

    for result in reader.deserialize::<Record>() {
        let record: Record = result.map_err(|_| Error::InvalidRequest)?;

        let id = Uuid::now_v7();

        let Record {
            name,
            mail,
            linkedin,
            study,
            degree,
            institution,
            graduation_year,
            badge,
            token,
        } = record;

        let mut transaction = pool.begin().await.map_err(|_| Error::Internal)?;

        sqlx::query!(
            "insert into users ( id, name, mail )
             values ( $1, $2, $3 )",
            id,
            name,
            mail,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::Internal)?;

        sqlx::query!(
            "insert into attendees
                  ( user_id
                  , linkedin
                  , study
                  , degree
                  , institution
                  , graduation_year
                  )
             values ( $1, $2, $3, $4, $5, $6 )",
            id,
            linkedin,
            study,
            degree,
            institution,
            graduation_year,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::Internal)?;

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
