mod service;
mod web;
use crate::domain::result::Result;

use tokio::time::{Instant, Duration};
use tokio::runtime::Handle;
use crate::domain::result;

pub async fn run(handle: &Handle) -> Result<()> {
    service::init_db().await?;


    handle.spawn(async move {
        let mut interval = tokio::time::interval_at(Instant::now(),
                                                    Duration::from_secs(60*60*3));
        loop {
            interval.tick().await;
            info!("Starting db refresh.");
            let res = service::update_db().await;
            if res.is_err() {
                error!("Error updating db: {}", result::fmt_backtrace(&res.unwrap_err()));
            }
            info!("Finished db refresh.");
        }
    });
    web::start().await
        .map_err(|e| e.into())
}
