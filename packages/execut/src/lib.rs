mod auth;
mod errors;
mod users;

pub mod handlers;

use sqlx::{Pool, Postgres};

pub use crate::{
    auth::Keys,
    errors::{Error, Result},
};

#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Postgres>,
    pub keys: Keys,
}
