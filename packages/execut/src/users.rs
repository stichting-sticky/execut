mod attendee;
mod exhibitor;
mod scans;

use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

pub use attendee::{seed as seed_attendees, Attendee};
pub use exhibitor::{seed as seed_exhibitors, Exhibitor};
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

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub role: Role,
    pub name: String,
    pub mail: String,
    #[serde(flatten)]
    pub attendee: Option<Attendee>,
    #[serde(flatten)]
    pub exhibitor: Option<Exhibitor>,
}

impl User {
    pub fn new(
        id: Uuid,
        role: Role,
        name: String,
        mail: String,
        attendee: Option<Attendee>,
        exhibitor: Option<Exhibitor>,
    ) -> Self {
        Self {
            id,
            role,
            name,
            mail,
            attendee,
            exhibitor,
        }
    }
}
