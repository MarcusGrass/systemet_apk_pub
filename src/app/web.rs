use std::io;

use actix_web::http::{Method, StatusCode};
use actix_utils::mpsc;
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::Query;
use bytes::Bytes;
use crate::domain::models::product::{ProductOpts};
use crate::domain::result::{Result, fmt_backtrace};
use crate::app::service;
use actix_cors::Cors;
use serde::Serialize;


async fn get_top(product_opts: Query<ProductOpts>) -> HttpResponse {
    ok_or_err(service::fetch_products(product_opts.0).await)
}

async fn get_site_names() -> HttpResponse {
    ok_or_err(service::fetch_site_names().await)
}

fn ok_or_err<T: Sized + Serialize>(res: Result<T>) -> HttpResponse {
    res.map(|t| -> HttpResponse {
        to_ok(&t)
    }).unwrap_or_else(|e| -> HttpResponse {
        error!("Caught error responding to request: {}", fmt_backtrace(&e));
        HttpResponse::InternalServerError().finish()
    })
}

fn to_ok<T: Sized + Serialize>(val: &T) -> HttpResponse {
    let text = serde_json::to_string(val)
        .expect("Failed to serialize value");
    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

pub async fn start() -> io::Result<()> {
    let bind = "127.0.0.1:8080";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            .wrap(Cors::new()
                .allowed_origin("*")
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .finish()
            )
            .service(
                web::resource("/top").
                    route(web::get().to(get_top))
            ).service(
            web::resource("/site_names").
                route(web::get().to(get_site_names))
            )
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
        .bind(bind)?
        .run()
        .await
}

async fn p404() -> HttpResponse {
    HttpResponse::NotFound().finish()
}