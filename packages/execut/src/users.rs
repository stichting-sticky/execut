mod populate;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

pub use populate::populate;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Badge(pub Uuid);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Token(pub String);

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Type)]
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
