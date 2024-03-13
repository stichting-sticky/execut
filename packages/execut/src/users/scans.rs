use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::Claims, users::Role, Context, Error, Result};

#[derive(Deserialize, Serialize)]
pub struct Scan {
    id: Uuid,
    subject_id: Uuid,
    initiator_id: Uuid,
    is_expunged: bool,
    created_at: DateTime<Utc>,
    modified_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct Scans {
    active: Vec<Scan>,
    passive: Vec<Scan>,
}

pub async fn get_scans(claims: Claims, State(context): State<Context>) -> Result<Json<Scans>> {
    let Claims { subject, role, .. } = claims;

    let Context { pool, .. } = context;

    let mut transaction = pool.begin().await.map_err(|_| Error::Internal)?;

    if role != Role::Exhibitor {
        return Err(Error::Unauthorized);
    }

    let active = sqlx::query_as!(
        Scan,
        "  select *
    from scans
   where initiator_id = $1
     and is_expunged = 'false'
order by created_at desc",
        subject,
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?;

    let passive = sqlx::query_as!(
        Scan,
        "  select *
    from scans
   where subject_id = $1
     and is_expunged = 'false'
order by created_at desc",
        subject,
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?;

    transaction.commit().await.map_err(|_| Error::Internal)?;

    Ok(Json(Scans { active, passive }))
}

pub async fn scan_badge(
    claims: Claims,
    State(context): State<Context>,
    Path(badge): Path<Uuid>,
) -> Result<Json<Scan>> {
    let Claims { subject, role, .. } = claims;

    let (initiator_id, initiator_role) = (subject, role);

    let Context { pool, .. } = context;

    let mut transaction = pool.begin().await.map_err(|_| Error::Internal)?;

    let (subject_id, subject_role) = sqlx::query!(
        "select users.id, users.role as \"role: Role\"
  from badges, users
 where badges.badge = $1
   and badges.user_id = users.id",
        badge,
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?
    .map(|res| (res.id, res.role))
    .ok_or_else(|| Error::UnknownBadge)?;

    if &initiator_id == &subject_id {
        return Err(Error::SelfScan);
    }

    match (&initiator_role, &subject_role) {
        (Role::Attendee, Role::Exhibitor) => (),
        (Role::Exhibitor, Role::Attendee) => (),
        _ => return Err(Error::Unauthorized),
    }

    let (id, created_at, modified_at) = sqlx::query!(
        "   insert into scans ( initiator_id, subject_id )
   values ( $1, $2 )
returning id, created_at, modified_at",
        initiator_id,
        subject_id,
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::DuplicateScan)?
    .map(|res| (res.id, res.created_at, res.modified_at))
    .ok_or_else(|| Error::UnknownUser)?;

    transaction.commit().await.map_err(|_| Error::Internal)?;

    Ok(Json(Scan {
        id,
        subject_id,
        initiator_id,
        is_expunged: false,
        created_at,
        modified_at,
    }))
}
