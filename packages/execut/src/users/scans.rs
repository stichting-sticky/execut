use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::Claims,
    users::{Attendee, Exhibitor, Role, User},
    Context, Error, Result,
};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scan {
    pub id: Uuid,
    #[serde(skip)]
    pub subject_id: Uuid,
    pub subject: Option<User>,
    #[serde(skip)]
    pub initiator_id: Uuid,
    pub initiator: Option<User>,
    pub is_expunged: bool,
    pub scanned_at: DateTime<Utc>,
}

impl Scan {
    pub fn new(
        id: Uuid,
        subject_id: Uuid,
        subject: Option<User>,
        initiator_id: Uuid,
        initiator: Option<User>,
        is_expunged: bool,
        scanned_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            subject_id,
            subject,
            initiator_id,
            initiator,
            is_expunged,
            scanned_at,
        }
    }
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

    let initiator = sqlx::query!(
        "select u.name
              , u.mail
              , e.company
           from exhibitors as e
              , users as u
          where u.id = $1
            and u.id = e.user_id",
        subject,
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?
    .map(|res| {
        let exhibitor = Exhibitor::new(subject, res.company);

        User::new(subject, role, res.name, res.mail, None, Some(exhibitor))
    })
    .ok_or_else(|| Error::UnknownUser)?;

    let active = sqlx::query!(
        "select s.id
              , s.subject_id
              , u.role as \"role: Role\"
              , u.name
              , u.mail
              , a.linkedin
              , a.study
              , a.degree
              , a.institution
              , a.graduation_year
              , s.created_at
           from attendees as a
              , scans as s
              , users as u
          where s.initiator_id = $1
            and s.subject_id = u.id
            and u.id = a.user_id
            and s.is_expunged = 'false'
       order by s.created_at desc",
        subject,
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?
    .into_iter()
    .map(|res| {
        let attendee = Attendee::new(
            res.subject_id,
            res.linkedin,
            res.study,
            res.degree,
            res.institution,
            res.graduation_year,
        );

        let user = User::new(
            res.subject_id,
            res.role,
            res.name,
            res.mail,
            Some(attendee),
            None,
        );

        Scan::new(
            res.id,
            res.subject_id,
            Some(user),
            subject,
            Some(initiator.clone()),
            false,
            res.created_at,
        )
    })
    .collect();

    let passive = sqlx::query!(
        "select s.id
              , s.initiator_id
              , u.role as \"role: Role\"
              , u.name
              , u.mail
              , a.linkedin
              , a.study
              , a.degree
              , a.institution
              , a.graduation_year
              , s.created_at
           from attendees as a
              , scans as s
              , users as u
          where s.subject_id = $1
            and s.initiator_id = u.id
            and u.id = a.user_id
            and s.is_expunged = 'false'
       order by s.created_at desc",
        subject,
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?
    .into_iter()
    .map(|res| {
        let attendee = Attendee::new(
            res.initiator_id,
            res.linkedin,
            res.study,
            res.degree,
            res.institution,
            res.graduation_year,
        );

        let user = User::new(
            res.initiator_id,
            res.role,
            res.name,
            res.mail,
            Some(attendee),
            None,
        );

        Scan::new(
            res.id,
            res.initiator_id,
            Some(initiator.clone()),
            subject,
            Some(user),
            false,
            res.created_at,
        )
    })
    .collect();

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
        "select u.id, u.role as \"role: Role\"
           from badges as b
              , users as u
          where b.badge = $1
            and b.user_id = u.id",
        badge,
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::DuplicateScan)?
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

    let mut scan = sqlx::query!(
        "insert into scans ( initiator_id, subject_id )
         values ( $1, $2 )
         returning id, created_at",
        initiator_id,
        subject_id,
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::DuplicateScan)?
    .map(|res| Scan::new(res.id, subject_id, None, initiator_id, None, false, res.created_at))
    .ok_or_else(|| Error::UnknownScan)?;

    let attendee = sqlx::query!(
        "select u.id
              , u.name
              , u.mail
              , a.linkedin
              , a.study
              , a.degree
              , a.institution
              , a.graduation_year
           from attendees as a
              , users as u
          where u.id = $1
            and u.id = a.user_id",
        if initiator_role == Role::Exhibitor {
            subject_id
        } else {
            initiator_id
        },
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?
    .map(|res| {
        let attendee = Attendee::new(
            res.id,
            res.linkedin,
            res.study,
            res.degree,
            res.institution,
            res.graduation_year,
        );

        User::new(
            res.id,
            Role::Attendee,
            res.name,
            res.mail,
            Some(attendee),
            None,
        )
    })
    .ok_or_else(|| Error::UnknownUser)?;

    let exhibitor = sqlx::query!(
        "select u.id
              , u.name
              , u.mail
              , e.company
           from exhibitors as e
              , users as u
          where u.id = $1
            and u.id = e.user_id",
        if initiator_role == Role::Attendee {
            subject_id
        } else {
            initiator_id
        },
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| Error::Internal)?
    .map(|res| {
        let exhibitor = Exhibitor::new(res.id, res.company);

        User::new(
            res.id,
            Role::Exhibitor,
            res.name,
            res.mail,
            None,
            Some(exhibitor),
        )
    })
    .ok_or_else(|| Error::UnknownUser)?;

    transaction.commit().await.map_err(|_| Error::Internal)?;

    match &initiator_role {
        Role::Attendee => {
            scan.initiator = Some(attendee);
            scan.subject = Some(exhibitor);
        }
        Role::Exhibitor => {
            scan.initiator = Some(exhibitor);
            scan.subject = Some(attendee);
        }
        _ => unreachable!(),
    }

    Ok(Json(scan))
}
