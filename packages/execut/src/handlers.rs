use crate::Result;

pub async fn health_check() -> Result<()> {
    Ok(())
}

pub use crate::{
    auth::authorize,
    users::{get_scans, populate, scan_badge},
};
