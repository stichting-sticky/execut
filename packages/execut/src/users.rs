mod populate;
mod scans;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

pub use populate::populate;
pub use scans::{get_scans, scan_badge};

#[derive(Clone, Debug, Deserialize, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct Badge(pub Uuid);

#[derive(Clone, Debug, Deserialize, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct Token(pub String);

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase", type_name = "role")]
pub enum Role {
    Admin,
    Exhibitor,
    Attendee,
}

#[derive(Deserialize)]
pub struct User {
    pub id: Uuid,
    pub role: Role,
    pub name: String,
    pub mail: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
}
