mod domain;
mod external;
mod database;
mod app;
use log4rs;
use log4rs::config::Deserializers;

#[macro_use]
extern crate rusqlite;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate error_chain;

#[actix_rt::main]
async fn main() -> domain::result::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Deserializers::default()).unwrap();
    let rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(1)
        .max_threads(4)
        .thread_name("app:worker:")
        .build().unwrap();
    info!("Starting app");
    app::run(rt.handle()).await
        .map_err(|e| -> domain::result::Error {
            error!("Terminal error causing shutdown: {}", domain::result::fmt_backtrace(&e));
            std::process::exit(-1);
        })
}

