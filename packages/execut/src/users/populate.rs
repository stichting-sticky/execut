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

#[derive(Debug, Deserialize)]
struct Record {
    role: Role,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    mail: Option<String>,
    #[serde(default)]
    linkedin: Option<String>,
    #[serde(default)]
    study: Option<String>,
    #[serde(default)]
    degree: Option<String>,
    #[serde(default)]
    institution: Option<String>,
    #[serde(default)]
    graduation_year: Option<String>,
    #[serde(default)]
    company: Option<String>,
    badge: Badge,
    token: Token,
}

#[derive(Serialize)]
pub struct Response {
    users: Vec<Uuid>,
}

pub async fn populate(
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
            role,
            name,
            mail,
            linkedin,
            study,
            degree,
            institution,
            graduation_year,
            company,
            badge,
            token,
        } = record;

        let mut transaction = pool.begin().await.map_err(|_| Error::Internal)?;

        let id = sqlx::query!(
            "insert into users ( role ) values ( $1 ) returning id",
            role as Role,
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| Error::Internal)?
        .id;

        if role == Role::Attendee {
            sqlx::query!(
                "insert into attendees ( user_id, name, mail, linkedin, study, degree, institution, graduation_year )
values ( $1, $2, $3, $4, $5, $6, $7, $8 )",
                    id,
                    name,
                    mail,
                    linkedin,
                    study,
                    degree,
                    institution,
                    graduation_year,
                )
                .execute(&mut *transaction)
                .await
                .map_err(|_| Error::Internal)?;
        }

        if role == Role::Exhibitor {
            sqlx::query!(
                "insert into exhibitors ( user_id, company ) values ( $1, $2 )",
                id,
                company,
            )
            .execute(&mut *transaction)
            .await
            .map_err(|_| Error::Internal)?;
        }

        sqlx::query!(
            "insert into badges ( user_id, badge ) values ( $1, $2 )",
            id,
            badge as Badge,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| Error::DuplicateBadge)?;

        sqlx::query!(
            "insert into tokens ( user_id, token ) values ( $1, $2 )",
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
